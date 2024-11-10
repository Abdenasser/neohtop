export interface Theme {
  name: string;
  label: string;
  colors: {
    base: string;
    mantle: string;
    crust: string;
    text: string;
    subtext0: string;
    subtext1: string;
    surface0: string;
    surface1: string;
    surface2: string;
    overlay0: string;
    overlay1: string;
    blue: string;
    lavender: string;
    sapphire: string;
    sky: string;
    red: string;
    maroon: string;
    peach: string;
    yellow: string;
    green: string;
    teal: string;
  };
}

export const themes: Record<string, Theme> = {
  catppuccin: {
    name: "catppuccin",
    label: "Catppuccin",
    colors: {
      base: "#1e1e2e",
      mantle: "#181825",
      crust: "#11111b",
      text: "#cdd6f4",
      subtext0: "#a6adc8",
      subtext1: "#bac2de",
      surface0: "#313244",
      surface1: "#45475a",
      surface2: "#585b70",
      overlay0: "#6c7086",
      overlay1: "#7f849c",
      blue: "#89b4fa",
      lavender: "#b4befe",
      sapphire: "#74c7ec",
      sky: "#89dceb",
      red: "#f38ba8",
      maroon: "#eba0ac",
      peach: "#fab387",
      yellow: "#f9e2af",
      green: "#a6e3a1",
      teal: "#94e2d5",
    },
  },
  dracula: {
    name: "dracula",
    label: "Dracula",
    colors: {
      base: "#282a36",
      mantle: "#1e1f29",
      crust: "#191a21",
      text: "#f8f8f2",
      subtext0: "#bfbfbf",
      subtext1: "#e6e6e6",
      surface0: "#44475a",
      surface1: "#6272a4",
      surface2: "#7970a9",
      overlay0: "#6272a4",
      overlay1: "#7970a9",
      blue: "#8be9fd",
      lavender: "#bd93f9",
      sapphire: "#62d6e8",
      sky: "#89ddff",
      red: "#ff5555",
      maroon: "#ff6e6e",
      peach: "#ffb86c",
      yellow: "#f1fa8c",
      green: "#50fa7b",
      teal: "#8be9fd",
    },
  },
  monokaiPro: {
    name: "monokaiPro",
    label: "Monokai Pro",
    colors: {
      base: "#2d2a2e",
      mantle: "#221f22",
      crust: "#1b1b1b",
      text: "#fcfcfa",
      subtext0: "#939293",
      subtext1: "#c1c0c0",
      surface0: "#403e41",
      surface1: "#565457",
      surface2: "#69676c",
      overlay0: "#727072",
      overlay1: "#848486",
      blue: "#78dce8",
      lavender: "#ab9df2",
      sapphire: "#66d9ef",
      sky: "#78dce8",
      red: "#ff6188",
      maroon: "#ff6188",
      peach: "#fc9867",
      yellow: "#ffd866",
      green: "#a9dc76",
      teal: "#78dce8",
    },
  },
  tokyoNight: {
    name: "tokyoNight",
    label: "Tokyo Night",
    colors: {
      base: "#1a1b26",
      mantle: "#16161e",
      crust: "#13131a",
      text: "#a9b1d6",
      subtext0: "#9aa5ce",
      subtext1: "#9aa5ce",
      surface0: "#232433",
      surface1: "#2a2b3d",
      surface2: "#32344a",
      overlay0: "#565f89",
      overlay1: "#6b7089",
      blue: "#7aa2f7",
      lavender: "#bb9af7",
      sapphire: "#7dcfff",
      sky: "#7dcfff",
      red: "#f7768e",
      maroon: "#ff9e64",
      peach: "#ff9e64",
      yellow: "#e0af68",
      green: "#9ece6a",
      teal: "#2ac3de",
    },
  },
  gruvbox: {
    name: "gruvbox",
    label: "Gruvbox Dark",
    colors: {
      base: "#282828",
      mantle: "#1d2021",
      crust: "#1b1b1b",
      text: "#ebdbb2",
      subtext0: "#a89984",
      subtext1: "#bdae93",
      surface0: "#3c3836",
      surface1: "#504945",
      surface2: "#665c54",
      overlay0: "#7c6f64",
      overlay1: "#928374",
      blue: "#83a598",
      lavender: "#d3869b",
      sapphire: "#83a598",
      sky: "#8ec07c",
      red: "#fb4934",
      maroon: "#cc241d",
      peach: "#fe8019",
      yellow: "#fabd2f",
      green: "#b8bb26",
      teal: "#8ec07c",
    },
  },
  nord: {
    name: "nord",
    label: "Nord",
    colors: {
      base: "#2e3440",
      mantle: "#272c36",
      crust: "#242933",
      text: "#eceff4",
      subtext0: "#d8dee9",
      subtext1: "#e5e9f0",
      surface0: "#3b4252",
      surface1: "#434c5e",
      surface2: "#4c566a",
      overlay0: "#616e88",
      overlay1: "#7b88a1",
      blue: "#88c0d0",
      lavender: "#b48ead",
      sapphire: "#81a1c1",
      sky: "#88c0d0",
      red: "#bf616a",
      maroon: "#d08770",
      peach: "#d08770",
      yellow: "#ebcb8b",
      green: "#a3be8c",
      teal: "#8fbcbb",
    },
  },
  oneDark: {
    name: "oneDark",
    label: "One Dark",
    colors: {
      base: "#282c34",
      mantle: "#21252b",
      crust: "#1b1f23",
      text: "#abb2bf",
      subtext0: "#828997",
      subtext1: "#9da5b4",
      surface0: "#31353f",
      surface1: "#393f4a",
      surface2: "#4b5263",
      overlay0: "#636d83",
      overlay1: "#767d8d",
      blue: "#61afef",
      lavender: "#c678dd",
      sapphire: "#56b6c2",
      sky: "#56b6c2",
      red: "#e06c75",
      maroon: "#be5046",
      peach: "#d19a66",
      yellow: "#e5c07b",
      green: "#98c379",
      teal: "#56b6c2",
    },
  },
  highContrast: {
    name: "highContrast",
    label: "High Contrast",
    colors: {
      base: "#000000", // Pure black background
      mantle: "#0a0a0a", // Slightly lighter black for layering
      crust: "#141414", // Even lighter black for depth
      text: "#ffffff", // Pure white text
      subtext0: "#e0e0e0", // Very light grey for secondary text
      subtext1: "#f0f0f0", // Almost white for important secondary text
      surface0: "#1a1a1a", // Dark surface for contrast
      surface1: "#2a2a2a", // Lighter surface for hover states
      surface2: "#3a3a3a", // Even lighter surface for active states
      overlay0: "#4a4a4a", // Medium grey for overlays
      overlay1: "#5a5a5a", // Lighter grey for overlay hover states
      blue: "#00ffff", // Cyan for primary actions
      lavender: "#ff00ff", // Magenta for accents
      sapphire: "#00ccff", // Bright blue for links
      sky: "#00ffee", // Bright cyan for highlights
      red: "#ff0000", // Pure red for errors/warnings
      maroon: "#ff3333", // Lighter red for secondary warnings
      peach: "#ffaa00", // Bright orange for notifications
      yellow: "#ffff00", // Pure yellow for important highlights
      green: "#00ff00", // Pure green for success states
      teal: "#00ffcc", // Bright teal for special actions
    },
  },
  githubLight: {
    name: "githubLight",
    label: "GitHub Light",
    colors: {
      base: "#ffffff",
      mantle: "#f6f8fa",
      crust: "#eaeef2",
      text: "#24292f",
      subtext0: "#57606a",
      subtext1: "#6e7781",
      surface0: "#f3f6fa",
      surface1: "#eaeef2",
      surface2: "#d0d7de",
      overlay0: "#8c959f",
      overlay1: "#6e7781",
      blue: "#0969da",
      lavender: "#8250df",
      sapphire: "#0550ae",
      sky: "#218bff",
      red: "#cf222e",
      maroon: "#a40e26",
      peach: "#bc4c00",
      yellow: "#9a6700",
      green: "#1a7f37",
      teal: "#0969da",
    },
  },
  solarizedLight: {
    name: "solarizedLight",
    label: "Solarized Light",
    colors: {
      base: "#fdf6e3",
      mantle: "#eee8d5",
      crust: "#e4dcc9",
      text: "#657b83",
      subtext0: "#839496",
      subtext1: "#93a1a1",
      surface0: "#f7f2e4",
      surface1: "#eee8d5",
      surface2: "#dcd4c4",
      overlay0: "#93a1a1",
      overlay1: "#839496",
      blue: "#268bd2",
      lavender: "#6c71c4",
      sapphire: "#2aa198",
      sky: "#2aa198",
      red: "#dc322f",
      maroon: "#cb4b16",
      peach: "#cb4b16",
      yellow: "#b58900",
      green: "#859900",
      teal: "#2aa198",
    },
  },
  oneLight: {
    name: "oneLight",
    label: "One Light",
    colors: {
      base: "#fafafa",
      mantle: "#f0f0f0",
      crust: "#e5e5e5",
      text: "#383a42",
      subtext0: "#4f525e",
      subtext1: "#696c77",
      surface0: "#f2f2f2",
      surface1: "#e5e5e5",
      surface2: "#d4d4d4",
      overlay0: "#a0a1a7",
      overlay1: "#696c77",
      blue: "#4078f2",
      lavender: "#a626a4",
      sapphire: "#0184bc",
      sky: "#0997b3",
      red: "#e45649",
      maroon: "#ca1243",
      peach: "#d75f00",
      yellow: "#c18401",
      green: "#50a14f",
      teal: "#0184bc",
    },
  },
  bubblegum: {
    name: "bubblegum",
    label: "Bubblegum",
    colors: {
      base: "#ff9ac1", // Light pink background
      mantle: "#ffa7cc", // Slightly darker pink
      crust: "#ffb4d8", // Even darker pink for depth
      text: "#2d1c2d", // Dark purple text
      subtext0: "#4b384b", // Medium purple for secondary text
      subtext1: "#5c465c", // Lighter purple for tertiary text
      surface0: "#ffc1e0", // Light pink surface
      surface1: "#ffcee7", // Lighter pink surface
      surface2: "#ffdaf0", // Even lighter pink surface
      overlay0: "#7e5c7e", // Muted purple overlay
      overlay1: "#6e4f6e", // Darker purple overlay
      blue: "#7287fd", // Soft blue
      lavender: "#b4befe", // Soft lavender
      sapphire: "#89dceb", // Soft cyan
      sky: "#89dceb", // Matching cyan
      red: "#ff8089", // Soft red
      maroon: "#ff9999", // Soft maroon
      peach: "#ffb4a1", // Soft peach
      yellow: "#ffe5a0", // Soft yellow
      green: "#a6e3a1", // Soft green
      teal: "#94e2d5", // Soft teal
    },
  },
  rosePine: {
    name: "rosePine",
    label: "Rosé Pine",
    colors: {
      base: "#191724", // Deep purple base
      mantle: "#1f1d2e", // Slightly lighter purple
      crust: "#26233a", // Even lighter purple
      text: "#e0def4", // Soft white text
      subtext0: "#908caa", // Muted purple text
      subtext1: "#6e6a86", // Darker muted text
      surface0: "#2a2837", // Surface purple
      surface1: "#343145", // Lighter surface
      surface2: "#3e3b54", // Even lighter surface
      overlay0: "#524f67", // Overlay purple
      overlay1: "#6e6a86", // Lighter overlay
      blue: "#9ccfd8", // Soft blue
      lavender: "#c4a7e7", // Soft lavender
      sapphire: "#31748f", // Deep blue
      sky: "#9ccfd8", // Light blue
      red: "#eb6f92", // Soft pink
      maroon: "#ebbcba", // Soft rose
      peach: "#f6c177", // Soft peach
      yellow: "#f6c177", // Gold
      green: "#31748f", // Sage
      teal: "#9ccfd8", // Soft teal
    },
  },
  cottonCandy: {
    name: "cottonCandy",
    label: "Cotton Candy",
    colors: {
      base: "#f5d1eb", // Light pink
      mantle: "#f7d7ee", // Slightly darker pink
      crust: "#fae1f3", // Even darker pink
      text: "#2d0c3a", // Deep purple text
      subtext0: "#4a1259", // Medium purple text
      subtext1: "#671878", // Light purple text
      surface0: "#f9def1", // Surface pink
      surface1: "#fde9f5", // Lighter surface
      surface2: "#fff2fa", // Even lighter surface
      overlay0: "#b87dd3", // Purple overlay
      overlay1: "#9c5fb8", // Darker overlay
      blue: "#79c7ff", // Baby blue
      lavender: "#d5a6ff", // Soft purple
      sapphire: "#7cb8ff", // Light blue
      sky: "#89dcff", // Bright blue
      red: "#ff9ed2", // Soft pink
      maroon: "#ff8ac4", // Darker pink
      peach: "#ffb2c7", // Peachy pink
      yellow: "#ffffc2", // Pastel yellow
      green: "#b6ffd7", // Mint green
      teal: "#89ffea", // Turquoise
    },
  },
  synthwave: {
    name: "synthwave",
    label: "Synthwave",
    colors: {
      base: "#2b213a", // Deep purple
      mantle: "#2f2444", // Slightly lighter purple
      crust: "#33274f", // Even lighter purple
      text: "#ff7edb", // Neon pink text
      subtext0: "#e58ee0", // Softer pink text
      subtext1: "#cb9ee6", // Lavender text
      surface0: "#392662", // Surface purple
      surface1: "#443773", // Lighter surface
      surface2: "#504785", // Even lighter surface
      overlay0: "#625997", // Purple overlay
      overlay1: "#7267aa", // Lighter overlay
      blue: "#36f9f6", // Cyan
      lavender: "#ff7edb", // Pink
      sapphire: "#72f1b8", // Mint
      sky: "#36f9f6", // Bright cyan
      red: "#fe4450", // Hot red
      maroon: "#ff558f", // Hot pink
      peach: "#ff8b39", // Orange
      yellow: "#fede5d", // Yellow
      green: "#72f1b8", // Neon green
      teal: "#36f9f6", // Bright teal
    },
  },
  candyfloss: {
    name: "candyfloss",
    label: "Candyfloss",
    colors: {
      base: "#f8e2ff", // Light purple
      mantle: "#ffe2f8", // Pink tint
      crust: "#ffe9f3", // Lighter pink
      text: "#5c1b99", // Deep purple text
      subtext0: "#7a3aaf", // Medium purple text
      subtext1: "#944bc6", // Light purple text
      surface0: "#ffeaf8", // Surface pink
      surface1: "#fff2fb", // Lighter surface
      surface2: "#fff7fd", // Even lighter surface
      overlay0: "#d59bff", // Purple overlay
      overlay1: "#c77dff", // Darker overlay
      blue: "#79baff", // Soft blue
      lavender: "#cc8fff", // Light purple
      sapphire: "#85a5ff", // Periwinkle
      sky: "#8aceff", // Light blue
      red: "#ff8fab", // Soft red
      maroon: "#ff7fa6", // Pink
      peach: "#ffb2c7", // Peach
      yellow: "#fff3b2", // Soft yellow
      green: "#b8ffda", // Mint
      teal: "#8affef", // Aqua
    },
  },
  terminal: {
    name: "terminal",
    label: "Green Terminal",
    colors: {
      base: "#0D1117", // Deep black background
      mantle: "#161B22", // Slightly lighter black
      crust: "#1B2127", // Terminal border color
      text: "#00FF00", // Classic terminal green
      subtext0: "#00D700", // Dimmer green
      subtext1: "#00BB00", // Even dimmer green
      surface0: "#1C2128", // Slightly lifted surface
      surface1: "#21262D", // Terminal input area
      surface2: "#282E35", // Selected area
      overlay0: "#008800", // Darker green for overlays
      overlay1: "#006600", // Even darker green
      blue: "#00FF00", // Keep everything in green shades
      lavender: "#00FF66", // Slight variation
      sapphire: "#00DD88", // Another variation
      sky: "#00FFBB", // Lighter green
      red: "#FF0000", // Error red (keep for errors)
      maroon: "#AA0000", // Darker error
      peach: "#00FF99", // Another green variation
      yellow: "#FFFF00", // Warning yellow (keep for warnings)
      green: "#00FF00", // Main green
      teal: "#00FFCC", // Cyan-ish green
    },
  },
  amber: {
    name: "amber",
    label: "Amber Terminal",
    colors: {
      base: "#0D0904", // Deep black with amber tint
      mantle: "#160E06", // Slightly lighter black
      crust: "#1B1109", // Terminal border color
      text: "#FFB000", // Classic amber
      subtext0: "#CC8800", // Dimmer amber
      subtext1: "#995500", // Even dimmer amber
      surface0: "#1C1409", // Slightly lifted surface
      surface1: "#211909", // Terminal input area
      surface2: "#281E0A", // Selected area
      overlay0: "#663300", // Darker amber for overlays
      overlay1: "#442200", // Even darker amber
      blue: "#FFB000", // Keep everything in amber shades
      lavender: "#FFAA00", // Slight variation
      sapphire: "#FF9500", // Another variation
      sky: "#FFB000", // Main amber
      red: "#FF3300", // Error red (keep for errors)
      maroon: "#CC3300", // Darker error
      peach: "#FFAA55", // Lighter amber
      yellow: "#FFDD00", // Warning yellow
      green: "#FFB000", // Main amber
      teal: "#FFC000", // Lighter amber
    },
  },
  ibmPC: {
    name: "ibmPC",
    label: "IBM PC",
    colors: {
      base: "#000000", // Classic black background
      mantle: "#0A0A0A", // Slightly lighter black
      crust: "#141414", // Border color
      text: "#AAAAAA", // Light gray text
      subtext0: "#888888", // Dimmer text
      subtext1: "#666666", // Even dimmer text
      surface0: "#1C1C1C", // Slightly lifted surface
      surface1: "#212121", // Input area
      surface2: "#282828", // Selected area
      overlay0: "#444444", // Overlay
      overlay1: "#333333", // Darker overlay
      blue: "#5555FF", // CGA blue
      lavender: "#FF55FF", // CGA magenta
      sapphire: "#5555FF", // Another blue
      sky: "#55FFFF", // CGA cyan
      red: "#FF5555", // CGA red
      maroon: "#AA0000", // Darker red
      peach: "#FF5555", // Another red shade
      yellow: "#FFFF55", // CGA yellow
      green: "#55FF55", // CGA green
      teal: "#55FFFF", // Another cyan
    },
  },
};
