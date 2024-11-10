import { writable } from "svelte/store";
import { themes, type Theme } from "$lib/styles";

function createThemeStore() {
  // Default theme
  const defaultTheme = themes.catppuccin;

  // Initialize with default theme
  const { subscribe, set } = writable<Theme>(defaultTheme);

  // Initialize theme on client-side only
  if (typeof window !== "undefined") {
    const storedTheme = localStorage.getItem("theme");
    if (storedTheme && themes[storedTheme]) {
      set(themes[storedTheme]);
    }
  }

  return {
    subscribe,
    setTheme: (themeName: string) => {
      const theme = themes[themeName];
      if (theme) {
        if (typeof window !== "undefined") {
          localStorage.setItem("theme", themeName);
        }
        set(theme);
        applyTheme(theme);
      }
    },
    init: () => {
      const storedTheme =
        typeof window !== "undefined" ? localStorage.getItem("theme") : null;
      const theme = (storedTheme && themes[storedTheme]) || defaultTheme;
      applyTheme(theme);
    },
  };
}

function applyTheme(theme: Theme) {
  if (typeof window !== "undefined") {
    const root = document.documentElement;
    Object.entries(theme.colors).forEach(([key, value]) => {
      root.style.setProperty(`--${key}`, value);
    });
  }
}

export const themeStore = createThemeStore();
