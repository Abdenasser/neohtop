//! Native application menus and persistent auxiliary windows.
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem as Stock, Submenu},
    AppHandle, Emitter, Manager,
};
#[derive(Default)]
pub struct DesktopState(pub Mutex<String>);
fn show(app: &AppHandle, label: &str) -> Result<(), String> {
    crate::companion::hide(app);
    let window = app.get_webview_window(label).ok_or("Window unavailable")?;
    window.unminimize().map_err(|e| e.to_string())?;
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())
}
#[tauri::command]
pub fn open_settings(app: AppHandle, pane: Option<String>) -> Result<(), String> {
    if let Some(pane) = pane {
        if pane == "general" || pane == "appearance" {
            *app.state::<DesktopState>()
                .0
                .lock()
                .map_err(|e| e.to_string())? = pane.clone();
            app.emit_to("settings", "settings-pane", pane)
                .map_err(|e| e.to_string())?;
        }
    }
    show(&app, "settings")
}
#[tauri::command]
pub fn settings_pane(app: AppHandle) -> String {
    app.state::<DesktopState>()
        .0
        .lock()
        .map(|s| s.clone())
        .unwrap_or_default()
}
#[tauri::command]
pub fn open_about(app: AppHandle, check: bool) -> Result<(), String> {
    show(&app, "about")?;
    if check {
        app.emit_to("about", "check-updates", ())
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
pub fn setup(app: &AppHandle) -> tauri::Result<()> {
    let about = MenuItem::with_id(app, "app-about", "About NeoHtop", true, None::<&str>)?;
    let updates = MenuItem::with_id(app, "app-updates", "Check for Updates…", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "app-settings", "Settings…", true, Some("CmdOrCtrl+,"))?;
    let quit = Stock::quit(app, Some("Quit NeoHtop"))?;
    let application = Submenu::with_items(
        app,
        "NeoHtop",
        true,
        &[&about, &updates, &Stock::separator(app)?, &settings],
    )?;
    #[cfg(target_os = "macos")]
    application.append_items(&[
        &Stock::separator(app)?,
        &Stock::services(app, None)?,
        &Stock::separator(app)?,
        &Stock::hide(app, None)?,
        &Stock::hide_others(app, None)?,
        &Stock::show_all(app, None)?,
    ])?;
    application.append_items(&[&Stock::separator(app)?, &quit])?;
    let edit = Submenu::with_items(
        app,
        "Edit",
        true,
        &[
            &Stock::undo(app, None)?,
            &Stock::redo(app, None)?,
            &Stock::separator(app)?,
            &Stock::cut(app, None)?,
            &Stock::copy(app, None)?,
            &Stock::paste(app, None)?,
            &Stock::select_all(app, None)?,
        ],
    )?;
    let list = MenuItem::with_id(app, "view-list", "Process List", true, None::<&str>)?;
    let tree = MenuItem::with_id(app, "view-tree", "Process Tree", true, None::<&str>)?;
    let appearance = MenuItem::with_id(app, "view-appearance", "Appearance…", true, None::<&str>)?;
    let overview = MenuItem::with_id(
        app,
        "view-overview",
        "Menu-bar Overview",
        true,
        None::<&str>,
    )?;
    let view = Submenu::with_items(
        app,
        "View",
        true,
        &[
            &list,
            &tree,
            &Stock::separator(app)?,
            &overview,
            &appearance,
        ],
    )?;
    let window = Submenu::with_items(
        app,
        "Window",
        true,
        &[
            &Stock::minimize(app, None)?,
            &Stock::close_window(app, None)?,
        ],
    )?;
    app.set_menu(Menu::with_items(
        app,
        &[&application, &edit, &view, &window],
    )?)?;
    app.on_menu_event(|app, event| {
        let _ = match event.id.as_ref() {
            "app-settings" => open_settings(app.clone(), None),
            "app-about" => open_about(app.clone(), false),
            "app-updates" => open_about(app.clone(), true),
            "view-appearance" => open_settings(app.clone(), Some("appearance".into())),
            "view-overview" => {
                let handle = app.clone();
                tauri::async_runtime::spawn(async move {
                    let _ = crate::companion::show_companion(handle).await;
                });
                Ok(())
            }
            command @ ("view-list" | "view-tree") => {
                crate::companion::open_main(app).and_then(|_| {
                    app.emit_to("main", "app-command", command)
                        .map_err(|e| e.to_string())
                })
            }
            _ => Ok(()),
        };
    });
    Ok(())
}
