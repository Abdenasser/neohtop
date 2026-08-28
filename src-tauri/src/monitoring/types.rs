use serde::Serialize;
use std::fmt::Debug;
use sysinfo::{DiskUsage, ProcessStatus};

/// Internal representation of process data collected from the system
/// This struct is used internally and not exposed directly to the frontend
#[derive(Clone, Debug)]
pub(crate) struct ProcessData {
    /// Process ID
    pub pid: u32,
    /// Name of the process
    pub name: String,
    /// Complete command line arguments
    pub cmd: Vec<String>,
    /// User ID that owns the process
    pub user_id: Option<String>,
    /// CPU usage as percentage (0-100)
    pub cpu_usage: f32,
    /// Physical memory usage in bytes
    pub memory: u64,
    /// Current process status
    pub status: ProcessStatus,
    /// Parent process ID
    pub ppid: Option<u32>,
    /// Environment variables
    pub environ: Vec<String>,
    /// Root directory of the process
    pub root: String,
    /// Virtual memory usage in bytes
    pub virtual_memory: u64,
    /// Process start time (Unix timestamp)
    pub start_time: u64,
    /// Process running time in seconds
    pub run_time: u64,
    /// Disk I/O statistics
    pub disk_usage: DiskUsage,
    /// Session ID of the process
    pub session_id: Option<u32>,
}

/// Static information about a process that doesn't change frequently
/// Used for caching purposes to avoid frequent updates of stable data
#[derive(Clone, Debug)]
pub struct ProcessStaticInfo {
    /// Process name
    pub name: String,
    /// Full command string
    pub command: String,
    /// Username of the process owner
    pub user: String,
}

/// Process information exposed to the frontend via Tauri
/// Contains formatted and filtered process data for UI consumption
#[derive(Serialize, Debug)]
pub struct ProcessInfo {
    /// Process ID
    pub pid: u32,
    /// Parent process ID
    pub ppid: u32,
    /// Process name
    pub name: String,
    /// CPU usage as percentage (0-100)
    pub cpu_usage: f32,
    /// Physical memory usage in bytes
    pub memory_usage: u64,
    /// Process status as string
    pub status: String,
    /// Username of the process owner
    pub user: String,
    /// Full command string
    pub command: String,
    /// Number of threads (if available)
    pub threads: Option<u32>,
    /// Environment variables
    pub environ: Vec<String>,
    /// Root directory of the process
    pub root: String,
    /// Virtual memory usage in bytes
    pub virtual_memory: u64,
    /// Process start time (Unix timestamp)
    pub start_time: u64,
    /// Process running time in seconds
    pub run_time: u64,
    /// Disk I/O statistics (read bytes, written bytes)
    pub disk_usage: (u64, u64),
    /// Session ID of the process
    pub session_id: Option<u32>,
    /// Local bound ports (TCP LISTEN + UDP). Empty when port collection is skipped.
    pub ports: Vec<u16>,
}

/// System-wide statistics exposed to the frontend
/// Provides overall system resource usage and performance metrics
#[derive(Serialize, Debug)]
pub struct SystemStats {
    /// CPU usage per core as percentage (0-100)
    pub cpu_usage: Vec<f32>,
    /// Total physical memory in bytes
    pub memory_total: u64,
    /// Used physical memory in bytes
    pub memory_used: u64,
    /// Free physical memory in bytes
    pub memory_free: u64,
    /// Cached memory in bytes
    pub memory_cached: u64,
    /// System uptime in seconds
    pub uptime: u64,
    /// Load averages for 1, 5, and 15 minutes
    pub load_avg: [f64; 3],
    /// Total bytes received over network
    pub network_rx_bytes: u64,
    /// Total bytes transmitted over network
    pub network_tx_bytes: u64,
    /// Total disk space in bytes
    pub disk_total_bytes: u64,
    /// Used disk space in bytes
    pub disk_used_bytes: u64,
    /// Free disk space in bytes
    pub disk_free_bytes: u64,
}
