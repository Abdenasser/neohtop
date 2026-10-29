# NeoHtop: open-source desktop system monitor

NeoHtop is a free, MIT-licensed system monitor for macOS, Linux, and Windows, built with Rust, Tauri, Svelte, and shadcn-svelte. Local monitoring requires no account or subscription.

## What’s new in v1.3.0

- Refreshed desktop controls with all 24 themes, light/dark choices and native process icons.
- Process families with combined CPU/RAM, search, pinning, filters and configurable columns.
- On-demand local-port search using `:3000` or `port:443`; bare numbers still match PIDs.
- CPU, memory, GPU, storage, system and network cards with detailed sheets. CPU, memory, GPU and network include two-minute in-memory histories.
- A menu-bar/system-tray companion with shared pause, interval, theme and process shortcuts.
- Separate Settings and About windows; automatic preference migration and saving.
- Driver-dependent GPU utilization, memory, sensors, power and core clock. Optional per-process GPU engine activity is supported through Linux DRM counters.
- Linux x86_64/ARM64 packages, Intel/Apple Silicon DMGs and a Windows x64 installer, with release checksums and a build manifest.

GPU support varies by driver, and operating-system permissions can restrict process/socket details. Missing counters remain unavailable. Family CPU can exceed 100% across cores, and family RAM may double-count shared pages. The app does not provide cleanup, security scanning, a firewall, cloud history or AI diagnostics.

[Read the desktop guide](https://github.com/Abdenasser/neohtop/blob/main/docs/usage.md) or [changelog](https://github.com/Abdenasser/neohtop/blob/main/CHANGELOG.md). Resource use depends on OS, process count, refresh cadence and visible data; there is no universal CPU/memory benchmark.

## Downloads

[Published GitHub releases](https://github.com/Abdenasser/neohtop/releases/latest) are authoritative for stable binaries and architectures. Older versions do not include all v1.3.0 features and may lack ARM64 Linux packages or a Windows installer. Community packages can update on a separate schedule.

## NeoHtop Pro

Pro is postponed and is not available to purchase or install. The current release focuses on the free, open-source local desktop monitor. No Pro scope, price or launch date is announced.

## NeoHtop CLI

The CLI is a separate Go/Charm terminal application: `npm install -g neohtop-cli`. It does not install the desktop app. [Compare Desktop and CLI](https://abdenasser.github.io/neohtop/desktop-vs-cli.html).

## Links

- [Home](https://abdenasser.github.io/neohtop/)
- [Install and troubleshoot](https://abdenasser.github.io/neohtop/install.html)
- [Desktop source](https://github.com/Abdenasser/neohtop)
- [Releases](https://github.com/Abdenasser/neohtop/releases/latest)
- [CLI source](https://github.com/Abdenasser/neohtop-cli)
- [Issues](https://github.com/Abdenasser/neohtop/issues)
- [License](https://github.com/Abdenasser/neohtop/blob/main/LICENSE)
