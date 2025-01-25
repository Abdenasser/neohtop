#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//! NeoHtop - A modern system monitor built with Tauri
//!
//! This is the main entry point for the application. It sets up the Tauri
//! application, initializes plugins, and configures window effects.

mod commands;
mod monitoring;
mod state;
mod ui;

use state::AppState;
use tauri::Manager;

/// Main entry point for the application
///
/// # Panics
///
/// Will panic if:
/// - Unable to create the main window
/// - Failed to apply window effects
/// - Failed to initialize the application state
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            ui::setup_window_effects(&window).expect("Failed to apply window effects");
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_processes,
            commands::kill_process,
            commands::open_process_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
