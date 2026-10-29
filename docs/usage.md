# Using NeoHtop v1.3.0

NeoHtop is a free, open-source local system monitor for macOS, Linux, and Windows. No account, cloud subscription, or background agent is required. Pro is postponed and is not available.

## Find the process you need

Use **⌘F** (macOS) or **Ctrl+F** to focus search. Search by name, command, or PID; use commas to match any term and regular expressions for names. Try `node, python`, `^kernel`, or `d$`. A bare number matches a PID. Use `:3000` or `port:443` to find a local TCP listener or a bound UDP socket. Sockets are collected only while an explicit port query is active; permissions can hide other users’ sockets. Resume monitoring before searching ports.

**Filters** combines CPU, GPU engine activity, RAM, runtime and status conditions. Active conditions and matching counts appear below the toolbar; **Clear all** restores the complete list. GPU filters exclude unavailable readings rather than treating them as zero.

Choose **List** for individual processes or **Tree** for families. Tree search retains parents and expands matching branches; family CPU and RAM continue to include hidden descendants. CPU may exceed 100% across cores, and shared pages can overlap in summed family RAM. Expanded families stay together when paging.

Pin a process with the pin action, inspect it with the information action, or use the end action to request termination after confirmation. **View** controls columns; required process names stay visible. Page size and pagination remain below the table.

## Explore resources

Use each card’s arrow to open a detailed sheet. CPU, memory, GPU and network histories cover roughly two minutes in memory. They are cleared on restart and are not long-term monitoring storage. Inspect charts with the pointer or keyboard arrows. Gaps indicate paused, missing or delayed readings.

- **CPU:** average utilization, individual logical cores, and hardware identity. Process CPU can exceed the 100% scale of a single core.
- **Memory:** used RAM, actual free pages, swap and breakdowns exposed by the OS. “Not in use” on the overview means total minus used and differs from strictly free pages.
- **Graphics:** select an adapter and inspect utilization, memory, temperature, core count, power and core clock where the driver exposes them. Apple silicon shared GPU memory is already part of system RAM.
- **Storage:** capacity and mounted volumes. Unknown/zero capacity stays unavailable. This is capacity monitoring, not a cleaner or duplicate finder.
- **System:** OS/build, CPU architecture, uptime and load averages. Load average is not CPU percentage.
- **Network:** received/sent rates, totals and interface details. Search interfaces or include inactive interfaces. New/reset counters stay unavailable until a valid rate baseline exists. This is network I/O monitoring, not a firewall or packet inspector.

GPU support is driver dependent: macOS IORegistry statistics; Linux AMD sysfs and NVIDIA NVML; Windows NVIDIA NVML with DXGI inventory for other adapters. Missing counters never imply idle hardware. The optional **GPU %** process column and filter use Linux DRM engine accounting. They need two samples and may exceed 100% across engines; unsupported or unreadable clients remain unavailable.

## Menu-bar companion

Open the ghost icon in the menu bar/system tray or the app-bar overview button for CPU, RAM, GPU, network, free storage and the five busiest processes. Select a process to open its inspector in the main app. The companion and main window share a sample, interval, theme and pause state. The companion stops polling while hidden.

Linux tray behavior varies with the desktop; use **Show overview** in the tray menu or the app-bar button if clicking the icon does not open it.

## Settings and appearance

Open **Settings** from the application menu or **⌘, / Ctrl+,**. General holds the refresh interval, tray icon and keep-running choice. Changes are saved automatically in the app configuration directory and synchronize across windows. Existing local preferences are migrated; save failures are shown with a retry action.

The default is to quit when the main window closes. Enable **Keep running when the window closes** to continue through the tray; this requires the tray icon. **Quit NeoHtop** always exits. Closing Settings or About leaves monitoring running.

Appearance and the app-bar palette chooser share all 24 original themes. Search by name; light/dark palettes also update native title bars. Glassy requires supported native window effects on macOS or Windows.

## Downloads and support

Use the [official release](https://github.com/Abdenasser/neohtop/releases/latest) and [installation guide](install.md). Include OS, architecture, app version, package type and reproduction steps in [bug reports](https://github.com/Abdenasser/neohtop/issues). Redact private command lines, environment variables and identifiers before sharing diagnostics or screenshots.
