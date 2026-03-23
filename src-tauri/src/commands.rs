//! Tauri command handlers
//!
//! This module contains the command handlers that are exposed to the frontend
//! through Tauri's IPC mechanism. These commands provide the interface between
//! the frontend and the system monitoring functionality.

use crate::monitoring::{ProcessInfo, ProcessMonitor, SystemStats};
use crate::state::AppState;
use crate::virustotal::{self, VTReport};
use tauri::State;

/// Retrieves the current list of processes and system statistics
///
/// # Arguments
///
/// * `state` - The application state containing system monitoring components
///
/// # Returns
///
/// A tuple containing:
/// * A vector of process information
/// * Current system statistics
///
/// # Errors
///
/// Returns an error string if:
/// * Failed to acquire locks on system state
/// * Failed to collect process information
#[tauri::command]
pub async fn get_processes(
    state: State<'_, AppState>,
) -> Result<(Vec<ProcessInfo>, SystemStats), String> {
    let mut sys = state.sys.lock().map_err(|e| e.to_string())?;
    let mut disks = state.disks.lock().map_err(|e| e.to_string())?;
    let mut networks = state.networks.lock().map_err(|e| e.to_string())?;
    sys.refresh_all();
    disks.refresh(true);
    networks.refresh(true);

    let mut process_monitor = state.process_monitor.lock().map_err(|e| e.to_string())?;
    let mut system_monitor = state.system_monitor.lock().map_err(|e| e.to_string())?;

    let processes = process_monitor.collect_processes(&sys)?;
    let system_stats = system_monitor.collect_stats(&sys, &networks, &disks);

    Ok((processes, system_stats))
}

/// Attempts to kill a process with the specified PID
///
/// # Arguments
///
/// * `pid` - Process ID to kill
/// * `state` - The application state
///
/// # Returns
///
/// * `true` if the process was successfully killed
/// * `false` if the process couldn't be killed or wasn't found
///
/// # Errors
///
/// Returns an error string if failed to acquire lock on system state
#[tauri::command]
pub async fn kill_process(pid: u32, state: State<'_, AppState>) -> Result<bool, String> {
    let sys = state.sys.lock().map_err(|e| e.to_string())?;
    Ok(ProcessMonitor::kill_process(&sys, pid))
}

/// Computes the SHA-256 hash of the executable for the given PID.
///
/// Reads `/proc/<pid>/exe` and returns a 64-character lowercase hex string.
///
/// # Errors
///
/// Returns an error string if the executable cannot be read
/// (e.g., permission denied for root-owned processes).
#[tauri::command]
pub async fn hash_process(pid: u32) -> Result<String, String> {
    virustotal::hash_executable(pid)
}

/// Queries the VirusTotal v3 API for a file by its SHA-256 hash.
///
/// # Arguments
///
/// * `hash`    - 64-char lowercase hex SHA-256 string
/// * `api_key` - VirusTotal API key (entered by the user in the UI)
///
/// # Errors
///
/// Returns a user-friendly error string for auth failures, rate limits, etc.
#[tauri::command]
pub async fn check_virustotal_hash(hash: String, api_key: String) -> Result<VTReport, String> {
    virustotal::check_virustotal(&hash, &api_key).await
}
