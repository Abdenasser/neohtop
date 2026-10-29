import { trackUsage } from "$lib/utils/telemetry";
import { emit, listen } from "@tauri-apps/api/event";
import { isTauri } from "@tauri-apps/api/core";
import { writable } from "svelte/store";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { themes } from "$lib/definitions/themes";
import { themeTokens, themeIsDark } from "$lib/utils/theme-tokens.js";
import type { Theme } from "$lib/types";

function applyTheme(theme: Theme) {
  if (typeof document === "undefined") return;
  const root = document.documentElement;
  root.dataset.theme = theme.name;
  const dark = themeIsDark(theme.colors);
  root.classList.toggle("dark", dark);
  root.style.colorScheme = dark ? "dark" : "light";
  if (
    isTauri() &&
    new URLSearchParams(window.location.search).get("view") !== "companion"
  )
    void getCurrentWindow()
      .setTheme(dark ? "dark" : "light")
      .catch(() => {});
  for (const [name, value] of Object.entries({
    ...theme.colors,
    ...themeTokens(theme.colors, { name: theme.name }),
  }))
    root.style.setProperty(`--${name}`, value);
}
function storedTheme() {
  if (typeof localStorage === "undefined") return themes.catppuccin;
  try {
    return themes[localStorage.getItem("theme") || ""] || themes.catppuccin;
  } catch {
    return themes.catppuccin;
  }
}
function createThemeStore() {
  const { subscribe, set } = writable<Theme>(storedTheme());
  let listening = false;
  return {
    subscribe,
    setTheme(name: string) {
      const theme = themes[name];
      if (!theme) return;
      try {
        localStorage.setItem("theme", name);
      } catch {
        /* Current-session theming still works when storage is unavailable. */
      }
      set(theme);
      applyTheme(theme);
      trackUsage("theme_changed", name);
      if (isTauri()) void emit("theme-change", name);
    },
    init() {
      if (isTauri() && !listening) {
        listening = true;
        void listen<string>("theme-change", ({ payload }) => {
          const next = themes[payload];
          if (next) {
            set(next);
            applyTheme(next);
          }
        });
      }
      const theme = storedTheme();
      set(theme);
      applyTheme(theme);
    },
  };
}
export const themeStore = createThemeStore();
