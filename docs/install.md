# Install NeoHtop Desktop

Official desktop binaries are on the [latest published GitHub release](https://github.com/Abdenasser/neohtop/releases/latest). Choose the package for your platform and CPU architecture. Development features may not be in the current published app. NeoHtop Pro is not available yet.

## macOS

Choose the Apple Silicon DMG for an M-series Mac, or the Intel DMG for an Intel-based Mac. Check Apple menu → About This Mac for the chip or processor. Open the DMG, drag NeoHtop to Applications, and launch it.

Community alternative: `brew install --cask neohtop`. The Homebrew cask is maintained separately and may update on a different schedule. If macOS cannot verify a download, verify its source and consult release notes. Do not disable Gatekeeper as a routine installation step.

## Windows

Download the x64 .exe from the published release. A filename ending in `-setup.exe` indicates an installer; run it and follow the prompts. Older releases may provide a portable .exe, which you can keep and run in a folder you control. Consult release notes for requirements.

## Linux

Run `uname -m`: `x86_64` means x64 and `aarch64` means ARM64. Older releases may not provide both architectures.

- Debian/Ubuntu: choose .deb and use the distribution's package installer.
- Fedora or compatible RPM distributions: choose .rpm and install it with the distribution's package manager.
- AppImage: make the downloaded file executable in its Properties and open it. System libraries and possibly FUSE are still required; an AppImage is not compatible with every Linux distribution.

## Troubleshooting

For glibc errors or a blank Linux window, check your distribution version, package type, and release notes. An older Ubuntu build baseline and a bundled-Wayland-library AppImage check are development work for a future desktop release; they may not be in existing downloads. An appropriate native package may be another option.

Operating-system permissions can restrict process details. Start without administrator privileges and consult platform-specific repository guidance before granting elevated access.

[Report issues](https://github.com/Abdenasser/neohtop/issues) with app version, operating system, CPU architecture, package type, error text, and reproduction steps. Redact credentials and private command-line arguments.

## Build the development version

Use Node.js 22, stable Rust, and the relevant Tauri platform dependencies. See the [development documentation](https://github.com/Abdenasser/neohtop#development).

```sh
git clone https://github.com/Abdenasser/neohtop.git
cd neohtop
npm ci
npm run tauri dev
```

[HTML installation guide](https://abdenasser.github.io/neohtop/install.html)
