# Developing and releasing NeoHtop

Use Node.js 22 and Rust 1.88 or newer. Install the [Tauri platform prerequisites](https://v2.tauri.app/start/prerequisites/) for your OS. Shared controls live in `src/lib/components/ui`; compose shadcn-svelte primitives and semantic theme tokens before adding custom widgets.

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

GitHub Pages serves the generated static `docs/` directory from `main`. The
website source is `site/Site.svelte`, with the same shadcn buttons, cards, badges
and Catppuccin semantic tokens used by the desktop app. A small Svelte server
render produces complete HTML; there is no Svelte client bundle to hydrate.
Native FAQ disclosures, links and download fallbacks work without JavaScript.

After editing the website source, regenerate all three public pages and CSS:

```bash
npm run build:site
npm run test:release
python3 -m http.server 8765 --bind 127.0.0.1 --directory docs
```

Open `http://127.0.0.1:8765/`. Commit both the source and generated pages/assets.
Do not hand-edit generated HTML. `docs/main.js` progressively enhances actual
published-release download links and copy controls. Website analytics is separate from the desktop relay and does not affect navigation or downloads. Desktop analytics requires its explicit build-time endpoint; see `telemetry/README.md`.

Each public HTML page has a canonical URL, social metadata, structured data,
and a linked Markdown equivalent. Keep the HTML, Markdown, `llms.txt`, and
`sitemap.xml` consistent when changing product facts. The site uses local assets,
system fonts, native FAQ disclosures, and reduced-motion styles. Blocking Google Analytics must not affect downloads or navigation.

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

### Published website downloads

`docs/release.json` is a small snapshot of an actual published stable release, not a version inferred from the source package. It supplies all installer links during static rendering so downloads work without JavaScript, a successful API call or a fresh browser cache. The live lookup can only advance to a newer stable release; older cached results and partial same-version responses cannot replace verified links.

After publishing a new release, run `npm run refresh:site-release`, then `npm run build:site` and `npm run test:release`, and commit the snapshot and generated website. Website runtime scripts use content-hashed filenames; retain earlier hashed bundles so cached HTML can still load its assets. Homebrew is independently maintained and can still offer an older version.
