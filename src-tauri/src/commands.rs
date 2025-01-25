//! Tauri command handlers
//!
//! This module contains the command handlers that are exposed to the frontend
//! through Tauri's IPC mechanism. These commands provide the interface between
//! the frontend and the system monitoring functionality.

use crate::monitoring::{ProcessInfo, ProcessMonitor, SystemStats};
use crate::state::AppState;
use sysinfo::SystemExt;
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
    sys.refresh_all();
    sys.refresh_networks_list();
    sys.refresh_disks_list();

    let mut process_monitor = state.process_monitor.lock().map_err(|e| e.to_string())?;
    let mut system_monitor = state.system_monitor.lock().map_err(|e| e.to_string())?;

    let processes = process_monitor.collect_processes(&sys)?;
    let system_stats = system_monitor.collect_stats(&sys);

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

/// Opens the directory of the process executable in the file explorer
///
/// # Arguments
///
/// * `pid` - Process ID
/// * `state` - The application state
///
/// # Returns
///
/// * `true` if the directory was successfully opened
/// * `false` otherwise
///
/// # Errors
///
/// Returns an error string if failed to acquire lock on system state
#[tauri::command]
pub async fn open_process_directory(pid: u32, state: State<'_, AppState>) -> Result<bool, String> {
    let sys = state.sys.lock().map_err(|e| e.to_string())?;
    Ok(ProcessMonitor::open_process_directory(&sys, pid))
}
