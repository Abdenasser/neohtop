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
        if pane == "general" || pane == "appearance" || pane == "privacy" {
            *app.state::<DesktopState>()
                .0
                .lock()
                .map_err(|e| e.to_string())? = pane.clone();
            app.emit_to("settings", "settings-pane", pane)
                .map_err(|e| e.to_string())?;
        }
    }
    crate::telemetry::track(&app, "settings_open", None);
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
pub fn apply_language(app: &AppHandle, chinese: bool) -> tauri::Result<()> {
    let text = |en: &'static str, zh: &'static str| if chinese { zh } else { en };
    let about_title = text("About NeoHtop", "关于 NeoHtop");
    let about = MenuItem::with_id(app, "app-about", about_title, true, None::<&str>)?;
    let updates = MenuItem::with_id(
        app,
        "app-updates",
        text("Check for Updates…", "检查更新…"),
        true,
        None::<&str>,
    )?;
    let settings = MenuItem::with_id(
        app,
        "app-settings",
        text("Settings…", "设置…"),
        true,
        Some("CmdOrCtrl+,"),
    )?;
    let quit = Stock::quit(app, Some(text("Quit NeoHtop", "退出 NeoHtop")))?;
    let application = Submenu::with_items(
        app,
        "NeoHtop",
        true,
        &[&about, &updates, &Stock::separator(app)?, &settings],
    )?;
    #[cfg(target_os = "macos")]
    application.append_items(&[
        &Stock::separator(app)?,
        &Stock::services(app, chinese.then_some("服务"))?,
        &Stock::separator(app)?,
        &Stock::hide(app, chinese.then_some("隐藏 NeoHtop"))?,
        &Stock::hide_others(app, chinese.then_some("隐藏其他应用"))?,
        &Stock::show_all(app, chinese.then_some("显示全部"))?,
    ])?;
    application.append_items(&[&Stock::separator(app)?, &quit])?;
    let edit = Submenu::with_items(
        app,
        text("Edit", "编辑"),
        true,
        &[
            &Stock::undo(app, chinese.then_some("撤销"))?,
            &Stock::redo(app, chinese.then_some("重做"))?,
            &Stock::separator(app)?,
            &Stock::cut(app, chinese.then_some("剪切"))?,
            &Stock::copy(app, chinese.then_some("复制"))?,
            &Stock::paste(app, chinese.then_some("粘贴"))?,
            &Stock::select_all(app, chinese.then_some("全选"))?,
        ],
    )?;
    let list = MenuItem::with_id(
        app,
        "view-list",
        text("Process List", "进程列表"),
        true,
        None::<&str>,
    )?;
    let tree = MenuItem::with_id(
        app,
        "view-tree",
        text("Process Tree", "进程树"),
        true,
        None::<&str>,
    )?;
    let appearance = MenuItem::with_id(
        app,
        "view-appearance",
        text("Appearance…", "外观…"),
        true,
        None::<&str>,
    )?;
    let overview = MenuItem::with_id(
        app,
        "view-overview",
        text("Menu-bar Overview", "菜单栏概览"),
        true,
        None::<&str>,
    )?;
    let view = Submenu::with_items(
        app,
        text("View", "显示"),
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
        text("Window", "窗口"),
        true,
        &[
            &Stock::minimize(app, chinese.then_some("最小化"))?,
            &Stock::close_window(app, chinese.then_some("关闭窗口"))?,
        ],
    )?;
    app.set_menu(Menu::with_items(
        app,
        &[&application, &edit, &view, &window],
    )?)?;
    for (label, title) in [
        ("settings", text("Settings", "设置")),
        ("about", about_title),
    ] {
        if let Some(window) = app.get_webview_window(label) {
            window.set_title(title)?;
        }
    }
    Ok(())
}
pub fn setup(app: &AppHandle) -> tauri::Result<()> {
    apply_language(app, false)?;
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
