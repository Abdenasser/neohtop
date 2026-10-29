<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import {
    Cpu,
    MemoryStick,
    Layers,
    Network,
    Monitor,
    GitBranch,
    Search,
    ArrowDown,
    ArrowUpRight,
    Check,
    Palette,
    Settings2,
    Activity,
    HardDrive,
  } from "@lucide/svelte";
  import { faq } from "./content.js";
  export let page = "home";
  const repo = "https://github.com/Abdenasser/neohtop";
  const release = `${repo}/releases/latest`;
  const features = [
    {
      icon: GitBranch,
      color: "var(--chart-1)",
      title: "Find the whole story.",
      text: "Expand process families, follow their combined CPU and RAM, and keep matching children in context as you search.",
      detail: "Process trees, search, filters & pinning",
    },
    {
      icon: Activity,
      color: "var(--chart-2)",
      title: "Get to know your GPU.",
      text: "See utilization, memory and the diagnostics your graphics driver exposes. Choose an adapter and go deeper.",
      detail: "Apple silicon, AMD & NVIDIA · driver dependent",
    },
    {
      icon: Monitor,
      color: "var(--chart-4)",
      title: "A little window. A lot inside.",
      text: "Your essential readings and busiest processes, a click away in the menu bar. Jump straight from a process to its inspector.",
      detail: "One sample, pause state & theme across windows",
    },
    {
      icon: Search,
      color: "var(--chart-3)",
      title: "Who’s holding that port?",
      text: "Type :3000 or port:443 to find a local TCP listener or bound UDP socket. Socket scans run only when you ask.",
      detail: "Keep bare numbers for process IDs",
    },
    {
      icon: Layers,
      color: "var(--chart-5)",
      title: "Every card opens a door.",
      text: "CPU cores, memory breakdowns, graphics adapters, mounted volumes and network interfaces. Inspect the reading behind the number.",
      detail: "Six resource views, with useful details",
    },
    {
      icon: Settings2,
      color: "var(--chart-1)",
      title: "Made for your desktop.",
      text: "Native process icons, clear controls, saved preferences and a refresh pace you choose. The ghost fills in when an icon is unavailable.",
      detail: "Rust + Tauri + Svelte + shadcn-svelte",
    },
  ];
  const packages = [
    {
      title: "macOS",
      detail: "Apple Silicon",
      items: [
        ["macos-silicon", "Apple Silicon", "DMG"],
        ["macos-intel", "Intel", "DMG"],
      ],
    },
    {
      title: "Windows",
      detail: "64-bit",
      items: [["windows", "Windows x64", "Installer"]],
    },
    {
      title: "Linux",
      detail: "x86_64 & ARM64",
      items: [
        ["linux-deb-x64", "x86_64", ".deb"],
        ["linux-rpm-x64", "x86_64", ".rpm"],
        ["linux-appimage-x64", "x86_64", "AppImage"],
        ["linux-deb-arm64", "ARM64", ".deb"],
        ["linux-rpm-arm64", "ARM64", ".rpm"],
        ["linux-appimage-arm64", "ARM64", "AppImage"],
      ],
    },
  ];
  const themes = [
    ["Catppuccin", "#89b4fa", "#f38ba8", "#a6e3a1"],
    ["Dracula", "#bd93f9", "#ff79c6", "#50fa7b"],
    ["Tokyo Night", "#7aa2f7", "#bb9af7", "#9ece6a"],
    ["Nord", "#81a1c1", "#bf616a", "#a3be8c"],
    ["GitHub Light", "#0969da", "#cf222e", "#1a7f37"],
    ["Gruvbox", "#83a598", "#fb4934", "#b8bb26"],
  ];
</script>

<a class="skip-link" href="#main">Skip to content</a>
<header class="site-header">
  <div class="site-shell header-row">
    <a class="brand" href="./" aria-label="NeoHtop home"
      ><img src="assets/neohtop-icon.png" width="28" height="28" alt="" /><span
        >NeoHtop</span
      ><Badge variant="secondary" class="version-badge">1.3</Badge></a
    >
    <nav aria-label="Main navigation">
      <a href="./#features">Features</a><a href="./#themes">Themes</a><a
        href="install.html">Install</a
      ><a href={`${repo}/blob/main/CHANGELOG.md`}
        >Changelog<ArrowUpRight class="size-3" /></a
      >
    </nav>
    <Button href="./#download" size="sm"><ArrowDown />Download</Button>
  </div>
</header>

<main id="main">
  {#if page === "home"}
    <section class="site-shell hero">
      <Badge variant="outline" class="release-badge"
        ><span class="status-dot"></span>NeoHtop 1.3<span class="badge-divider"
          >/</span
        >More to explore.</Badge
      >
      <h1>Your system.<br /><span>In plain sight.</span></h1>
      <p class="hero-description">
        A modern system monitor for people who like to look under the hood.<br
          class="desktop-break"
        /> Find the process. Follow the spike. Keep the essentials in your menu bar.
      </p>
      <div class="hero-actions">
        <Button href="#download" size="lg" class="hero-download"
          ><ArrowDown />Download NeoHtop</Button
        ><Button
          href={repo}
          variant="outline"
          size="lg"
          target="_blank"
          rel="noopener noreferrer"><ArrowUpRight />Explore the source</Button
        >
      </div>
      <p class="hero-footnote">
        <Check />Free & open source<span>·</span>macOS, Linux & Windows<span
          >·</span
        >No account required
      </p>
    </section>
    <section
      class="product-stage site-shell-wide"
      aria-label="NeoHtop desktop and menu-bar overview"
    >
      <figure class="desktop-preview">
        <img
          src="assets/neohtop-desktop.png"
          alt="NeoHtop in Catppuccin, showing CPU, memory and GPU charts, storage, network and processes"
          width="1280"
          height="900"
          fetchpriority="high"
        />
        <figcaption>
          <span
            ><span class="status-dot"></span>Catppuccin. The familiar ghost. A
            fresh desktop.</span
          ><a href={`${repo}/blob/main/docs/screenshots/README.md`}
            >More screenshots<ArrowUpRight /></a
          >
        </figcaption>
      </figure>
      <figure class="companion-preview">
        <img
          src="screenshots/menubar-overview-system.png"
          width="400"
          height="630"
          alt="The NeoHtop ghost icon in the macOS menu bar with its rounded resource overview open"
        />
        <figcaption>A whole monitor in your menu bar.</figcaption>
      </figure>
    </section>
    <section
      class="site-shell platform-line"
      aria-label="Platforms and project facts"
    >
      <span>Built for your desktop.</span><strong>macOS</strong><strong
        >Linux</strong
      ><strong>Windows</strong><span class="platform-divider"></span><span
        >Rust + Tauri + Svelte</span
      ><a href={`${repo}/blob/main/LICENSE`}>MIT licensed<ArrowUpRight /></a>
    </section>
    <section id="features" class="site-shell section-block">
      <div class="section-heading">
        <p class="eyebrow">A CLOSER LOOK</p>
        <h2>Follow your curiosity.<br /><span>Find what’s going on.</span></h2>
        <p>
          From a busy core to the process behind it. One clear place to explore
          your machine.
        </p>
      </div>
      <div class="feature-grid">
        {#each features as feature}<Card.Root class="feature-card" size="sm"
            ><Card.Header
              ><span
                class="feature-icon"
                style={`--feature-color:${feature.color}`}
                ><feature.icon class="size-5" /></span
              ><Card.Title>{feature.title}</Card.Title></Card.Header
            ><Card.Content
              ><p>{feature.text}</p>
              <span class="feature-detail">{feature.detail}</span></Card.Content
            ></Card.Root
          >{/each}
      </div>
    </section>
    <section class="site-shell section-block resource-section">
      <div class="section-heading">
        <p class="eyebrow">THE NUMBER IS JUST THE START</p>
        <h2>Open a card.<br /><span>Get the useful details.</span></h2>
        <p>
          Explore individual cores, inspect a short history and follow the
          readings that matter.
        </p>
      </div>
      <figure class="detail-preview">
        <img
          src="screenshots/cpu-details.jpg"
          width="1280"
          height="900"
          alt="CPU details with per-core activity and a utilization history"
          loading="lazy"
        />
      </figure>
      <div class="resource-labels">
        <Badge variant="secondary"><Cpu />CPU</Badge><Badge variant="secondary"
          ><MemoryStick />Memory</Badge
        ><Badge variant="secondary"><Activity />Graphics</Badge><Badge
          variant="secondary"><HardDrive />Storage</Badge
        ><Badge variant="secondary"><Monitor />System</Badge><Badge
          variant="secondary"><Network />Network</Badge
        >
      </div>
      <p class="section-note">
        Charts keep roughly two minutes in memory. Gaps show paused or missing
        readings. GPU diagnostics depend on your driver.
      </p>
    </section>
    <section id="themes" class="site-shell section-block">
      <Card.Root class="theme-card"
        ><Card.Content class="theme-content"
          ><div class="theme-copy">
            <Badge variant="outline"><Palette />24 original themes</Badge>
            <h2>Still your setup.<br /><span>Still your atmosphere.</span></h2>
            <p>
              Catppuccin, front and center. Light, dark, colorful and retro when
              you want a change. The themes you love, with consistent controls
              and a searchable gallery.
            </p>
            <div class="theme-chips">
              {#each themes as theme}<Badge
                  variant="secondary"
                  class="theme-chip"
                  ><span class="swatch-group"
                    >{#each theme.slice(1) as color}<i
                        style={`background:${color}`}
                      ></i>{/each}</span
                  >{theme[0]}</Badge
                >{/each}
            </div>
          </div>
          <figure>
            <img
              src="screenshots/themes.jpg"
              width="640"
              height="640"
              alt="The searchable NeoHtop theme gallery, with GitHub Light selected"
              loading="lazy"
            />
          </figure></Card.Content
        ></Card.Root
      >
    </section>
    <section id="download" class="site-shell section-block download-section">
      <div class="section-heading">
        <p class="eyebrow">MAKE YOURSELF AT HOME</p>
        <h2>Meet your next<br /><span>favorite desktop utility.</span></h2>
        <p>
          Free, local and open source. Choose your system and get a clearer
          view.
        </p>
        <Badge variant="outline" id="current-version"
          >Latest published release</Badge
        >
      </div>
      <div class="download-grid">
        {#each packages as platform}<Card.Root class="download-card" size="sm"
            ><Card.Header
              ><Card.Title>{platform.title}</Card.Title><Card.Description
                >{platform.detail}</Card.Description
              ></Card.Header
            ><Card.Content
              >{#each platform.items as item}<Button
                  href={release}
                  variant="outline"
                  class="download-button"
                  data-type={item[0]}
                  ><span class="download-label"
                    >{item[1]}<span class="secondary">{item[2]}</span></span
                  ><ArrowDown /></Button
                >{/each}</Card.Content
            ></Card.Root
          >{/each}
      </div>
      <p id="release-status" role="status" class="section-note">
        Links open the published release page. Direct downloads load from the
        actual release assets.
      </p>
      <div class="download-help">
        <Button href="install.html" variant="ghost"
          >Installation & troubleshooting<ArrowUpRight /></Button
        ><Button href={repo + "/releases"} variant="ghost"
          >All release notes<ArrowUpRight /></Button
        >
      </div>
      <Card.Root size="sm" class="brew-card"
        ><Card.Content
          ><div>
            <strong>Prefer Homebrew?</strong>
            <p>Community-maintained; updates can follow a separate schedule.</p>
          </div>
          <code id="brew-command">brew install --cask neohtop</code><Button
            variant="outline"
            size="sm"
            data-copy="brew-command">Copy</Button
          ></Card.Content
        ></Card.Root
      ><span id="copy-status" class="sr-only" role="status"></span>
    </section>
    <section id="faq" class="site-shell section-block faq-section">
      <div class="section-heading">
        <p class="eyebrow">GOOD TO KNOW</p>
        <h2>A few useful answers.</h2>
      </div>
      <div class="faq-list">
        {#each faq as [question, answer]}<Card.Root size="sm"
            ><Card.Content
              ><details>
                <summary>{question}<span aria-hidden="true">+</span></summary>
                <p>{answer}</p>
              </details></Card.Content
            ></Card.Root
          >{/each}
      </div>
    </section>
    <section id="pro" class="site-shell project-note">
      <div>
        <img src="assets/neohtop-icon.png" width="40" height="40" alt="" />
        <div>
          <h2>Open source at the core.</h2>
          <p>
            NeoHtop Pro is postponed. This release is the free local desktop
            app.
          </p>
        </div>
      </div>
      <Button href="https://github.com/sponsors/Abdenasser" variant="outline"
        >Support development<ArrowUpRight /></Button
      >
    </section>
  {:else if page === "install"}
    <article class="site-shell guide-page">
      <Badge variant="outline">GET STARTED</Badge>
      <h1>Make it part<br /><span>of your desktop.</span></h1>
      <p class="guide-intro">
        Choose an official package for your operating system and architecture.
        Version 1.3.0 includes the refreshed desktop, GPU diagnostics, process
        families and menu-bar companion.
      </p>
      <div class="guide-grid">
        <Card.Root
          ><Card.Header><Card.Title>macOS</Card.Title></Card.Header
          ><Card.Content
            ><ol>
              <li>Check Apple menu → About This Mac for your chip.</li>
              <li>
                Download the Apple Silicon or Intel DMG from GitHub Releases.
              </li>
              <li>
                Open the DMG, drag NeoHtop into Applications and launch it.
              </li>
            </ol>
            <p>
              Release DMGs require Developer ID signing and Apple notarization.
              Check release notes if a download cannot be verified.
            </p>
            <Button href={release} variant="outline"
              >Choose a Mac download<ArrowUpRight /></Button
            ></Card.Content
          ></Card.Root
        ><Card.Root
          ><Card.Header><Card.Title>Windows</Card.Title></Card.Header
          ><Card.Content
            ><ol>
              <li>
                Download the Windows x64 file ending in <code>-setup.exe</code>.
              </li>
              <li>Run the installer and follow the prompts.</li>
              <li>Launch NeoHtop from your installed apps.</li>
            </ol>
            <p>
              Older releases may provide a portable executable instead. Windows
              code signing is not configured.
            </p>
            <Button href={release} variant="outline"
              >Choose a Windows download<ArrowUpRight /></Button
            ></Card.Content
          ></Card.Root
        ><Card.Root
          ><Card.Header><Card.Title>Linux</Card.Title></Card.Header
          ><Card.Content
            ><p>
              <code>uname -m</code> reports <code>x86_64</code> or
              <code>aarch64</code> (ARM64).
            </p>
            <ul>
              <li>Debian/Ubuntu: choose the matching <code>.deb</code>.</li>
              <li>Fedora/RPM distributions: choose <code>.rpm</code>.</li>
              <li>
                AppImage: mark executable in Properties and open it.
                Distribution libraries and possibly FUSE are still required.
              </li>
            </ul>
            <Button href={release} variant="outline"
              >Choose a Linux package<ArrowUpRight /></Button
            ></Card.Content
          ></Card.Root
        >
      </div>
      <section id="troubleshooting">
        <h2>Something needs a closer look?</h2>
        <h3>Linux reports a glibc error or a blank window</h3>
        <p>
          Check distribution, package type and release notes. Version 1.3.0 uses
          an Ubuntu 22.04 build baseline and checks AppImages for bundled host
          Wayland libraries. Older releases can differ. An appropriate native
          package may work better on your distribution.
        </p>
        <h3>Some process, socket or GPU details are unavailable</h3>
        <p>
          Operating-system permissions and GPU drivers determine which counters
          can be read. Unavailable does not mean idle. Start without
          administrator access; the app does not elevate automatically.
          Per-process GPU engine activity requires a supported Linux DRM driver.
        </p>
        <h3>Where should I report a problem?</h3>
        <p>
          Include OS, architecture, app version, package type, error text and
          reproduction steps. Redact private command lines, environment values
          and identifiers.
        </p>
        <Button href={repo + "/issues"} variant="outline"
          >Report an issue<ArrowUpRight /></Button
        >
      </section>
      <section id="source">
        <h2>Build it yourself.</h2>
        <p>
          Use Node.js 22, Rust 1.88 or newer, and the relevant Tauri platform
          prerequisites.
        </p>
        <pre><code
            >git clone https://github.com/Abdenasser/neohtop.git
cd neohtop
npm ci
npm run tauri dev</code
          ></pre>
        <Button
          href={repo + "/blob/main/.github/DEVELOPMENT.md"}
          variant="outline">Development guide<ArrowUpRight /></Button
        >
      </section>
      <p class="section-note">
        Community package managers are maintained independently. Pro is
        postponed and is not part of these downloads.
      </p>
    </article>
  {:else if page === "privacy"}
    <article class="site-shell guide-page">
      <Badge variant="outline">YOUR DATA</Badge>
      <h1>Local readings.<br /><span>Clear choices.</span></h1>
      <p class="guide-intro">
        NeoHtop Desktop v1.3.0 privacy notice · Updated October 29, 2026.
      </p>
      <h2>Your system stays on your machine.</h2>
      <p>
        NeoHtop does not upload process names, commands, search text, file
        paths, environment values, port numbers or your system readings.
        Monitoring works offline, without an account.
      </p>
      <h2>One basic launch count.</h2>
      <p>
        Official builds with analytics configured send one app-open event per
        launch, including the app version, even when optional sharing is off. It
        has no persistent identifier or OS field. It counts launches, not unique
        people.
      </p>
      <p>
        The HTTPS request passes through our Cloudflare Worker to Google
        Analytics 4. Cloudflare receives your IP address to handle the
        connection. Our Worker does not forward it or your request headers to
        Google, store events or write request logs. It generates a fresh random
        GA client ID for each basic launch. Service providers also process data
        under their own policies; network requests are not anonymous to them.
      </p>
      <h2>Feature sharing is your choice.</h2>
      <p>
        Settings → Privacy → Share feature usage is off by default. If enabled,
        it shares use of resource views, built-in themes, list/tree view, the
        companion, inspector, Settings and port search, together with your app
        version and OS family. Search queries and port numbers are never sent.
      </p>
      <p>
        Optional sharing uses a random installation ID and a launch-session ID.
        Turning it off stops future optional events and deletes the local
        installation ID; enabling it again creates a new one. It does not erase
        data already received by Google. Advertising personalization and
        ad-user-data consent are denied.
      </p>
      <h2>Monitoring comes first.</h2>
      <p>
        Events have a short timeout, no disk queue and no retries. Blocked or
        unavailable analytics never prevents monitoring. Reopening a hidden
        window does not count another launch. Self-built apps without an
        analytics endpoint send no desktop events.
      </p>
      <h2>The website is separate.</h2>
      <p>
        This production website uses Google Analytics 4 (G-88HFXRBNLS) for page
        usage. Google's browser tag can set analytics cookies and receives
        browser and network information. Desktop preferences do not control
        website analytics. Analytics is disabled on localhost; blocking it does
        not affect downloads or navigation.
      </p>
      <h2>Read more or ask us.</h2>
      <p>
        See the <a href="privacy.md">full Markdown notice</a>,
        <a href="https://www.cloudflare.com/privacypolicy/"
          >Cloudflare's privacy policy</a
        >
        and
        <a href="https://policies.google.com/privacy">Google's privacy policy</a
        >. Questions can go to the
        <a href={repo + "/issues"}>project issue tracker</a>; please omit
        private information.
      </p>
    </article>
  {:else}
    <article class="site-shell guide-page">
      <Badge variant="outline">TWO WAYS TO LOOK UNDER THE HOOD</Badge>
      <h1>Desktop or terminal.<br /><span>Your kind of workspace.</span></h1>
      <p class="guide-intro">
        NeoHtop Desktop and NeoHtop CLI are separate, MIT-licensed local
        monitors. Install one, or use both. Neither requires the other.
      </p>
      <Card.Root class="comparison-table"
        ><Card.Content
          ><table>
            <thead
              ><tr
                ><th>Choose your view</th><th>NeoHtop Desktop</th><th
                  >NeoHtop CLI</th
                ></tr
              ></thead
            ><tbody
              ><tr
                ><th>Interface</th><td>Native desktop window</td><td
                  >Terminal UI</td
                ></tr
              ><tr
                ><th>Built with</th><td>Rust, Tauri, Svelte</td><td
                  >Go and the Charm ecosystem</td
                ></tr
              ><tr
                ><th>Monitoring</th><td
                  >Processes, CPU/GPU, memory, storage, network, menu-bar
                  overview</td
                ><td>Process metrics, CPU sparklines, memory visualizations</td
                ></tr
              ><tr
                ><th>Process families</th><td
                  >Expandable trees with CPU/RAM totals in v1.3.0</td
                ><td>Documented in the CLI project</td></tr
              ><tr
                ><th>Install</th><td
                  >GitHub packages or community package managers</td
                ><td><code>npm install -g neohtop-cli</code></td></tr
              ><tr><th>License</th><td>MIT</td><td>MIT</td></tr></tbody
            >
          </table></Card.Content
        ></Card.Root
      >
      <div class="guide-grid two">
        <Card.Root
          ><Card.Header
            ><Card.Title>Room for the whole picture.</Card.Title></Card.Header
          ><Card.Content
            ><p>
              Choose Desktop for a visual process table, resource detail sheets,
              driver-dependent GPU diagnostics and a menu-bar companion.
            </p>
            <Button href="./#download">Download Desktop<ArrowDown /></Button
            ></Card.Content
          ></Card.Root
        ><Card.Root
          ><Card.Header
            ><Card.Title>Stay in your shell.</Card.Title></Card.Header
          ><Card.Content
            ><p>
              Choose CLI for keyboard-driven monitoring in your terminal, with
              its own feature set and release schedule.
            </p>
            <pre><code>npm install -g neohtop-cli</code></pre>
            <Button
              href="https://github.com/Abdenasser/neohtop-cli"
              variant="outline">Explore NeoHtop CLI<ArrowUpRight /></Button
            ></Card.Content
          ></Card.Root
        >
      </div>
      <p class="section-note">
        NeoHtop Pro is postponed and is not available. No price, scope or
        release date is announced. Both projects remain independent open-source
        local monitors.
      </p>
    </article>
  {/if}
</main>
<footer class="site-footer">
  <div class="site-shell">
    <div class="footer-top">
      <a class="brand" href="./"
        ><img
          src="assets/neohtop-icon.png"
          width="24"
          height="24"
          alt=""
        />NeoHtop</a
      >
      <p>Your system. In plain sight.</p>
      <nav aria-label="Footer">
        <a href={repo}>GitHub</a><a href="privacy.html">Privacy</a><a
          href="desktop-vs-cli.html">Desktop vs CLI</a
        ><a href="https://www.buymeacoffee.com/abdenasser"
          >Buy me a coffee<ArrowUpRight /></a
        >
      </nav>
    </div>
    <div class="footer-bottom">
      <span>Made by Abdenasser. Free under the MIT license.</span>
      <div>
        <a
          href={page === "home"
            ? "index.md"
            : page === "install"
              ? "install.md"
              : page === "privacy"
                ? "privacy.md"
                : "desktop-vs-cli.md"}>Read as Markdown</a
        ><a href="llms.txt">llms.txt</a><a href="sitemap.xml">Sitemap</a>
      </div>
    </div>
  </div>
</footer>
