#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use sysinfo::{
    CpuExt, Disk, DiskExt, NetworkExt, NetworksExt, PidExt, ProcessExt, ProcessStatus, System,
    SystemExt,
};
use tauri::State;

struct AppState {
    sys: Mutex<System>,
    process_cache: Mutex<HashMap<u32, ProcessStaticInfo>>,
    last_network_update: Mutex<(Instant, u64, u64)>,
}

impl AppState {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_all();

        // Initialize network stats
        let initial_rx = sys
            .networks()
            .iter()
            .map(|(_, data)| data.total_received())
            .sum();
        let initial_tx = sys
            .networks()
            .iter()
            .map(|(_, data)| data.total_transmitted())
            .sum();

        Self {
            sys: Mutex::new(sys),
            process_cache: Mutex::new(HashMap::new()),
            last_network_update: Mutex::new((Instant::now(), initial_rx, initial_tx)),
        }
    }
}

#[derive(Clone)]
struct ProcessStaticInfo {
    name: String,
    command: String,
    user: String,
}

#[derive(serde::Serialize)]
struct ProcessInfo {
    pid: u32,
    ppid: u32,
    name: String,
    cpu_usage: f32,
    memory_usage: u64,
    status: String,
    user: String,
    command: String,
    threads: Option<u32>,
    environ: Vec<String>,
    root: String,
    virtual_memory: u64,
    start_time: u64,
    run_time: u64,
    disk_usage: (u64, u64), // (read_bytes, written_bytes)
    session_id: Option<u32>,
}

#[derive(serde::Serialize)]
pub struct SystemStats {
    pub cpu_usage: Vec<f32>,
    pub memory_total: u64,
    pub memory_used: u64,
    pub memory_free: u64,
    pub memory_cached: u64,
    pub uptime: u64,
    pub load_avg: [f64; 3],
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
    pub disk_total_bytes: u64,
    pub disk_used_bytes: u64,
    pub disk_free_bytes: u64,
}

// Assume MacOS or Linux
#[cfg(not(target_os = "windows"))]
fn filter_disks(disks: &[Disk]) -> Vec<&sysinfo::Disk> {
    disks
        .iter()
        .filter(|disk| {
            // Filter for physical disks - typically those mounted at "/"
            disk.mount_point() == std::path::Path::new("/")
        })
        .collect()
}

#[cfg(target_os = "windows")]
fn filter_disks(disks: &[Disk]) -> Vec<&sysinfo::Disk> {
    disks.iter().collect()
}

#[tauri::command]
async fn get_processes(
    state: State<'_, AppState>,
) -> Result<(Vec<ProcessInfo>, SystemStats), String> {
    let processes_data;
    let system_stats;

    // Get current time once for all calculations
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    // Scope for system lock
    {
        let mut sys = state
            .sys
            .lock()
            .map_err(|_| "Failed to lock system state")?;
        sys.refresh_all();
        sys.refresh_networks_list();
        sys.refresh_disks_list();

        // Collect all the process data we need while holding sys lock
        processes_data = sys
            .processes()
            .iter()
            .map(|(pid, process)| {
                let start_time = process.start_time();
                let run_time = if start_time > 0 {
                    current_time.saturating_sub(start_time)
                } else {
                    0
                };

                (
                    pid.as_u32(),
                    process.name().to_string(),
                    process.cmd().to_vec(),
                    process.user_id().map(|uid| uid.to_string()),
                    process.cpu_usage(),
                    process.memory(),
                    process.status(),
                    process.parent().map(|p| p.as_u32()),
                    process.environ().to_vec(),
                    process.root().to_string_lossy().into_owned(),
                    process.virtual_memory(),
                    start_time,
                    run_time, // Use calculated run_time
                    process.disk_usage().read_bytes,
                    process.disk_usage().written_bytes,
                    process.session_id().map(|id| id.as_u32()),
                )
            })
            .collect::<Vec<_>>();

        // Calculate total network I/O
        let mut last_update = state
            .last_network_update
            .lock()
            .map_err(|_| "Failed to lock network state")?;
        let elapsed = last_update.0.elapsed().as_secs_f64();
        let current_time = Instant::now();

        let current_rx: u64 = sys
            .networks()
            .iter()
            .map(|(_, data)| data.total_received())
            .sum();
        let current_tx: u64 = sys
            .networks()
            .iter()
            .map(|(_, data)| data.total_transmitted())
            .sum();

        let network_stats = (
            ((current_rx - last_update.1) as f64 / elapsed) as u64,
            ((current_tx - last_update.2) as f64 / elapsed) as u64,
        );

        *last_update = (current_time, current_rx, current_tx);

        // Calculate total disk usage
        let disk_stats = filter_disks(&sys.disks())
            .iter()
            .fold((0, 0, 0), |acc, disk| {
                (
                    acc.0 + disk.total_space(),
                    acc.1 + disk.total_space() - disk.available_space(),
                    acc.2 + disk.available_space(),
                )
            });

        system_stats = SystemStats {
            cpu_usage: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
            memory_total: sys.total_memory(),
            memory_used: sys.used_memory(),
            memory_free: sys.total_memory() - sys.used_memory(),
            memory_cached: sys.total_memory()
                - (sys.used_memory() + (sys.total_memory() - sys.used_memory())),
            uptime: sys.uptime(),
            load_avg: [
                sys.load_average().one,
                sys.load_average().five,
                sys.load_average().fifteen,
            ],
            network_rx_bytes: network_stats.0,
            network_tx_bytes: network_stats.1,
            disk_total_bytes: disk_stats.0,
            disk_used_bytes: disk_stats.1,
            disk_free_bytes: disk_stats.2,
        };
    } // sys lock is automatically dropped here

    // Now lock the process cache
    let mut process_cache = state
        .process_cache
        .lock()
        .map_err(|_| "Failed to lock process cache")?;

    // Build the process info list
    let processes = processes_data
        .into_iter()
        .map(
            |(
                pid,
                name,
                cmd,
                user_id,
                cpu_usage,
                memory,
                status,
                ppid,
                environ,
                root,
                virtual_memory,
                start_time,
                run_time,
                disk_read,
                disk_written,
                session_id,
            )| {
                let static_info = process_cache
                    .entry(pid)
                    .or_insert_with(|| ProcessStaticInfo {
                        name: name.clone(),
                        command: cmd.join(" "),
                        user: user_id.unwrap_or_else(|| "-".to_string()),
                    });

                let status_str = match status {
                    ProcessStatus::Run => "Running",
                    ProcessStatus::Sleep => "Sleeping",
                    ProcessStatus::Idle => "Idle",
                    _ => "Unknown",
                };

                ProcessInfo {
                    pid,
                    ppid: ppid.unwrap_or(0),
                    name: static_info.name.clone(),
                    cpu_usage,
                    memory_usage: memory,
                    status: status_str.to_string(),
                    user: static_info.user.clone(),
                    command: static_info.command.clone(),
                    threads: None,
                    environ,
                    root,
                    virtual_memory,
                    start_time,
                    run_time,
                    disk_usage: (disk_read, disk_written),
                    session_id,
                }
            },
        )
        .collect();

    Ok((processes, system_stats))
}

#[tauri::command]
async fn kill_process(pid: u32, state: State<'_, AppState>) -> Result<bool, String> {
    let sys = state
        .sys
        .lock()
        .map_err(|_| "Failed to lock system state")?;
    if let Some(process) = sys.process(sysinfo::Pid::from(pid as usize)) {
        Ok(process.kill())
    } else {
        Ok(false)
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![get_processes, kill_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
