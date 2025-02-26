export const ASCII_ART = `
███╗   ██╗███████╗ ██████╗ ██╗  ██╗████████╗ ██████╗ ██████╗ 
████╗  ██║██╔════╝██╔═══██╗██║  ██║╚══██╔══╝██╔═══██╗██╔══██╗
██╔██╗ ██║█████╗  ██║   ██║███████║   ██║   ██║   ██║██████╔╝
██║╚██╗██║██╔══╝  ██║   ██║██╔══██║   ██║   ██║   ██║██╔═══╝ 
██║ ╚████║███████╗╚██████╔╝██║  ██║   ██║   ╚██████╔╝██║     
╚═╝  ╚═══╝╚══════╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝    ╚═════╝ ╚═╝     
`;

export const APP_INFO = {
  name: "NeoHtop",
  developer: "Abdenasser",
  github: "https://github.com/Abdenasser/neohtop",
  stack: ["Tauri", "Rust", "Svelte", "TypeScript"],
};

export const ITEMS_PER_PAGE_OPTIONS = [15, 25, 50, 100, 250, 500];

export const REFRESH_RATE_OPTIONS = [
  { value: 1000, label: "1s" },
  { value: 2000, label: "2s" },
  { value: 5000, label: "5s" },
  { value: 10000, label: "10s" },
  { value: 30000, label: "30s" },
];

export const STATUS_OPTIONS = [
  { value: "all", label: "All Statuses" },
  { value: "running", label: "Running" },
  { value: "sleeping", label: "Sleeping" },
  { value: "idle", label: "Idle" },
  { value: "unknown", label: "Unknown" },
];

export const THEME_GROUPS = [
  {
    label: "Dark",
    themes: [
      "catppuccin",
      "dracula",
      "monokaiPro",
      "tokyoNight",
      "solarizedDark",
      "ayuDark",
      "ayuMirage",
    ],
  },
  {
    label: "Light",
    themes: ["githubLight", "solarizedLight", "oneLight", "ayuLight"],
  },
  {
    label: "Warm",
    themes: ["gruvbox"],
  },
  {
    label: "Cool",
    themes: ["nord", "oneDark"],
  },
  {
    label: "Fun",
    themes: ["bubblegum", "rosePine", "cottonCandy", "synthwave", "candyfloss"],
  },
  {
    label: "Retro",
    themes: ["terminal", "amber", "ibmPC"],
  },
  {
    label: "Accessibility",
    themes: ["highContrast"],
  },
];
