
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
    <img alt="NeoHtop Screenshot" src="./screenshot.png" width="800">
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
- 🚀 Real-time process monitoring
- 💻 CPU and Memory usage tracking
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
- `:3000` or `port:443`: Processes listening on that local port (TCP listen or UDP). A bare number still matches PID. Privileged sockets may require launching with sudo / pkexec.

## Tech Stack
- **Frontend**: SvelteKit, TypeScript
- **Backend**: Rust, Tauri
- **Styling**: CSS Variables for theming
- **Icons**: FontAwesome

## Getting Started

### Prerequisites
- Node.js (v16 or later)
- Rust (latest stable)
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
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

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

## Contributing
We welcome contributions! Please see our [contributing guidelines](./.github/CONTRIBUTING.md) for more information.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
