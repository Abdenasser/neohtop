#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//! NeoHtop - A modern system monitor built with Tauri
//!
//! This is the main entry point for the application. It sets up the Tauri
//! application, initializes plugins, and configures window effects.

mod commands;
mod companion;
mod desktop;
mod gpu;
mod monitoring;
mod native_icons;
mod preferences;
mod sampling;
mod state;
mod telemetry;
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
    #[cfg(target_os = "linux")]
    if std::env::var("XDG_SESSION_TYPE").unwrap_or_default() == "wayland" {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            ui::setup_window_effects(&window).expect("Failed to apply window effects");
            desktop::setup(app.handle())?;
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .manage(AppState::new())
        .manage(preferences::Preferences::default())
        .manage(telemetry::Telemetry::default())
        .manage(desktop::DesktopState::default())
        .manage(std::sync::Mutex::new(companion::CompanionState::default()))
        .on_window_event(companion::window_event)
        .invoke_handler(tauri::generate_handler![
            telemetry::track_usage,
            telemetry::analytics_configured,
            preferences::read_preferences,
            preferences::update_preferences,
            desktop::open_settings,
            desktop::settings_pane,
            desktop::open_about,
            commands::get_processes,
            commands::get_process_icons,
            commands::kill_process,
            sampling::get_overview,
            sampling::set_monitor_paused,
            sampling::set_monitor_interval,
            companion::configure_companion,
            companion::show_companion,
            companion::hide_companion,
            companion::open_main_window,
            companion::quit_app,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Reopen {
                has_visible_windows,
                ..
            } = event
            {
                // Reactivating the app must not steal focus from Settings or About.
                // Restore the monitor when all document windows are hidden instead.
                let auxiliary_visible = ["settings", "about"].iter().any(|label| {
                    app.get_webview_window(label)
                        .is_some_and(|window| window.is_visible().unwrap_or(false))
                });
                if !has_visible_windows || !auxiliary_visible {
                    let _ = companion::open_main(app);
                }
            }
            #[cfg(not(target_os = "macos"))]
            let _ = (app, event);
        });
}
