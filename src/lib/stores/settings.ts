import { get, writable } from "svelte/store";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { AppConfig } from "$lib/types";
import { DEFAULT_CONFIG } from "$lib/definitions/settings";
import { changedLeaves, mergePreferences } from "$lib/utils/settings-patch.js";

export const committedSettings = writable<AppConfig>(DEFAULT_CONFIG);

type Snapshot = { config: AppConfig; revision: number; ready: boolean };
export const settingsStatus = writable({
  ready: false,
  saving: false,
  error: "",
});
const status = (
  patch: Partial<{ ready: boolean; saving: boolean; error: string }>,
) => settingsStatus.update((value) => ({ ...value, ...patch }));
function legacyConfig(): AppConfig {
  try {
    return mergePreferences(
      DEFAULT_CONFIG,
      JSON.parse(localStorage.getItem("neohtop_config") || "{}"),
    );
  } catch {
    return DEFAULT_CONFIG;
  }
}
function createSettingsStore() {
  const store = writable<AppConfig>(DEFAULT_CONFIG);
  let base = DEFAULT_CONFIG;
  let revision = -1;
  let initializing: Promise<void> | undefined;
  let listening = false;
  let queue = Promise.resolve();
  let nextId = 0;
  let failedPatch: object | undefined;
  const pending = new Map<number, object>();
  function render() {
    let config = base;
    pending.forEach((patch) => {
      config = mergePreferences(config, patch);
    });
    store.set(config);
    status({ saving: pending.size > 0 });
  }
  function accept(snapshot: Snapshot) {
    if (!snapshot.ready) return;
    if (snapshot.revision < revision) {
      render();
      return;
    }
    base = snapshot.config;
    committedSettings.set(base);
    revision = snapshot.revision;
    status({ ready: snapshot.ready });
    try {
      localStorage.setItem("neohtop_config", JSON.stringify(base));
    } catch {
      /* Native file remains authoritative. */
    }
    render();
  }
  return {
    subscribe: store.subscribe,
    init(migrate = false): Promise<void> {
      if (initializing) return initializing;
      initializing = (async () => {
        try {
          if (isTauri()) {
            if (!listening) {
              await listen<Snapshot>("preferences-change", ({ payload }) =>
                accept(payload),
              );
              listening = true;
            }
            const legacy = migrate ? legacyConfig() : null;
            accept(await invoke<Snapshot>("read_preferences", { legacy }));
          } else {
            base = legacyConfig();
            render();
            status({ ready: true });
          }
          status({ error: "" });
        } catch (error) {
          status({ error: String(error) });
          initializing = undefined;
        }
      })();
      return initializing;
    },
    retry(migrate = false): Promise<void> {
      if (failedPatch) {
        const next = mergePreferences(get(store), failedPatch);
        failedPatch = undefined;
        this.updateConfig(next);
        return queue;
      }
      initializing = undefined;
      return this.init(migrate);
    },
    updateConfig(next: Partial<AppConfig>) {
      failedPatch = undefined;
      const patch = changedLeaves(get(store), next);
      if (!Object.keys(patch).length) return;
      const id = ++nextId;
      pending.set(id, patch);
      render();
      status({ error: "" });
      queue = queue.then(async () => {
        try {
          if (isTauri()) {
            const snapshot = await invoke<Snapshot>("update_preferences", {
              patch,
            });
            pending.delete(id);
            accept(snapshot);
          } else {
            base = mergePreferences(base, patch);
            pending.delete(id);
            localStorage.setItem("neohtop_config", JSON.stringify(base));
            render();
          }
        } catch (error) {
          failedPatch = patch;
          pending.delete(id);
          render();
          status({ error: String(error) });
        }
      });
    },
  };
}
export const settingsStore = createSettingsStore();
