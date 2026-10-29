// App-owned configuration: changing or hiding toolbar controls never owns sampling/tray lifetime.
import { get, writable } from "svelte/store";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { committedSettings } from "./settings";
export const monitoringPreferencesError = writable("");
let applied = "";
let queue = Promise.resolve();
function apply() {
  if (!isTauri()) return queue;
  const { behavior } = get(committedSettings);
  const key = JSON.stringify([
    behavior.refreshRate,
    behavior.menuBarEnabled,
    behavior.keepRunning,
  ]);
  if (key === applied) return queue;
  applied = key;
  queue = queue.then(async () => {
    try {
      await invoke("set_monitor_interval", {
        milliseconds: behavior.refreshRate,
      });
      await invoke("configure_companion", {
        enabled: behavior.menuBarEnabled,
        keepRunning: behavior.keepRunning,
      });
      monitoringPreferencesError.set("");
    } catch (error) {
      applied = "";
      monitoringPreferencesError.set(String(error));
    }
  });
  return queue;
}
export async function startMonitoringPreferences() {
  await apply();
  return committedSettings.subscribe(() => {
    void apply();
  });
}
export function retryMonitoringPreferences() {
  applied = "";
  return apply();
}
