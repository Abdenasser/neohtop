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
    <source media="(prefers-color-scheme: dark)" srcset="screenshot.png">
    <source media="(prefers-color-scheme: light)" srcset="screenshot-light.png">
    <img alt="NeoHtop Screenshot" src="./screenshot.png" width="800">
  </picture>
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
- [Support](#support)
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
- 📂 Open the directory of the process
- 🎯 Sort by any column
- 🔄 Auto-refresh system stats

### Search Functionality
Search for processes by name, command, or PID. Use commas to search for multiple terms simultaneously. Regular expressions are supported for advanced filtering.

Examples:
- `arm, x86`: Returns processes with "arm" or "x86" in the name or command
- `d$`: Lists daemons (processes ending with 'd')
- `^(\w+\.)+\w+$`: Shows processes with reverse domain name notation (e.g., com.docker.vmnetd)

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
Download the latest release from the [releases page](https://github.com/Abdenasser/neohtop/releases).

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

## Support
If you find this project helpful, consider buying me a coffee:

<a href="https://www.buymeacoffee.com/abdenasser" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 60px !important;width: 217px !important;" ></a>

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
