import { invoke, isTauri } from "@tauri-apps/api/core";
// This helper accepts only product actions. No process, search, path or metric data.
export function trackUsage(event: string, value?: string) {
  if (isTauri())
    void invoke("track_usage", { event, value: value ?? null }).catch(() => {});
}
