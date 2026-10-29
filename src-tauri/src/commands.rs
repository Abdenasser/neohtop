//! Tauri command handlers
//!
//! This module contains the command handlers that are exposed to the frontend
//! through Tauri's IPC mechanism. These commands provide the interface between
//! the frontend and the system monitoring functionality.

use crate::monitoring::ProcessMonitor;
use crate::state::AppState;
use tauri::State;

/// The main window and companion share one interval-gated native capture.
#[tauri::command]
pub async fn get_processes(
    app: tauri::AppHandle,
    include_gpu: Option<bool>,
) -> Result<crate::sampling::Snapshot, String> {
    tauri::async_runtime::spawn_blocking(move || {
        use tauri::Manager;
        let state = app.state::<AppState>();
        let mut hub = state.hub.lock().map_err(|e| e.to_string())?;
        hub.process_gpu = include_gpu.unwrap_or(false);
        hub.read(&state)
    })
    .await
    .map_err(|e| e.to_string())?
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

/// Resolve only requested, currently observed process identities. File paths never
/// come from the frontend; release the sampling lock before doing icon I/O.
#[tauri::command]
pub async fn get_process_icons(
    requests: Vec<crate::native_icons::IconRequest>,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<Vec<crate::native_icons::IconReply>, String> {
    use crate::native_icons::{IconReply, MAX_BATCH};
    if requests.len() > MAX_BATCH {
        return Err("Too many icon requests".into());
    }
    let targets = {
        let sys = state.sys.lock().map_err(|e| e.to_string())?;
        requests
            .into_iter()
            .map(|request| {
                let path = crate::native_icons::process_target(&sys, &request);
                (request, path)
            })
            .collect::<Vec<_>>()
    };
    tauri::async_runtime::spawn_blocking(move || {
        use tauri::Manager;
        let state = app.state::<AppState>();
        let mut icons = state.icons.lock().map_err(|e| e.to_string())?;
        Ok(targets
            .into_iter()
            .map(|(request, path)| IconReply {
                pid: request.pid,
                start_time: request.start_time,
                icon: path.and_then(|p| icons.resolve(&p)),
            })
            .collect())
    })
    .await
    .map_err(|e| e.to_string())?
}
