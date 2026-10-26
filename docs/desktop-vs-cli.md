# NeoHtop Desktop vs NeoHtop CLI

These are separate MIT-licensed open-source applications. Both support macOS, Linux, and Windows and monitor local processes. The desktop app does not require the CLI, and the CLI install does not install the desktop app.

| Aspect        | NeoHtop Desktop                                     | NeoHtop CLI                                                      |
| ------------- | --------------------------------------------------- | ---------------------------------------------------------------- |
| Interface     | Graphical desktop window                            | Terminal user interface                                          |
| Technology    | Rust, Tauri, Svelte                                 | Go and the Charm ecosystem                                       |
| Installation  | GitHub release packages; community package managers | npm install -g neohtop-cli or CLI release binaries               |
| Monitoring    | Process metrics, CPU, memory, storage, network I/O  | Process monitoring with CPU sparklines and memory visualizations |
| Process trees | In development for the next desktop release         | Documented in the CLI project                                    |
| License       | MIT                                                 | MIT                                                              |

Pick Desktop for a visual process table, search, filters, pinning, and themes. Pick CLI for keyboard-driven monitoring inside your shell. Feature sets and releases differ; do not assume parity.

NeoHtop Pro is in development for possible remote monitoring, history, and alerts. Scope, price, and launch date are not announced. Neither application currently installs a Pro subscription.

- [HTML comparison](https://abdenasser.github.io/neohtop/desktop-vs-cli.html)
- [Desktop repository](https://github.com/Abdenasser/neohtop)
- [Desktop releases](https://github.com/Abdenasser/neohtop/releases/latest)
- [CLI repository](https://github.com/Abdenasser/neohtop-cli)
- [CLI npm package](https://www.npmjs.com/package/neohtop-cli)
