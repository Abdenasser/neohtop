import { writable } from "svelte/store";
import type { AppConfig } from "$lib/types/config";
import { DEFAULT_CONFIG } from "$lib/types/config";

function createConfigStore() {
  const { subscribe, set, update } = writable<AppConfig>(DEFAULT_CONFIG);

  return {
    subscribe,
    init: () => {
      if (typeof window !== "undefined") {
        const stored = localStorage.getItem("neohtop_config");
        if (stored) {
          try {
            const config = JSON.parse(stored);
            set({ ...DEFAULT_CONFIG, ...config });
          } catch (e) {
            console.error("Failed to parse stored config:", e);
            set(DEFAULT_CONFIG);
          }
        }
      }
    },
    updateConfig: (newConfig: Partial<AppConfig>) => {
      update((config) => {
        const updated = { ...config, ...newConfig };
        if (typeof window !== "undefined") {
          localStorage.setItem("neohtop_config", JSON.stringify(updated));
        }
        return updated;
      });
    },
  };
}

export const configStore = createConfigStore();
