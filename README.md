
<div align="center">
  <img src="app-icon.png" alt="NeoHtop Logo" width="120" />
  <h1>NeoHtop</h1>
  <p>A modern, cross-platform system monitor built on top of Svelte, Rust, and Tauri.</p>

  [![License](https://img.shields.io/github/license/Abdenasser/neohtop)](https://github.com/Abdenasser/neohtop/blob/main/LICENSE)
  [![GitHub stars](https://img.shields.io/github/stars/Abdenasser/neohtop)](https://github.com/Abdenasser/neohtop/stargazers)
  [![GitHub issues](https://img.shields.io/github/issues/Abdenasser/neohtop)](https://github.com/Abdenasser/neohtop/issues)
  [![GitHub release](https://img.shields.io/github/v/release/Abdenasser/neohtop)](https://github.com/Abdenasser/neohtop/releases)
  [![Notarized by Apple](https://img.shields.io/badge/Release_Notarized_by_Apple-000000?style=flat-square&logo=apple&logoColor=white)](https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution)
</div>

<div align="center">
  <picture>
    <!-- <source media="(prefers-color-scheme: dark)" srcset="screenshot.png">
    <source media="(prefers-color-scheme: light)" srcset="screenshot-light.png"> -->
    <img alt="NeoHtop Screenshot" src="./docs/assets/neohtop-desktop.png" width="800">
  </picture>
</div>

<div align="center">
  <p>If you find this project helpful, consider buying me a coffee:</p>
  <a href="https://www.buymeacoffee.com/abdenasser" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 60px !important;width: 217px !important;" ></a>
  <p>Or sponsor me on GitHub:</p>
  <a href="https://github.com/sponsors/Abdenasser" target="_blank"><img src="https://img.shields.io/badge/Sponsor-abdenasser-white?style=flat&logo=github&logoColor=pink" alt="Sponsor @abdenasser" style="height: auto !important;width: 217px !important;"></a>

</div>

<br>

<div align="center">
  <table>
    <tr>
      <td align="center">
        <h2>👻 NEW: <a href="https://github.com/Abdenasser/neohtop-cli">NeoHtop CLI</a> is here!</h2>
        <p><strong>All the power of NeoHtop, right in your terminal</strong> — real-time CPU sparklines, memory bars, process tree view, search, filters, themes & more.</p>
        <p>Built with <b>Go</b> + the <b>Charm</b> ecosystem. Works everywhere — macOS, Linux, Windows.</p>
        <br>
        <a href="https://github.com/Abdenasser/neohtop-cli"><img src="https://img.shields.io/badge/⭐_Star_NeoHtop_CLI-171717?style=for-the-badge&logo=github&logoColor=white" alt="Star NeoHtop CLI"></a>&nbsp;
        <a href="https://www.npmjs.com/package/neohtop-cli"><img src="https://img.shields.io/npm/v/neohtop-cli?style=for-the-badge&logo=npm&logoColor=white&label=npm&color=CB3837" alt="npm"></a>
        <br><br>
        <pre>npm install -g neohtop-cli</pre>
      </td>
    </tr>
  </table>
</div>

## Table of Contents
- [Why NeoHtop?](#why-neohtop)
- [Features](#features)
- [Tech Stack](#tech-stack)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Running with Sudo](#running-with-sudo)
- [Development](#development)
  - [Setup](#setup)
  - [Code Formatting](#code-formatting)
  - [Pull Requests](#pull-requests)
- [Contributing](#contributing)
- [License](#license)

## Why NeoHtop?
[Read about the back story and motivation behind NeoHtop](https://www.abdenasser.com/2024/11/06/oh-boy-neohtop/)

## Features

**[What’s new in v1.3.0](CHANGELOG.md)** · [Desktop guide](docs/usage.md) · [Downloads](https://github.com/Abdenasser/neohtop/releases/latest)

- 🚀 Real-time process monitoring
- 🌳 Expandable process trees with family CPU/RAM totals and search that keeps matching processes in context
- 💻 CPU and Memory usage tracking
- 🎮 GPU utilization, memory and adapter details (driver-dependent)
- 👻 Menu-bar/system-tray overview with shared pause and process shortcuts
- 🎨 Beautiful, modern UI with dark/light themes
- 🔍 Advanced process search and filtering
- 📌 Pin important processes
- 🛠 Process management (kill processes)
- 🎯 Sort by any column
- 🔄 Auto-refresh system stats

### Search Functionality
Search for processes by name, command, PID, or port. Use commas to search for multiple terms simultaneously. Regular expressions are supported for advanced filtering.

Examples:
- `arm, x86`: Returns processes with "arm" or "x86" in the name or command
- `d$`: Lists daemons (processes ending with 'd')
- `^(\w+\.)+\w+$`: Shows processes with reverse domain name notation (e.g., com.docker.vmnetd)
- `:3000` or `port:443`: Processes listening on that local port (TCP listen or UDP). A bare number still matches PID. Socket visibility depends on operating-system permissions; NeoHtop does not request elevation automatically.

### Process families

Switch from **List** to **Tree** above the process table to expand parent/child
relationships. Tree mode shows combined CPU usage and resident memory for each
process family; CPU can exceed 100% across multiple cores, and shared memory can
be counted more than once in the family RAM sum. List mode keeps individual
process metrics.

Search and filters match individual processes. Tree mode retains their parents
and expands matching branches automatically, while family totals continue to
include all descendants. Clear the search and filters to collapse those branches.
Pages keep expanded families together, and your choice of List or Tree is saved.

### GPU readings and menu-bar overview

The GPU card includes a two-minute utilization history and a **Graphics** details
panel. On multi-GPU machines, select an adapter in that panel. Readings are local
and read-only; unavailable driver counters stay unavailable instead of showing
zero.

| Platform | GPU support |
| --- | --- |
| macOS | IORegistry driver utilization; Apple silicon shared-memory usage and core count where exposed. Shared memory is part of RAM, not extra VRAM. |
| Linux | NVIDIA utilization, VRAM and temperature through the installed NVML driver; AMD utilization/VRAM/temperature through amdgpu sysfs; other DRM adapters provide inventory. |
| Windows | NVIDIA utilization, VRAM and temperature through NVML; other DXGI adapters provide inventory and reported dedicated capacity. |

Support varies by GPU and driver. Missing/denied counters do not require running
NeoHtop as administrator. There is no bundled GPU driver or monitoring agent.

Click the ghost in the menu bar/system tray, or the **overview icon** in the app bar, for
CPU, RAM, GPU, network, storage and the five busiest processes. Selecting a process
opens its inspector in the main window. Both windows share the sampling interval,
pause state and themes. The overview stops polling while hidden.

**Settings → General** controls the refresh interval, menu-bar icon and optional
monitoring after the main window closes. Open Settings from the app menu or with
**⌘,** on macOS (**Ctrl+,** elsewhere). **Settings → Appearance** and the app bar's
palette button share all 24 themes, including searchable dark and light choices. By default, closing the window quits the app.
**Quit NeoHtop** always exits. On Linux, tray click handling depends on the desktop;
use the icon's **Show overview** menu item or the app-bar button when needed.

GPU engine activity per process is optional: enable **GPU %** in **View** or a GPU
filter. It is available on Linux drivers exposing DRM fdinfo counters; other
platforms and unreadable counters show a dash. Several engines can exceed 100%;
this is different from the device's normalized utilization. The first sample
establishes a baseline. Power and core clock are shown in Graphics details only
when the installed AMD/NVIDIA driver exposes them.

## Tech Stack
- **Frontend**: SvelteKit, TypeScript
- **Backend**: Rust, Tauri
- **Styling**: shadcn-svelte, Tailwind CSS 4, and semantic theme tokens
- **Icons**: Lucide controls, native process icons, and the ghost fallback

## Getting Started

### Prerequisites
- Node.js 22 (see `.nvmrc`)
- Rust 1.88 or newer (latest stable recommended)
- Xcode Command Line Tools (for macOS)

### Installation

#### Manual
Download the latest release from the [releases page](https://github.com/Abdenasser/neohtop/releases).

#### Package Managers

Members of the community have kindly published unofficial packages for various platforms and package managers.

Please note, these packages are community-maintained and not officially released, reviewed, or endorsed by NeoHtop.
We only provide official builds through the GitHub Releases page.
Since these external packages are managed by third parties, we cannot guarantee their security, integrity, or update frequency.

Please use them at your own discretion.

##### macOS

Using [Homebrew](https://brew.sh/).

```bash
brew install --cask neohtop
```

##### Arch Linux (AUR)

Using the [AUR](https://aur.archlinux.org/) and [an AUR helper](https://wiki.archlinux.org/title/AUR_helpers).

```bash
yay -S neohtop
```

or

```bash
paru -S neohtop
```

##### Fedora Linux
Install the [Terra repository](https://terra.fyralabs.com/).

```bash
dnf install neohtop
```

##### Windows
Install the [Scoop repository](https://scoop.sh/), then make sure you have the Scoop extras bucket added:

```bash
scoop bucket add extras
```

Then install with:

```bash
scoop install extras/neohtop
```

##### Solus

```bash
eopkg install neohtop
```

### Running with Sudo
Some processes require monitoring with sudo privileges. To monitor these processes, launch NeoHtop with sudo:

- macOS: `sudo /Applications/NeoHtop.app/Contents/MacOS/NeoHtop`
- Linux: `pkexec /path/to/neohtop` (recommended)

## Development

### Setup
```bash
# Install dependencies
npm ci

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Build checks and releases

Pull requests and pushes to `main` run formatting, type checks, release-tool tests,
and package builds for Windows x64, macOS Intel/Apple Silicon, and Linux x64/ARM64.
Linux packages are built on Ubuntu 22.04; newer distributions still need the
appropriate WebKitGTK runtime. AppImages are checked for bundled Wayland client
libraries to prevent the display-driver conflict reported in issue #203.

To build preview packages, run **Build Check** or **Draft Release** from GitHub
Actions on a branch. Download the `packages-*` workflow artifacts. Preview macOS
packages are ad-hoc signed and are not notarized for public distribution.

For a release candidate:

1. Update the version in `package.json`, both root version entries in `package-lock.json`,
   `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, and the NeoHtop entry in
   `src-tauri/Cargo.lock`. Run `npm run release:check` and commit the changes.
2. Push a matching version tag, for example `v1.3.0`. All platform packages are
   built from that tag. A published version cannot be overwritten.
3. After every build passes, **Draft Release** verifies all nine packages and
   attaches `SHA256SUMS` and `release-manifest.json` to a draft GitHub release.
4. Install and launch the packages on their supported operating systems, review
   the release notes, then publish the draft from GitHub when ready.

Tagged macOS builds require the existing repository secrets `MACOS_CERTIFICATE`
(base64-encoded Developer ID `.p12`), `MACOS_CERTIFICATE_PASSWORD`, `APPLE_ID`,
`APPLE_PASSWORD` (an app-specific password), and `APPLE_TEAM_ID`. The optional
repository variable `APPLE_SIGNING_IDENTITY` overrides the current Developer ID
identity. Tagged builds fail if credentials are missing or signature/notarization
checks fail. Preview builds and pull requests do not need signing credentials.
Windows packages are NSIS installers, named `NeoHtop_<version>_x64-setup.exe`;
Windows code signing is not configured by this workflow.

The release uses the scoped `GITHUB_TOKEN`; `PAT_TOKEN` is no longer needed.
Existing branch-protection rules that require the old **Format Check** job should
be updated to require **Source checks** and the platform package jobs.

### Code Formatting
We use Prettier for web code and `cargo fmt` for Rust code.

```bash
# Format all files
npm run format

# Check formatting without making changes
npm run format:check
```

### Pull Requests
Before submitting a PR, ensure:
1. All code is formatted (`npm run format`)
2. The format check passes (`npm run format:check`)
3. Your commits follow the project's commit message conventions

### Website and discoverability

GitHub Pages serves the static `docs/` directory from `main`. Preview it over
HTTP so browser modules and copy controls work:

```bash
python3 -m http.server 8765 --bind 127.0.0.1 --directory docs
```

Open `http://127.0.0.1:8765/`. The HTML contains the product content, navigation,
FAQ answers, and release-page download links without requiring JavaScript.
JavaScript resolves only assets from the latest published stable release; API
failures retain working release-page links. Version 1.3.0 brings process families, GPU details, native app icons, resource
histories, local-port search, the menu-bar companion and Settings to the free
open-source desktop app. Pro is postponed and is not available to buy or install.

Each public HTML page has a canonical URL, social metadata, structured data,
and a linked Markdown equivalent. Keep the HTML, Markdown, `llms.txt`, and
`sitemap.xml` consistent when changing product facts. The site uses local assets,
system fonts, native FAQ disclosures, and reduced-motion styles. Existing Google
Analytics remains optional; blocking it must not affect downloads or navigation.

`llms.txt` follows a [proposed convention](https://llmstxt.org/), not a search
ranking guarantee. [Google's AI search guidance](https://developers.google.com/search/docs/appearance/ai-features)
prioritizes the same useful, crawlable content as ordinary SEO. Submit
`https://abdenasser.github.io/neohtop/sitemap.xml` in the site's verified Search
Console property after deployment and monitor indexing there. Ownership
verification and sitemap submission are account-side actions, not completed by
this repository.

Robots rules apply at `https://abdenasser.github.io/robots.txt`, the host root.
A `docs/robots.txt` would be served below `/neohtop/` and would not control
crawlers. No blocking host-root robots file was present when the site was
refreshed. Any future host-root rules must keep the public site crawlable; do not
change unrelated host-wide policy from this project. The sitemap is linked from
the footer even without a host-root robots declaration.

Run `npm run test:release` to check internal links, metadata, structured data,
Markdown references, sitemap coverage, and static download fallbacks alongside
the release and process-tree tests.

## Contributing
We welcome contributions! Please see our [contributing guidelines](./.github/CONTRIBUTING.md) for more information.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


## UI components and themes

NeoHtop uses [shadcn-svelte](https://www.shadcn-svelte.com/) components with Tailwind CSS. Shared primitives live in `src/lib/components/ui`; `components.json` configures the registry. Keep controls composed from that library and use semantic tokens rather than introducing independent widget styles. All existing themes, including supported native Glassy mode, remain available through the theme picker.

See [the migration and validation notes](SHADCN_MIGRATION.md) for feature parity, theme mappings, actual native checks and remaining cross-platform release validation.
