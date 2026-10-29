//! Fixed product events only. Process/system readings never enter this module.
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    fs,
    sync::Mutex,
    time::{Duration, Instant},
};
use tauri::{AppHandle, Manager};
use uuid::Uuid;
#[derive(Default)]
struct State {
    started: bool,
    enabled: bool,
    identity: Option<String>,
    session_id: u64,
    last: HashMap<String, Instant>,
}
#[derive(Default)]
pub struct Telemetry(Mutex<State>);
fn endpoint() -> Option<String> {
    let url = option_env!("NEOHTOP_ANALYTICS_ENDPOINT")?;
    let parsed = reqwest::Url::parse(url).ok()?;
    (parsed.scheme() == "https"
        && parsed.host_str().is_some()
        && parsed.username().is_empty()
        && parsed.password().is_none()
        && parsed.query().is_none()
        && parsed.fragment().is_none()
        && parsed.path() == "/v1/events")
        .then(|| url.to_owned())
}
fn events() -> Value {
    serde_json::from_str(include_str!("../../telemetry/events.json")).unwrap()
}
fn valid(event: &str, value: Option<&str>) -> bool {
    let spec = events();
    let Some(choices) = spec[event].as_array() else {
        return false;
    };
    if choices.is_empty() {
        value.is_none()
    } else {
        value.is_some_and(|value| choices.iter().any(|choice| choice.as_str() == Some(value)))
    }
}
fn basic() -> Value {
    json!({"schema":1,"mode":"basic","event":"desktop_open","version":env!("CARGO_PKG_VERSION")})
}
fn usage(id: &str, session_id: u64, event: &str, value: Option<&str>) -> Value {
    let mut payload = json!({"schema":1,"mode":"usage","event":event,"version":env!("CARGO_PKG_VERSION"),"client_id":id,"session_id":session_id,"platform":std::env::consts::OS});
    if let Some(value) = value {
        payload["value"] = json!(value);
    }
    payload
}
fn send(app: &AppHandle, payload: Value) {
    let Some(url) = endpoint() else { return };
    let handle = app.clone();
    tauri::async_runtime::spawn(async move {
        // Recheck consent immediately before optional delivery. No disk queue or retries.
        if payload["mode"] == "usage" {
            let state = handle.state::<Telemetry>();
            let Ok(state) = state.0.lock() else { return };
            if !state.enabled || state.identity.as_deref() != payload["client_id"].as_str() {
                return;
            }
        }
        let Ok(client) = reqwest::Client::builder()
            .https_only(true)
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(4))
            .user_agent(concat!("NeoHtop/", env!("CARGO_PKG_VERSION")))
            .build()
        else {
            return;
        };
        let _ = client.post(url).json(&payload).send().await;
    });
}
pub fn configure(app: &AppHandle, enabled: bool) -> Result<(), String> {
    let configured = endpoint().is_some();
    let service = app.state::<Telemetry>();
    let mut state = service.0.lock().map_err(|e| e.to_string())?;
    let path = app
        .path()
        .app_config_dir()
        .map_err(|e| e.to_string())?
        .join("usage-id");
    if enabled && configured {
        if state.identity.is_none() {
            let saved = fs::read_to_string(&path)
                .ok()
                .filter(|id| Uuid::parse_str(id.trim()).is_ok());
            let id = saved
                .map(|id| id.trim().to_owned())
                .unwrap_or_else(|| Uuid::new_v4().to_string());
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            fs::write(&path, &id).map_err(|e| format!("Could not save usage preference: {e}"))?;
            state.identity = Some(id);
        }
    } else {
        state.identity = None;
        state.last.clear();
        match fs::remove_file(&path) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(format!("Could not reset usage identifier: {e}")),
        }
    }
    state.enabled = enabled && configured;
    let initial = if !state.started {
        state.started = true;
        state.session_id = crate::sampling::now_ms() / 1000;
        Some(if state.enabled {
            usage(
                state.identity.as_deref().unwrap(),
                state.session_id,
                "desktop_open",
                None,
            )
        } else {
            basic()
        })
    } else {
        None
    };
    drop(state);
    if let Some(payload) = initial {
        send(app, payload);
    }
    Ok(())
}
pub fn track(app: &AppHandle, event: &str, value: Option<&str>) {
    if !valid(event, value) || event == "desktop_open" {
        return;
    }
    let service = app.state::<Telemetry>();
    let Ok(mut state) = service.0.lock() else {
        return;
    };
    if !state.enabled {
        return;
    }
    let key = format!("{event}:{}", value.unwrap_or_default());
    if state
        .last
        .get(&key)
        .is_some_and(|last| last.elapsed() < Duration::from_secs(30))
    {
        return;
    }
    // Event/choice allowlists bound the map; a refresh loop cannot create an event flood.
    state.last.insert(key, Instant::now());
    let Some(id) = state.identity.as_deref() else {
        return;
    };
    let payload = usage(id, state.session_id, event, value);
    drop(state);
    send(app, payload);
}
#[tauri::command]
pub fn track_usage(app: AppHandle, event: String, value: Option<String>) {
    track(&app, &event, value.as_deref());
}
#[tauri::command]
pub fn analytics_configured() -> bool {
    endpoint().is_some()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_open_has_no_identifier_or_machine_data() {
        let p = basic();
        assert_eq!(p.as_object().unwrap().len(), 4);
        assert!(p.get("client_id").is_none());
        assert!(p.get("platform").is_none());
    }
    #[test]
    fn only_fixed_events_and_choices_are_accepted() {
        assert!(valid("resource_view", Some("cpu")));
        assert!(valid("theme_changed", Some("catppuccin")));
        assert!(!valid("resource_view", Some("/Users/private/file")));
        assert!(!valid("search_query", Some("secret")));
        assert!(!valid("companion_open", Some("process-name")));
    }
    #[test]
    fn opt_in_payload_is_bounded() {
        let p = usage(
            "00000000-0000-4000-8000-000000000000",
            1700000000,
            "resource_view",
            Some("gpu"),
        );
        assert_eq!(p["value"], "gpu");
        assert!(p.get("processes").is_none());
        assert!(p.get("ip_address").is_none());
    }
}
