//! Both windows read one interval-gated capture and one bounded in-memory history.
use crate::{
    gpu::GpuDevice,
    monitoring::{ProcessInfo, SystemStats},
    state::AppState,
};
use serde::Serialize;
use std::{
    collections::{BTreeMap, VecDeque},
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
pub fn interval(ms: u64) -> u64 {
    if ms == 1000 {
        1500
    } else {
        ms.clamp(1500, 30000)
    }
}

#[derive(Debug, Serialize)]
pub struct Frame {
    pub captured_at: u64,
    pub sequence: u64,
    pub processes: Vec<ProcessInfo>,
    pub stats: SystemStats,
    pub gpus: Vec<GpuDevice>,
}
#[derive(Clone, Debug, Serialize)]
pub struct Sample {
    pub timestamp: u64,
    pub interval: u64,
    pub cpu: Option<f64>,
    pub memory: Option<f64>,
    pub received: Option<u64>,
    pub sent: Option<u64>,
    pub gpus: BTreeMap<String, Option<f64>>,
}
impl Sample {
    fn gap(timestamp: u64, interval: u64) -> Self {
        Self {
            timestamp,
            interval,
            cpu: None,
            memory: None,
            received: None,
            sent: None,
            gpus: BTreeMap::new(),
        }
    }
    fn from_frame(frame: &Frame, interval: u64) -> Self {
        let stats = &frame.stats;
        let cpu = if stats.cpu_usage.is_empty() || stats.cpu_usage.iter().any(|v| !v.is_finite()) {
            None
        } else {
            Some(
                (stats.cpu_usage.iter().map(|v| *v as f64).sum::<f64>()
                    / stats.cpu_usage.len() as f64)
                    .clamp(0.0, 100.0),
            )
        };
        Self {
            cpu,
            memory: (stats.memory_total > 0).then(|| {
                (stats.memory_used as f64 / stats.memory_total as f64 * 100.0).clamp(0.0, 100.0)
            }),
            received: stats.network_rx_bytes,
            sent: stats.network_tx_bytes,
            gpus: frame
                .gpus
                .iter()
                .map(|g| (g.id.clone(), g.utilization))
                .collect(),
            ..Self::gap(frame.captured_at, interval)
        }
    }
}
fn append(history: &mut VecDeque<Sample>, sample: Sample) {
    let timestamp = sample.timestamp;
    if history.back().is_some_and(|s| s.timestamp > timestamp) {
        history.clear();
    }
    if let Some(last) = history.back() {
        if timestamp.saturating_sub(last.timestamp) > last.interval.max(sample.interval) * 5 / 2 {
            history.push_back(Sample::gap(
                last.timestamp + (timestamp - last.timestamp) / 2,
                sample.interval,
            ));
        }
    }
    if history.back().is_some_and(|s| s.timestamp == timestamp) {
        history.pop_back();
    }
    history.push_back(sample);
    while history.len() > 160
        || history
            .front()
            .is_some_and(|s| s.timestamp < timestamp.saturating_sub(120000))
    {
        history.pop_front();
    }
}

#[derive(Debug)]
pub struct Hub {
    pub frame: Option<Arc<Frame>>,
    pub process_gpu: bool,
    gpu_process_monitor: crate::monitoring::GpuProcessMonitor,
    history: VecDeque<Sample>,
    pub interval: u64,
    pub paused: bool,
    last_attempt: Option<Instant>,
    sequence: u64,
    error: Option<String>,
}
impl Default for Hub {
    fn default() -> Self {
        Self {
            frame: None,
            process_gpu: false,
            gpu_process_monitor: crate::monitoring::GpuProcessMonitor::new(),
            history: VecDeque::new(),
            interval: 3000,
            paused: false,
            last_attempt: None,
            sequence: 0,
            error: None,
        }
    }
}
#[derive(Serialize)]
pub struct Snapshot {
    pub frame: Arc<Frame>,
    pub history: Vec<Sample>,
    pub paused: bool,
    pub interval: u64,
    pub error: Option<String>,
}
impl Hub {
    fn due(&self) -> bool {
        self.frame.is_none()
            || (!self.paused
                && self
                    .last_attempt
                    .is_none_or(|i| i.elapsed() >= Duration::from_millis(self.interval)))
    }
    pub fn pause(&mut self, paused: bool) {
        if self.paused == paused {
            return;
        }
        self.paused = paused;
        append(&mut self.history, Sample::gap(now_ms(), self.interval));
        // Preserve the CPU minimum refresh window when resuming immediately.
        if !paused
            && self
                .last_attempt
                .is_some_and(|i| i.elapsed() >= Duration::from_millis(1500))
        {
            self.last_attempt = None;
        }
    }
    pub fn read(&mut self, state: &AppState) -> Result<Snapshot, String> {
        if self.due() {
            self.last_attempt = Some(Instant::now());
            match self.capture(state) {
                Ok(frame) => {
                    append(&mut self.history, Sample::from_frame(&frame, self.interval));
                    self.frame = Some(Arc::new(frame));
                    self.error = None;
                }
                Err(error) => {
                    append(&mut self.history, Sample::gap(now_ms(), self.interval));
                    self.error = Some(error);
                }
            }
        }
        Ok(Snapshot {
            frame: self.frame.clone().ok_or_else(|| {
                self.error
                    .clone()
                    .unwrap_or_else(|| "No system readings yet".into())
            })?,
            history: self.history.iter().cloned().collect(),
            paused: self.paused,
            interval: self.interval,
            error: self.error.clone(),
        })
    }
    fn capture(&mut self, state: &AppState) -> Result<Frame, String> {
        let (mut processes, stats) = {
            let mut sys = state.sys.lock().map_err(|e| e.to_string())?;
            let mut disks = state.disks.lock().map_err(|e| e.to_string())?;
            let mut networks = state.networks.lock().map_err(|e| e.to_string())?;
            sys.refresh_all();
            disks.refresh(true);
            networks.refresh(true);
            let processes = state
                .process_monitor
                .lock()
                .map_err(|e| e.to_string())?
                .collect_processes(&sys)?;
            let stats = state
                .system_monitor
                .lock()
                .map_err(|e| e.to_string())?
                .collect_stats(&sys, &networks, &disks);
            (processes, stats)
        };
        if self.process_gpu {
            let usage = self.gpu_process_monitor.collect(&processes);
            for process in &mut processes {
                process.gpu_usage = usage.get(&(process.pid, process.start_time)).copied();
            }
        } else {
            self.gpu_process_monitor.reset();
        }
        let gpus = state.gpu.lock().map_err(|e| e.to_string())?.collect();
        self.sequence += 1;
        Ok(Frame {
            captured_at: now_ms(),
            sequence: self.sequence,
            processes,
            stats,
            gpus,
        })
    }
}

#[derive(Serialize)]
pub struct BusyProcess {
    pub pid: u32,
    pub start_time: u64,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
}
#[derive(Serialize)]
pub struct Overview {
    pub captured_at: u64,
    pub stats: SystemStats,
    pub gpus: Vec<GpuDevice>,
    pub history: Vec<Sample>,
    pub processes: Vec<BusyProcess>,
    pub paused: bool,
    pub interval: u64,
    pub error: Option<String>,
}
impl From<Snapshot> for Overview {
    fn from(s: Snapshot) -> Self {
        let mut processes = s.frame.processes.iter().collect::<Vec<_>>();
        processes.sort_by(|a, b| {
            b.cpu_usage
                .total_cmp(&a.cpu_usage)
                .then_with(|| b.memory_usage.cmp(&a.memory_usage))
                .then_with(|| a.pid.cmp(&b.pid))
        });
        Self {
            captured_at: s.frame.captured_at,
            stats: s.frame.stats.clone(),
            gpus: s.frame.gpus.clone(),
            history: s.history,
            processes: processes
                .into_iter()
                .take(5)
                .map(|p| BusyProcess {
                    pid: p.pid,
                    start_time: p.start_time,
                    name: p.name.clone(),
                    cpu_usage: p.cpu_usage,
                    memory_usage: p.memory_usage,
                })
                .collect(),
            paused: s.paused,
            interval: s.interval,
            error: s.error,
        }
    }
}

#[tauri::command]
pub async fn get_overview(app: tauri::AppHandle) -> Result<Overview, String> {
    read(app).await.map(Into::into)
}
pub async fn read(app: tauri::AppHandle) -> Result<Snapshot, String> {
    tauri::async_runtime::spawn_blocking(move || {
        use tauri::Manager;
        let state = app.state::<AppState>();
        let result = state.hub.lock().map_err(|e| e.to_string())?.read(&state);
        result
    })
    .await
    .map_err(|e| e.to_string())?
}
#[tauri::command]
pub async fn set_monitor_paused(
    paused: bool,
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    use tauri::Emitter;
    state.hub.lock().map_err(|e| e.to_string())?.pause(paused);
    app.emit("monitor-control", ()).map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn set_monitor_interval(
    milliseconds: u64,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    state.hub.lock().map_err(|e| e.to_string())?.interval = interval(milliseconds);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded_history_clock_changes_and_missing_reads() {
        let mut h = VecDeque::new();
        append(&mut h, Sample::gap(10000, 3000));
        append(&mut h, Sample::gap(40000, 3000));
        assert_eq!(
            h.iter().map(|s| s.timestamp).collect::<Vec<_>>(),
            vec![10000, 25000, 40000]
        );
        for i in 0..10000 {
            append(&mut h, Sample::gap(50000 + i, 3000));
        }
        assert_eq!(h.len(), 160);
        append(&mut h, Sample::gap(500000, 3000));
        assert!(h.iter().all(|s| s.timestamp >= 380000));
        append(&mut h, Sample::gap(100, 3000));
        assert_eq!(h.len(), 1);
        append(&mut h, Sample::gap(100, 3000));
        assert_eq!(h.len(), 1);
    }
    #[test]
    fn shared_cache_pause_and_cadence() {
        let mut hub = Hub::default();
        assert!(hub.due());
        assert_eq!(interval(1000), 1500);
        assert_eq!(interval(300000), 30000);
        hub.pause(true);
        hub.pause(true);
        assert_eq!(hub.history.len(), 1);
        hub.pause(false);
        assert!(!hub.paused);
        // Capture readiness is interval-gated using monotonic time, not wall-clock.
        hub.last_attempt = Some(Instant::now());
        // A frame is required before the cache may suppress the first capture.
        assert!(hub.due());
    }
    #[test]
    fn changing_interval_does_not_create_a_false_gap() {
        let mut h = VecDeque::new();
        append(&mut h, Sample::gap(10000, 3000));
        append(&mut h, Sample::gap(40000, 30000));
        assert_eq!(h.len(), 2);
    }
    #[test]
    fn both_readers_share_one_capture_and_pause() {
        let state = AppState::new();
        let mut hub = Hub::default();
        let main = hub.read(&state).unwrap();
        let companion = hub.read(&state).unwrap();
        assert!(Arc::ptr_eq(&main.frame, &companion.frame));
        assert_eq!(main.history.len(), 1);
        assert_eq!(companion.history.len(), 1);
        hub.pause(true);
        let paused = hub.read(&state).unwrap();
        assert!(paused.paused);
        assert!(Arc::ptr_eq(&main.frame, &paused.frame));
        assert!(paused.history.last().unwrap().cpu.is_none());
        hub.last_attempt = Some(Instant::now() - Duration::from_secs(4));
        hub.pause(false);
        let resumed = hub.read(&state).unwrap();
        assert_eq!(resumed.frame.sequence, main.frame.sequence + 1);
        let overview: Overview = resumed.into();
        assert!(overview.processes.len() <= 5);
    }
    #[test]
    fn gpu_history_keeps_adapter_identity_and_real_zeros() {
        let state = AppState::new();
        let mut hub = Hub::default();
        let snapshot = hub.read(&state).unwrap();
        let device = |id: &str, utilization: Option<f64>| GpuDevice {
            id: id.into(),
            name: id.into(),
            vendor: "Fixture".into(),
            utilization,
            memory_used: None,
            memory_total: None,
            memory_kind: "unknown".into(),
            temperature: None,
            power_watts: None,
            core_clock_mhz: None,
            cores: None,
            source: "Fixture".into(),
        };
        let frame = Frame {
            captured_at: 10000,
            sequence: 1,
            processes: vec![],
            stats: snapshot.frame.stats.clone(),
            gpus: vec![
                device("adapter-a", Some(0.0)),
                device("adapter-b", Some(75.0)),
                device("adapter-c", None),
            ],
        };
        let sample = Sample::from_frame(&frame, 3000);
        let mut history = VecDeque::new();
        append(&mut history, sample);
        append(&mut history, Sample::gap(13000, 3000));
        assert_eq!(history[0].gpus["adapter-a"], Some(0.0));
        assert_eq!(history[0].gpus["adapter-b"], Some(75.0));
        assert_eq!(history[0].gpus["adapter-c"], None);
        assert!(history[1].gpus.is_empty());
    }
}
