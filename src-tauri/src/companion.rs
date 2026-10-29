//! Native tray lifetime and a separate, rounded overview webview.
use serde::{Deserialize, Serialize};
use std::{
    sync::Mutex,
    time::{Duration, Instant},
};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, Rect, WindowEvent};

const TRAY_ID: &str = "neohtop-overview";
#[derive(Debug, Default)]
pub struct CompanionState {
    pub keep_running: bool,
    pub hidden_at: Option<Instant>,
}

// A monochrome pixel ghost, matching the public app's silhouette. macOS treats
// it as a template so the OS controls menu-bar contrast in either appearance.
fn ghost() -> tauri::image::Image<'static> {
    let mut rgba = vec![0; 32 * 32 * 4];
    for y in 0..32i32 {
        for x in 0..32i32 {
            let head = (x - 16).pow(2) + (y - 13).pow(2) <= 100 && y <= 14;
            let body = (6..26).contains(&x) && (13..26).contains(&y);
            let hem = (6..26).contains(&x) && (26..29).contains(&y) && ((x - 6) / 4) % 2 == 0;
            let eye = ((10..13).contains(&x) || (19..22).contains(&x)) && (12..17).contains(&y);
            if (head || body || hem) && !eye {
                let i = ((y * 32 + x) * 4) as usize;
                // White on Windows/Linux; macOS uses alpha as a black template.
                #[cfg(not(target_os = "macos"))]
                {
                    rgba[i..i + 3].fill(255);
                }
                rgba[i + 3] = 255;
            }
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        let original = rgba.clone();
        for y in 1..31usize {
            for x in 1..31usize {
                let i = (y * 32 + x) * 4;
                if original[i + 3] != 0
                    && [i - 4, i + 4, i - 128, i + 128]
                        .iter()
                        .any(|n| original[n + 3] == 0)
                {
                    rgba[i..i + 3].fill(30);
                }
            }
        }
    }
    tauri::image::Image::new_owned(rgba, 32, 32)
}

fn configure(app: &AppHandle, enabled: bool, keep_running: bool) -> Result<(), String> {
    // Called only on the main thread: native menus and tray destruction require it.
    if enabled && app.tray_by_id(TRAY_ID).is_none() {
        let overview = MenuItem::with_id(app, "overview", "Show overview", true, None::<&str>)
            .map_err(|e| e.to_string())?;
        let open = MenuItem::with_id(app, "open-main", "Open NeoHtop", true, None::<&str>)
            .map_err(|e| e.to_string())?;
        let quit = MenuItem::with_id(app, "quit", "Quit NeoHtop", true, None::<&str>)
            .map_err(|e| e.to_string())?;
        let menu = Menu::with_items(app, &[&overview, &open, &quit]).map_err(|e| e.to_string())?;
        TrayIconBuilder::with_id(TRAY_ID)
            .icon(ghost())
            .icon_as_template(true)
            .tooltip("NeoHtop — system overview")
            .menu(&menu)
            .menu_on_left_click(false)
            .on_menu_event(|app, event| {
                let _ = match event.id.as_ref() {
                    "overview" => show(app, None),
                    "open-main" => open_main(app),
                    "quit" => {
                        app.exit(0);
                        Ok(())
                    }
                    _ => Ok(()),
                };
            })
            .on_tray_icon_event(|tray, event| {
                if let TrayIconEvent::Click {
                    rect,
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } = event
                {
                    let app = tray.app_handle();
                    let Some(window) = app.get_webview_window("overview") else {
                        return;
                    };
                    if window.is_visible().unwrap_or(false) {
                        hide(app);
                    } else {
                        let recent = app
                            .state::<Mutex<CompanionState>>()
                            .lock()
                            .ok()
                            .and_then(|s| s.hidden_at)
                            .is_some_and(|t| t.elapsed() < Duration::from_millis(200));
                        if !recent {
                            let _ = show(app, Some(rect));
                        }
                    }
                }
            })
            .build(app)
            .map_err(|e| e.to_string())?;
    } else if !enabled {
        drop(app.remove_tray_by_id(TRAY_ID));
    }
    // Only opt in to hiding the main window when the tray actually exists.
    app.state::<Mutex<CompanionState>>()
        .lock()
        .map_err(|e| e.to_string())?
        .keep_running = enabled && keep_running;
    Ok(())
}

pub fn hide(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("overview") {
        let _ = window.hide();
    }
    if let Ok(mut state) = app.state::<Mutex<CompanionState>>().lock() {
        state.hidden_at = Some(Instant::now());
    }
    let _ = app.emit_to("overview", "companion-visibility", false);
}

// All geometry is physical pixels. Negative monitor origins and scale factors
// must not be mixed with logical popup dimensions.
fn placement(
    bounds: (i32, i32, u32, u32),
    panel: (u32, u32),
    anchor: Option<(f64, f64, f64, f64)>,
    scale: f64,
) -> PhysicalPosition<i32> {
    let (left, top, width, height) = bounds;
    let (pw, ph) = panel;
    let margin = (8.0 * scale).round() as i32;
    let right = left.saturating_add(width as i32);
    let bottom = top.saturating_add(height as i32);
    let (x, y) = if let Some((ax, ay, aw, ah)) = anchor {
        let x = (ax + aw / 2.0 - pw as f64 / 2.0).round() as i32;
        let y = if ay + ah + ph as f64 <= bottom as f64 {
            (ay + ah).round() as i32
        } else {
            (ay - ph as f64).round() as i32
        };
        (x, y)
    } else {
        (right - pw as i32 - margin, top + (32.0 * scale) as i32)
    };
    PhysicalPosition::new(
        x.clamp(left, (right - pw as i32).max(left)),
        y.clamp(top, (bottom - ph as i32).max(top)),
    )
}
fn show(app: &AppHandle, rect: Option<Rect>) -> Result<(), String> {
    let window = app
        .get_webview_window("overview")
        .ok_or("Overview window unavailable")?;
    let rect = rect.or_else(|| {
        app.tray_by_id(TRAY_ID)
            .and_then(|tray| tray.rect().ok().flatten())
    });
    let anchor = rect.map(|r| {
        // Tray rects are converted using the monitor scale below. Tauri tray
        // events currently provide physical coordinates on macOS and Windows.
        let p = r.position.to_physical::<f64>(1.0);
        let s = r.size.to_physical::<f64>(1.0);
        (p.x, p.y, s.width, s.height)
    });
    let monitor = anchor
        .and_then(|(x, y, _, _)| window.monitor_from_point(x, y).ok().flatten())
        .or_else(|| {
            app.get_webview_window("main")
                .and_then(|w| w.current_monitor().ok().flatten())
        })
        .or_else(|| window.primary_monitor().ok().flatten())
        .ok_or("No display available")?;
    let scale = monitor.scale_factor();
    let size = tauri::LogicalSize::new(400.0, 600.0).to_physical::<u32>(scale);
    window
        .set_position(placement(
            (
                monitor.position().x,
                monitor.position().y,
                monitor.size().width,
                monitor.size().height,
            ),
            (size.width, size.height),
            anchor,
            scale,
        ))
        .map_err(|e| e.to_string())?;
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    app.emit_to("overview", "companion-visibility", true)
        .map_err(|e| e.to_string())
}
pub fn open_main(app: &AppHandle) -> Result<(), String> {
    hide(app);
    let window = app
        .get_webview_window("main")
        .ok_or("Main window unavailable")?;
    window.unminimize().map_err(|e| e.to_string())?;
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())
}
pub fn window_event(window: &tauri::Window, event: &WindowEvent) {
    let app = window.app_handle();
    match (window.label(), event) {
        ("overview", WindowEvent::Focused(false)) => hide(app),
        ("overview", WindowEvent::CloseRequested { api, .. }) => {
            api.prevent_close();
            hide(app);
        }
        ("settings" | "about", WindowEvent::CloseRequested { api, .. }) => {
            api.prevent_close();
            let _ = window.hide();
        }
        ("main", WindowEvent::CloseRequested { api, .. }) => {
            let keep = app
                .state::<Mutex<CompanionState>>()
                .lock()
                .map(|s| s.keep_running)
                .unwrap_or(false);
            api.prevent_close();
            if keep {
                let _ = window.hide();
            } else {
                app.exit(0);
            }
        }
        _ => {}
    }
}

#[tauri::command]
pub async fn configure_companion(
    app: AppHandle,
    enabled: bool,
    keep_running: bool,
) -> Result<(), String> {
    let (sender, mut receiver) = tauri::async_runtime::channel(1);
    let handle = app.clone();
    app.run_on_main_thread(move || {
        let result = configure(&handle, enabled, keep_running);
        let _ = sender.try_send(result);
    })
    .map_err(|e| e.to_string())?;
    receiver
        .recv()
        .await
        .ok_or("Companion configuration was interrupted")?
}
#[tauri::command]
pub async fn show_companion(app: AppHandle) -> Result<(), String> {
    show(&app, None)
}
#[tauri::command]
pub async fn hide_companion(app: AppHandle) {
    hide(&app);
}
#[derive(Deserialize, Serialize, Clone)]
pub struct ProcessTarget {
    pid: u32,
    start_time: u64,
}
#[tauri::command]
pub async fn open_main_window(
    app: AppHandle,
    process: Option<ProcessTarget>,
) -> Result<(), String> {
    if let Some(target) = process {
        let state = app.state::<crate::state::AppState>();
        let frame = state
            .hub
            .lock()
            .map_err(|e| e.to_string())?
            .frame
            .clone()
            .ok_or("No process readings yet")?;
        if !frame
            .processes
            .iter()
            .any(|p| p.pid == target.pid && p.start_time == target.start_time)
        {
            return Err("This process is no longer available".into());
        }
        open_main(&app)?;
        app.emit_to("main", "open-process", target)
            .map_err(|e| e.to_string())
    } else {
        open_main(&app)
    }
}
#[tauri::command]
pub async fn quit_app(app: AppHandle) {
    app.exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn panel_stays_on_scaled_and_negative_displays() {
        assert_eq!(
            placement(
                (0, 0, 2880, 1800),
                (800, 1200),
                Some((2800.0, 0.0, 40.0, 48.0)),
                2.0
            ),
            PhysicalPosition::new(2080, 48)
        );
        assert_eq!(
            placement(
                (-1920, 0, 1920, 1080),
                (400, 600),
                Some((-1900.0, 1040.0, 24.0, 40.0)),
                1.0
            ),
            PhysicalPosition::new(-1920, 440)
        );
        assert_eq!(
            placement((0, 0, 300, 400), (400, 600), None, 1.0),
            PhysicalPosition::new(0, 0)
        );
    }
}
