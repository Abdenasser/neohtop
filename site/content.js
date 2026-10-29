export const faq = [
  [
    "Does NeoHtop send analytics?",
    "Official v1.3.0 builds send a basic app-open count and version without a persistent identifier. Optional feature usage is off by default in Settings → Privacy. Process and system readings stay local. See the Privacy link for details; website analytics is separate.",
  ],
  [
    "Is NeoHtop free? Do I need an account?",
    "NeoHtop is free, MIT-licensed and open source. Local desktop monitoring requires no account or cloud subscription. Pro is postponed and is not available to buy or install.",
  ],
  [
    "Will it read my graphics card?",
    "GPU support depends on your operating system and driver. NeoHtop reads macOS IORegistry, AMD sysfs and NVIDIA NVML, with DXGI/DRM inventory for other adapters. Missing counters stay unavailable. Per-process GPU engine accounting is limited to supported Linux DRM drivers.",
  ],
  [
    "Are all 24 themes still here?",
    "Yes. The original themes remain, including Catppuccin, Dracula, Nord, Tokyo Night, light palettes, retro choices and High Contrast. Glassy needs supported native effects on macOS or Windows.",
  ],
  [
    "What does the menu-bar companion do?",
    "It shows CPU, RAM, GPU, network, free storage and the five busiest processes. It shares samples, pause and themes with the main window, and stops polling while hidden. Tray behavior varies by Linux desktop.",
  ],
  [
    "Does this include process trees and local-port search?",
    "Version 1.3.0 includes expandable families with combined CPU/RAM and explicit :3000 or port:443 searches. Family RAM may double-count shared pages, and socket visibility depends on OS permissions. Older releases can differ.",
  ],
  [
    "How is it different from NeoHtop CLI?",
    "Desktop is the Rust/Tauri/Svelte graphical app. NeoHtop CLI is a separate Go/Charm terminal application. Both are local, open-source monitors with independent feature sets and releases.",
  ],
];
