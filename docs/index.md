# NeoHtop: open-source desktop system monitor

NeoHtop is a free, MIT-licensed graphical system monitor for macOS, Linux, and Windows. The desktop app is built with Rust, Tauri, and Svelte. No account is required for local monitoring.

## Desktop capabilities

- Local process metrics and detailed process information.
- CPU, memory, storage, and network I/O panels.
- Process search by name, command, and PID; comma-separated terms and regular expressions.
- Filters for CPU, memory, status, and runtime.
- Process pinning, configurable columns, refresh controls, and themes.

Resource consumption depends on the operating system, process count, refresh interval, and visible data. There is no universal published CPU or memory benchmark.

## Published release versus development

[The latest published GitHub release](https://github.com/Abdenasser/neohtop/releases/latest) is the source for stable downloads and available architectures. Download links must match actual release assets; some older releases lack ARM64 Linux packages or offer a portable Windows executable rather than an installer.

Desktop process trees with combined family CPU/RAM are development work for a future release. Tree CPU sums all descendant processes and may exceed 100% across cores; family RAM sums reported resident memory and may double-count shared pages. Linux packaging and build compatibility improvements are also intended for a future release. These changes may not be available in published downloads yet.

## NeoHtop Pro

Pro is in development. Remote monitoring, historical metrics, and alerts are being explored for developers and small teams. Scope, pricing, and launch date are not final. There is no Pro product to purchase or install yet. The free open-source local monitor remains the foundation.

## NeoHtop CLI

The terminal companion is a separate Go/Charm application, not the desktop app. Its documentation lists sparklines, process trees, filters, and themes. Install it with `npm install -g neohtop-cli`. See the [Desktop vs CLI comparison](https://abdenasser.github.io/neohtop/desktop-vs-cli.html).

## Links

- [Home](https://abdenasser.github.io/neohtop/)
- [Install and troubleshoot](https://abdenasser.github.io/neohtop/install.html)
- [Desktop source](https://github.com/Abdenasser/neohtop)
- [Published releases](https://github.com/Abdenasser/neohtop/releases/latest)
- [CLI source](https://github.com/Abdenasser/neohtop-cli)
- [Issues](https://github.com/Abdenasser/neohtop/issues)
- [License](https://github.com/Abdenasser/neohtop/blob/main/LICENSE)
