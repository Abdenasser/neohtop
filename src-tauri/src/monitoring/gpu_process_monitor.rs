//! Per-process GPU usage monitoring
//!
//! Linux DRM drivers publish per-client engine busy times through
//! `/proc/<pid>/fdinfo/<fd>`, for every file descriptor pointing at a render
//! node. The counters are cumulative nanoseconds, so a usage percentage comes
//! from how much they grow between two samples, divided by the wall time that
//! passed in between.
//!
//! Only descriptors the current user is allowed to inspect can be read, so
//! processes belonging to other users are reported as idle rather than
//! unknown. Platforms without DRM report no usage at all, which the frontend
//! shows as a dash.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::time::Instant;

/// Where the kernel exposes its per-process information
const PROC_PATH: &str = "/proc";

/// Directory holding the DRM device nodes
const DRM_DEVICE_PATH: &str = "/dev/dri";

/// Upper bound on threads used to walk `/proc`, so that a machine with many
/// cores does not spawn far more workers than the scan can keep busy
const MAX_SCAN_THREADS: usize = 8;

/// Tracks how much GPU engine time each process is using
#[derive(Debug)]
pub struct GpuProcessMonitor {
    /// Whether this machine exposes DRM devices at all
    supported: bool,
    /// Cumulative engine nanoseconds per process at the previous sample
    previous: HashMap<u32, u64>,
    /// When that sample was taken
    sampled_at: Instant,
}

impl GpuProcessMonitor {
    /// Creates a monitor, noting whether per-process accounting is possible
    pub fn new() -> Self {
        Self {
            supported: Path::new(DRM_DEVICE_PATH).is_dir(),
            previous: HashMap::new(),
            sampled_at: Instant::now(),
        }
    }

    /// Whether this machine can account GPU time per process
    pub fn is_supported(&self) -> bool {
        self.supported
    }

    /// Samples every process and returns its GPU usage as a percentage
    ///
    /// The first call has no earlier sample to compare against and so reports
    /// nothing; usage appears from the second refresh onwards. A process can
    /// exceed 100% by keeping several engines busy at once, in the same way
    /// CPU usage exceeds 100% across cores.
    pub fn collect(&mut self) -> HashMap<u32, f32> {
        if !self.supported {
            return HashMap::new();
        }

        let current = collect_engine_times();
        let now = Instant::now();
        let elapsed = now.duration_since(self.sampled_at).as_nanos();

        // Two samples taken within the same instant carry no information
        let usage = if elapsed == 0 {
            HashMap::new()
        } else {
            current
                .iter()
                .filter_map(|(pid, total)| {
                    let previous = self.previous.get(pid)?;
                    // The counters only ever climb, but a recycled pid can
                    // look like a decrease, so those are dropped instead of
                    // being reported as negative usage
                    let delta = total.checked_sub(*previous)?;
                    let percentage = (delta as f64 / elapsed as f64) * 100.0;
                    Some((*pid, percentage as f32))
                })
                .collect()
        };

        self.previous = current;
        self.sampled_at = now;

        usage
    }
}

/// Sums the GPU engine time every readable process has accumulated
fn collect_engine_times() -> HashMap<u32, u64> {
    let Ok(entries) = fs::read_dir(PROC_PATH) else {
        return HashMap::new();
    };

    // Reading `/proc` lists thread group leaders only, so GPU time is
    // attributed to the process rather than counted once per thread
    let pids: Vec<u32> = entries
        .flatten()
        .filter_map(|entry| entry.file_name().to_str()?.parse().ok())
        .collect();

    // Inspecting every descriptor of every process costs one system call per
    // descriptor, which runs into the tens of thousands on a busy desktop.
    // The work is bound by those calls rather than by computation, so it is
    // spread across a few threads to keep each refresh short.
    let threads = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1)
        .min(MAX_SCAN_THREADS);
    let chunk_size = pids.len().div_ceil(threads).max(1);

    std::thread::scope(|scope| {
        let handles: Vec<_> = pids
            .chunks(chunk_size)
            .map(|chunk| {
                scope.spawn(move || {
                    chunk
                        .iter()
                        .filter_map(|&pid| Some((pid, read_process_engine_time(pid)?)))
                        .collect::<Vec<_>>()
                })
            })
            .collect();

        handles
            .into_iter()
            .flat_map(|handle| handle.join().unwrap_or_default())
            .collect()
    })
}

/// Sums the engine time across one process's DRM file descriptors
///
/// Returns `None` for processes that hold no DRM descriptor, or whose
/// descriptors cannot be inspected because they belong to another user.
fn read_process_engine_time(pid: u32) -> Option<u64> {
    let entries = fs::read_dir(format!("{PROC_PATH}/{pid}/fd")).ok()?;

    let mut total = 0;
    let mut is_drm_client = false;
    let mut seen_clients = HashSet::new();

    for entry in entries.flatten() {
        // Checking where the descriptor points is far cheaper than reading
        // every descriptor's fdinfo, and most of them are not GPU related
        let Ok(target) = fs::read_link(entry.path()) else {
            continue;
        };
        if !target.starts_with(DRM_DEVICE_PATH) {
            continue;
        }

        let Some(fd) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        let Ok(info) = fs::read_to_string(format!("{PROC_PATH}/{pid}/fdinfo/{fd}")) else {
            continue;
        };

        if let Some((client, engine_time)) = parse_fdinfo(&info) {
            is_drm_client = true;
            // Duplicated descriptors describe the same client and repeat its
            // counters, so each client is only counted once
            if seen_clients.insert(client) {
                total += engine_time;
            }
        }
    }

    is_drm_client.then_some(total)
}

/// Extracts a descriptor's DRM client id and its total engine time
///
/// Returns `None` for descriptors that are not DRM clients.
fn parse_fdinfo(info: &str) -> Option<(u64, u64)> {
    let mut client_id = None;
    let mut engine_time = 0;

    for line in info.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let value = value.trim();

        if key == "drm-client-id" {
            client_id = value.parse().ok();
        } else if key.starts_with("drm-engine-") && !key.starts_with("drm-engine-capacity-") {
            // Engine lines carry a unit, as in "1693357467 ns"
            if let Some(nanoseconds) = value
                .split_whitespace()
                .next()
                .and_then(|number| number.parse::<u64>().ok())
            {
                engine_time += nanoseconds;
            }
        }
    }

    client_id.map(|id| (id, engine_time))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A single fdinfo block is reduced to its client id and total engine time
    #[test]
    fn test_parse_fdinfo() {
        let info = "pos:\t0\n\
                    drm-driver:\tamdgpu\n\
                    drm-client-id:\t495\n\
                    drm-memory-vram:\t23240 KiB\n\
                    drm-engine-gfx:\t1000000 ns\n\
                    drm-engine-compute:\t500000 ns\n";

        assert_eq!(parse_fdinfo(info), Some((495, 1_500_000)));
    }

    /// Capacity lines describe how many engines exist, not how busy they were
    #[test]
    fn test_parse_fdinfo_ignores_capacity() {
        let info = "drm-client-id:\t7\n\
                    drm-engine-capacity-gfx:\t2\n\
                    drm-engine-gfx:\t400 ns\n";

        assert_eq!(parse_fdinfo(info), Some((7, 400)));
    }

    /// Descriptors that are not DRM clients carry no client id
    #[test]
    fn test_parse_fdinfo_rejects_non_drm() {
        assert_eq!(parse_fdinfo("pos:\t0\nflags:\t02\n"), None);
    }

    /// The first sample only establishes a baseline, so it reports nothing
    #[test]
    fn test_first_sample_has_no_baseline() {
        let mut monitor = GpuProcessMonitor::new();
        assert!(monitor.collect().is_empty());
    }

    /// Percentages must stay non-negative and finite
    #[test]
    fn test_collected_usage_is_plausible() {
        let mut monitor = GpuProcessMonitor::new();
        monitor.collect();

        for (_, usage) in monitor.collect() {
            assert!(usage.is_finite());
            assert!(usage >= 0.0);
        }
    }
}
