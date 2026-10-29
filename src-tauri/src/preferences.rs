//! One serialized source of preferences for all desktop windows.
use serde::Serialize;
use serde_json::Value;
use std::{fs, path::Path, sync::Mutex};
use tauri::{AppHandle, Emitter, Manager, WebviewWindow};

fn defaults() -> Value {
    serde_json::from_str(include_str!(
        "../../src/lib/definitions/default-config.json"
    ))
    .unwrap()
}
#[derive(Clone, Serialize)]
pub struct Snapshot {
    config: Value,
    revision: u64,
    ready: bool,
}
pub struct Preferences(pub Mutex<Snapshot>);
impl Default for Preferences {
    fn default() -> Self {
        Self(Mutex::new(Snapshot {
            config: defaults(),
            revision: 0,
            ready: false,
        }))
    }
}
// Only recognized fields are accepted. Old installations may have partial
// configs, so merge valid leaves into current defaults rather than replacing groups.
fn merge(config: &mut Value, patch: &Value) {
    for group in ["appearance", "behavior"] {
        let Some(values) = patch[group].as_object() else {
            continue;
        };
        for (key, value) in values {
            if group == "appearance" && key == "columnVisibility" {
                if let Some(columns) = value.as_object() {
                    for (column, visible) in columns {
                        if config[group][key].get(column).is_some() && visible.is_boolean() {
                            config[group][key][column] = visible.clone();
                        }
                    }
                }
                continue;
            }
            let valid = match key.as_str() {
                "itemsPerPage" => value
                    .as_u64()
                    .is_some_and(|n| [15, 25, 50, 100, 250, 500].contains(&n)),
                "refreshRate" => value.as_u64().is_some_and(|n| (1000..=30000).contains(&n)),
                "menuBarEnabled" | "keepRunning" | "usageAnalytics" => value.is_boolean(),
                "processView" => matches!(value.as_str(), Some("flat" | "tree")),
                "defaultStatusFilter" => matches!(
                    value.as_str(),
                    Some("all" | "running" | "sleeping" | "idle" | "unknown")
                ),
                _ => false,
            };
            if group == "behavior" && valid {
                config[group][key] = value.clone();
            }
        }
    }
    config["appearance"]["columnVisibility"]["name"] = Value::Bool(true);
}
fn persist(path: &Path, config: &Value) -> Result<(), String> {
    let parent = path.parent().ok_or("Settings location unavailable")?;
    fs::create_dir_all(parent).map_err(|e| format!("Could not create settings folder: {e}"))?;
    let temporary = path.with_extension("json.tmp");
    fs::write(
        &temporary,
        serde_json::to_vec_pretty(config).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("Could not save settings: {e}"))?;
    fs::rename(&temporary, path).map_err(|e| {
        let _ = fs::remove_file(&temporary);
        format!("Could not replace settings: {e}")
    })
}
fn load(path: &Path, legacy: Option<&Value>) -> Result<Option<Value>, String> {
    let existing = match fs::read(path) {
        Ok(bytes) => Some(
            serde_json::from_slice::<Value>(&bytes)
                .map_err(|e| format!("Could not read settings: {e}"))?,
        ),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => return Err(format!("Could not read settings: {e}")),
    };
    let Some(source) = existing.as_ref().or(legacy) else {
        return Ok(None);
    };
    let mut config = defaults();
    merge(&mut config, source);
    if existing.is_none() {
        persist(path, &config)?;
    }
    Ok(Some(config))
}
#[tauri::command]
pub async fn read_preferences(
    app: AppHandle,
    window: WebviewWindow,
    legacy: Option<Value>,
) -> Result<Snapshot, String> {
    let migrate = window.label() == "main";
    tauri::async_runtime::spawn_blocking(move || {
        let service = app.state::<Preferences>();
        let mut state = service.0.lock().map_err(|e| e.to_string())?;
        if !state.ready {
            let path = app
                .path()
                .app_config_dir()
                .map_err(|e| e.to_string())?
                .join("settings.json");
            if let Some(config) = load(&path, if migrate { legacy.as_ref() } else { None })? {
                state.config = config;
                state.ready = true;
                state.revision += 1;
                app.emit("preferences-change", state.clone())
                    .map_err(|e| e.to_string())?;
            }
        }
        if migrate && state.ready {
            crate::telemetry::configure(
                &app,
                state.config["behavior"]["usageAnalytics"]
                    .as_bool()
                    .unwrap_or(false),
            )?;
        }
        Ok(state.clone())
    })
    .await
    .map_err(|e| e.to_string())?
}
#[tauri::command]
pub async fn update_preferences(app: AppHandle, patch: Value) -> Result<Snapshot, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let service = app.state::<Preferences>();
        let mut state = service.0.lock().map_err(|e| e.to_string())?;
        if !state.ready {
            return Err("Settings are still starting. Try again shortly.".into());
        }
        let mut next = state.config.clone();
        merge(&mut next, &patch);
        if next != state.config {
            let path = app
                .path()
                .app_config_dir()
                .map_err(|e| e.to_string())?
                .join("settings.json");
            persist(&path, &next)?;
            if let Err(error) = crate::telemetry::configure(
                &app,
                next["behavior"]["usageAnalytics"]
                    .as_bool()
                    .unwrap_or(false),
            ) {
                persist(&path, &state.config).map_err(|rollback| format!("{error}. {rollback}"))?;
                return Err(error);
            }
            state.config = next;
            state.revision += 1;
            app.emit("preferences-change", state.clone())
                .map_err(|e| e.to_string())?;
        }
        Ok(state.clone())
    })
    .await
    .map_err(|e| e.to_string())?
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn legacy_partial_configs_keep_defaults_and_reject_invalid_values() {
        let mut config = defaults();
        merge(
            &mut config,
            &json!({"behavior":{"refreshRate":5000,"keepRunning":true,"itemsPerPage":0,"processView":"bad"},"appearance":{"columnVisibility":{"name":false,"pid":false,"unknown":true}}}),
        );
        assert_eq!(config["behavior"]["refreshRate"], 5000);
        assert_eq!(config["behavior"]["itemsPerPage"], 15);
        assert_eq!(config["behavior"]["processView"], "flat");
        assert_eq!(config["appearance"]["columnVisibility"]["name"], true);
        assert_eq!(config["appearance"]["columnVisibility"]["pid"], false);
        assert!(config["appearance"]["columnVisibility"]
            .get("unknown")
            .is_none());
    }
    #[test]
    fn independent_patches_do_not_overwrite_other_windows() {
        let mut config = defaults();
        merge(&mut config, &json!({"behavior":{"refreshRate":5000}}));
        merge(&mut config, &json!({"behavior":{"processView":"tree"}}));
        merge(
            &mut config,
            &json!({"appearance":{"columnVisibility":{"pid":false}}}),
        );
        assert_eq!(config["behavior"]["refreshRate"], 5000);
        assert_eq!(config["behavior"]["processView"], "tree");
    }
    #[test]
    fn saved_settings_win_over_legacy_and_failed_save_keeps_original() {
        let dir = std::env::temp_dir().join(format!("neohtop-preferences-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("settings.json");
        assert!(load(&path, None).unwrap().is_none());
        let config = load(&path, Some(&json!({"behavior":{"refreshRate":5000}})))
            .unwrap()
            .unwrap();
        assert_eq!(load(&path, Some(&defaults())).unwrap().unwrap(), config);
        assert!(persist(&dir, &defaults()).is_err());
        assert_eq!(load(&path, None).unwrap().unwrap(), config);
        fs::remove_dir_all(dir).unwrap();
    }
}
