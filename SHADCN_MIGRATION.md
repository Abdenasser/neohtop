# Open-source shadcn migration — 28 October 2026

Implemented on local branch `codex/shadcn-ui-migration`, based on public main `7c4007b`. The approved visual prototype informed the presentation. This is the real Tauri application using its existing Rust collectors and process commands, not the prototype's sample data.

## What changed

Official shadcn-svelte Vega primitives now provide buttons, cards, tables, inputs, numeric fields, checkboxes, selects, popovers, menus, disclosures, process details and confirmations. Tailwind 4 and semantic tokens replace the independent per-control CSS and full-toolbar overlays. `components.json` records the registry/style/aliases; `src/lib/components/ui` owns the reusable upstream component source. Add or compose these primitives before introducing another control. Domain components such as CPU panels and process rows compose the shared library.

The application keeps its resource-first layout and dense process explorer. Pagination now sits beneath the table. Headers and action columns remain visible during scrolling; the first name column stays visible in Tree view. Default columns fit the normal 1280px window. Wider optional columns and the supported 1120px minimum use deliberate horizontal scrolling, including keyboard Left/Right navigation in the labeled table region.

`src/lib/definitions/themes.ts` and `THEME_GROUPS` are unchanged. All 24 palettes remain, with the original platform restriction on Glassy. The existing `theme` and `neohtop_config` storage keys remain. Each palette maps to shared surfaces, foregrounds, accent, border, focus, warning and destructive roles; light/dark component variants follow palette brightness. Low-contrast secondary text is strengthened within its existing hue. Process icons now react to theme changes. Glassy retains transparent surfaces and the existing native vibrancy/acrylic implementation.

## Feature parity

| Area | Retained behavior |
| --- | --- |
| Resources | Per-core and average CPU; memory total/used/free and utilization; storage total/used/free and utilization; uptime and 1/5/15-minute load; received/sent interface rates. Large core counts scroll within the CPU panel. |
| Search | Name, command and PID; case-insensitive regular expressions for names; comma-separated alternatives; syntax examples. Added a stable search field, clear action and Cmd/Ctrl F. |
| Filters | CPU, resident MB and runtime minutes; comparison operators and thresholds; multiple statuses; combined conditions; clear/reset. |
| Table | Every existing column, required Process Name, persisted visibility, sorting, command-based pinning, process icons/ghost fallback, details and end actions. |
| Families | List/Tree choice, parent/child hierarchy, expansion/collapse-all, combined CPU/RAM and overlap explanation, expanded matching ancestor branches, family-based pagination and stable process identities. The tree engine is unchanged. |
| Controls | Every page-size and refresh-rate option, legacy saved values, pause/resume and persisted settings. |
| Details | Name/PID/status/user, CPU, resident/virtual memory, disk read/write, root/command, session, parent and child navigation, environment variables. Also shows the already available start/runtime/thread metadata. Environment data is disclosed on demand. |
| Process end | Existing Rust command and asynchronous busy/error path, explicit selected name/PID, cancel and confirmation. Busy execution prevents cancellation. |
| About | Ghost branding, ASCII art, version, GitHub source/release links, stack information and latest-release check. Numeric version comparison now avoids advertising an older release as an update. |

A small page-state correction resets the actual process store when search/filter/page size changes; the next live snapshot can no longer undo the displayed reset.

## Validation performed

- Svelte/TypeScript: **0 errors, 0 warnings**.
- **26 JavaScript tests passed**, including existing process-tree/release/website cases and added palette-role/mode and release-ordering cases.
- **4 Rust tests passed**. No Rust collectors, commands, native configuration, palette definitions or website sources changed.
- Repository formatting check, version check and whitespace check passed.
- Production frontend build and optimized arm64 macOS app bundle passed. Dependency lockfile retains macOS, Linux and Windows Tailwind native variants.

### Actual native workflows

- Selected every one of the 24 themes in the desktop app, including all light, retro, accessibility and Glassy choices; returned to Catppuccin. Inspected final light/dark output and the supported minimum window. These checks do not claim exhaustive contrast certification for every possible translucent background.
- Paused/resumed live sampling; changed pages and page size; changed refresh interval. The 5s/25-row choices, theme and column visibility survived restarting into the optimized build. Restored 3s/15 rows and List view afterward.
- Enabled all six initially hidden columns and restored their defaults. Required Process Name remained protected.
- Used a clean, disposable parent/child fixture to inspect identity, command/root and the test environment marker. Followed child and parent details with the keyboard. Final detail navigation focuses the newly selected process heading.
- Verified PID, command, regex and comma-separated search. Combined memory/runtime/status conditions matched the fixture; an incompatible status or CPU threshold excluded it; Reset restored the result. Search help, Escape and Cmd F were checked.
- Verified filtered families and combined totals, then expanded an unfiltered branch and used Collapse all. Family sorting/pagination safeguards remain covered by the existing tree tests.
- Pinned and unpinned the fixture; cancelled an end confirmation, then confirmed only one agent-created disposable child PID and observed it leave the list. No real user process was stopped. The remaining fixture and task-created development previews were shut down afterward.

Local screenshot evidence is kept outside the public repository in `Documents/Codex/NeoHtop Open Source Migration 2026-10-28/`. It is not part of a public upload.

## Local preview and remaining release checks

The optimized QA bundle is `src-tauri/target/release/bundle/macos/NeoHtop Shadcn Preview.app` (build-reported 7.34 MiB). A temporary config gives it a separate preview identifier; the repository's normal product identifier and window configuration are unchanged. The QA bundle was signed locally with an ad-hoc identity, and `codesign --verify --deep --strict` passed. Final executable SHA-256: `861acc490f7b11235746fd130502832fb69bec91a654fbbcb142fc98cbb13a08`.

No version bump, public push, tag, release, Developer ID signing or notarization was performed. Windows/Linux native rendering and packaging still need their respective CI/device checks; a macOS build does not establish those results. Long-session/battery performance and exhaustive assistive-technology evaluation were not performed in this UI migration.

## Provenance

Only public NeoHtop code and official open-source shadcn registry components were used. No proprietary Pro components, capabilities or assets were copied. The upstream MIT notice is in `licenses/shadcn-svelte-MIT.md`. Registry: https://www.shadcn-svelte.com/registry/styles/vega/ . Installation/theming conventions: https://www.shadcn-svelte.com/docs/installation/manual and https://www.shadcn-svelte.com/docs/theming .


## Resource chart refinement — 29 October 2026

CPU, memory and network now compose the official shadcn chart container with LayerChart area plots and theme-colored vertical gradient fills. CPU retains all per-core bars; memory retains total/used/free. Their histories use fixed 0–100% scales. Network receive/send overlay on one shared, zero-based rate scale, with solid receive and dashed send strokes and matching inline keys. The system card retains the existing uptime/load facts. Storage uses an explicit used/free capacity bar, rather than treating disk capacity as a changing rate.

Charts use only actual observations from the existing native sampling loop. History stays in memory, covers at most two minutes, and is capped at 160 records. There is no extra collector, timer, saved telemetry or startup backfill. Pause, failed reads and unusually delayed reads break the traces; resuming does not fabricate measurements across the gap. In-flight reads are shared and cannot add history after monitoring is paused. Missing values remain unknown; measured zero remains zero.

Each chart supports pointer inspection and keyboard Left/Right, Home/End and Escape, with exact sample time/readings exposed through its accessible value and native title. Network inspection uses a compact two-rate caption. An inspected sample stays near the oldest available observation when it ages out, rather than jumping to the latest reading. The five-card layout, native collectors, existing settings and all 24 palette definitions remain unchanged.

Validation for this refinement: Svelte check (0 errors, 0 warnings), 31 JavaScript tests, repository formatting, version and whitespace checks, production frontend and optimized macOS bundle. New tests cover unknown/zero readings, explicit and missed-read gaps, bounded history, clock changes, sampling intervals, shared network scaling and nearest-sample inspection. The four Rust tests recorded above apply to the unchanged backend; they were not rerun for this frontend refinement.

Native checks covered real live gradient fills, per-core/totals visibility, keyboard sample inspection, pause/resume gaps, GitHub Light and Green Terminal, and the supported 1120×700 window. Local screenshot evidence is in `Documents/Codex/NeoHtop Charts 2026-10-29/`, outside the public repository. This pass does not establish Windows/Linux rendering or long-session/battery performance.

The chart components come from https://www.shadcn-svelte.com/registry/styles/vega/chart.json and public LayerChart. The LayerChart MIT notice is in `licenses/layerchart-MIT.md`; no private Pro source or assets were copied.

The refreshed optimized preview uses the same separate QA bundle path noted above (build-reported 7.51 MiB). Its local ad-hoc signature passes strict verification. This chart build supersedes the earlier preview receipt; final executable SHA-256: `8feab3f683b1baac484ec2d23dd5be1e3f273d889255abd220e611c1e872af03`. No public release or version bump was performed.


## Native app icons — 29 October 2026

The table and process-details header now use real system app icons, keeping their existing sizes and spacing. macOS resolves the outermost owning `.app` bundle so nested browser helpers share their parent app icon. Windows requests a shell icon for the executable. Linux matches the actual executable to the primary installed XDG desktop entry and resolves its icon through the configured GTK icon theme, inherited themes, hicolor and pixmaps. Linux matching is deliberately conservative: wrappers/interpreters, missing desktop entries, inaccessible or oversized images, and failed decoding use the ghost. Native app images retain their colors; the ghost retains the selected theme's text color. No icon packs are guessed from process/company names or downloaded.

Only visible rows and the open detail header request icons. Requests are batched in groups of at most 32, with one client batch active at a time, a 256-request backlog cap, and a 256-entry client cache. Positive results last five minutes, misses one minute and transient IPC failures five seconds. The native cache shares images by bundle/executable/icon-file path, checks modification time, expires entries and evicts beyond 256 entries. Images are capped at 128 KiB; macOS/Windows extract 64px PNGs. Native work runs on a blocking worker after the sampling lock is released; it adds no poller or image payload to normal process snapshots. Temporary AppKit objects have an autorelease pool.

The IPC accepts PID/start-time pairs and derives paths from the native process snapshot. It never accepts arbitrary frontend file paths. Reused PIDs, old detail responses, malformed payloads and image-load failures cannot overwrite the current row with a stale or remote icon. Existing process layout, charts, actions, theme definitions and saved preferences are retained. Static import of just the existing ghost replaces the previous whole icon-catalog lookup.

Validation: 37 JavaScript tests and 8 native Rust tests passed; Svelte check reports zero errors/warnings. New tests cover shared pending requests, batching/backlog/cache bounds, expiration and retry, reused PID identity, image URL validation, owning bundles, primary desktop-entry parsing and malformed commands. A portable harness also type-checked the Linux matcher against freedesktop-icons on macOS and passed its two parser tests; this is not a Linux desktop integration test. The Rust test run still reports two pre-existing unsigned-comparison warnings in the system-monitor tests.

Native macOS review verified real table icons, a matching 28px detail icon, navigation from Chrome to a nested helper, ghost fallback and GitHub Light/Catppuccin appearances. Evidence remains outside the public repository in `Documents/Codex/NeoHtop Native Icons 2026-10-29/`. Windows and Linux native rendering/packaging still require platform CI/device checks. Linux wrapper-based launchers may remain ghosts rather than receiving a guessed app identity.

Public dependencies: [file_icon_provider 1.0.2](https://docs.rs/file_icon_provider/1.0.2/file_icon_provider/) and [freedesktop-icons 0.4.0](https://docs.rs/freedesktop-icons/0.4.0/freedesktop_icons/). Notices are in `licenses/file-icon-provider-MIT.md` and `licenses/freedesktop-icons-MIT.md`. No private Pro code or assets were copied.

The optimized macOS preview with native icons passed local ad-hoc signing verification and reports 5.57 MiB, down from 7.51 MiB for the previous chart preview after removing the full dynamically indexed brand-icon catalog. This receipt supersedes the earlier chart build: executable SHA-256 `d53a6557b0aeffae239db3c71dd8844c9bb146ccf6e2378cd9f65a20b8a2ea3d`. The normal product identifier and version remain unchanged; no public push or release was performed.


## Resource-card hierarchy and compact cores — 29 October 2026

The five existing resource cards now share a consistent header, primary reading, secondary facts and lower visualization area. CPU emphasizes average utilization with a compact numbered strip of per-core meters; memory emphasizes used RAM with total/free retained; storage emphasizes free space with total/used and both capacity percentages retained; system emphasizes full uptime and the 1/5/15-minute load averages; network emphasizes received rate while retaining a separately keyed sent rate. Values remain neutral, while theme-derived icon accents, CPU meters and plot/legend colors identify the measured series. High CPU utilization alone no longer turns the individual core meters into warning colors.

The compact core strip composes official shadcn Button, Progress and Tooltip primitives. Each core has an exact one-decimal reading on hover/focus and an accessible label. One Tab stop enters the strip; arrow keys traverse cores, Home/End move to the first/last, Escape dismisses the tooltip and Tab proceeds to the CPU history chart. Larger core counts wrap in a bounded scrolling region rather than increasing the card height. All per-core values remain available; no private Pro implementation or assets were copied.

Charts are now 48px tall and align along the bottom of the cards. The five-card desktop grid and the process explorer remain; the resource row uses 240px cards with consistent insets. The supported 1120px layout retains five cards, and the existing narrower browser fallback scrolls deliberately. Storage remains a capacity visualization rather than a fabricated time series. Windows explicitly shows that load average is unavailable instead of treating its unsupported zero tuple as a measurement.

All existing collectors, volatile histories, pause/gap behavior, app icons, saved settings, palette definitions and process workflows remain. Svelte check passed with zero errors/warnings, all 37 JavaScript tests passed, and repository formatting passed. The backend is unchanged from the previous native-icon pass and its eight Rust tests were not repeated for this presentation change. Tooltip source comes from the public Vega registry: https://www.shadcn-svelte.com/registry/styles/vega/tooltip.json .

Native review verified light/dark hierarchy, real per-core values, arrow/Home/End navigation, Escape dismissal and the single-stop Tab sequence. Local screenshots are in `Documents/Codex/NeoHtop Resource Cards 2026-10-29/`, outside the public repository. This pass does not establish Windows/Linux native rendering, exhaustive contrast certification or long-session/battery performance.

The final optimized preview also passed native light/dark checks at the supported 1120×700 size, then returned to 1280×900, Catppuccin and live monitoring. Its local ad-hoc signature passed strict verification; build-reported size is 5.57 MiB. This resource-card build supersedes the earlier icon receipt: executable SHA-256 `d51991080e2448433aca3ab393c675240701dcd4c5ed4c8235fe84bdd37382ec`. No release, version bump or public push was performed.


## Complete community-theme adaptation — 29 October 2026

All 24 original palette definitions, labels and saved identifiers remain unchanged. Their shadcn roles are now derived independently instead of treating palette swatches as interchangeable component colors. Text is resolved against page, card, popover, muted, secondary and accent surfaces; dark field fills, select hover fills, secondary hover, primary hover, selected-row tint and destructive-action tints are included in the color checks. Adjustments move colors toward black/white while retaining their source hue. Primary/destructive fill labels, half-opacity focus rings, input boundaries, chart strokes/resource-icon tints, status dots and scrollbars have explicit contrast checks. These are modeled sRGB token/state checks, not a certification of every rendered pixel or assistive-technology workflow.

Legacy alpha colors are composited against the actual canvas before becoming component surfaces. Ayu Light's translucent mantle now becomes a stable opaque card color; portaled popovers and inspectors have opaque theme surfaces. Light popovers lift toward white within the palette; dark popovers lift slightly from their card surface. The shared Card, Popover, Dialog and Dropdown primitives use the semantic border token rather than a fixed foreground-opacity edge. All five chart tokens are also exposed as Tailwind color utilities. Text selection, process-state markers and scrollbars use semantic theme roles.

High Contrast receives stronger structural/input edges and main-text checks. The three retro themes use tighter radius tokens. Glassy retains native backdrop effects and frosted content, with an 85% application tint and 78% content tint; token checks include black/white backdrop extremes, while portaled menus/details remain readable opaque surfaces. Saved theme preferences and the original platform restriction on Glassy are unchanged. Theme resolution runs only on theme application; it adds no sampling work or background timer.

The review found that the original migration omitted the public `shadcn-svelte/tailwind.css` import. This prevented Vega's checked/open/closed state variants from matching Bits UI's `data-state` attributes. The shared upstream stylesheet is now imported; selected checkboxes, menu state styling, opening/closing effects and orientation variants use the proper conventions. `npm run build` also runs `scripts/check-shadcn-css.mjs`, which checks the emitted CSS for actual checked/open/closed selectors. Theme search now folds accents, case and keyboard whitespace: “Rose” finds “Rosé Pine.”

Validation: zero Svelte errors/warnings, **65 JavaScript tests passed**, formatting/whitespace and version checks passed, production frontend/native preview builds passed, and the emitted-CSS state check passed. The expanded per-theme tests cover resolved surfaces, state contrast, opacity, action labels, focus, chart/status/scrollbar colors, intended modes, source-palette immutability, bootstrap CSS and saved identifiers. Known alpha/luminance vectors check the color math. Native backend collectors and commands were not changed by this pass; Rust tests were not rerun.

Every one of the 24 themes was selected and visually reviewed in the desktop app, with individual screenshots and six comparison sheets. Native follow-up covered Ayu Light, Bubblegum, High Contrast and Glassy checked controls/focus, Glassy column menus, the High Contrast inspector and an end confirmation for the agent's preview process that was cancelled. No process-end action was executed. The new accent-insensitive search was verified in the app. Evidence and the per-theme token audit JSON are under `Documents/Codex/NeoHtop Theme Audit 2026-10-29/`, outside the public repository. Windows/Linux native rendering, arbitrary third-party app-icon contrast, exhaustive accessibility and long-session/battery performance remain outside this macOS theme pass.

Official references: [shadcn-svelte theming](https://www.shadcn-svelte.com/docs/theming), [shared Tailwind stylesheet](https://www.shadcn-svelte.com/docs/utils/scroll-fade), and the installed `shadcn-svelte/dist/tailwind.css` state definitions. No private Pro code or assets were copied; no public release, push or version bump was performed.

The final optimized theme preview passed strict local ad-hoc signature verification and reports 5.57 MiB. It was reopened with Catppuccin, an empty search, no active filters and live monitoring. This receipt supersedes the earlier resource-card build: executable SHA-256 `40453aa7b80f13c122dc0f48545b2f25d5f1d2888c6cb91b1209fdc9594ef4cb`.


## GPU monitoring and menu-bar companion — 29 October 2026

Added a sixth resource card for GPU utilization, a theme-aware gradient history,
and a shadcn Graphics sheet with adapter selection, memory, core count,
temperature and driver-source/availability explanations. A selected adapter's
history is keyed by its identity; adapters are never averaged together. Unknown
counters stay null, and real zero utilization remains zero.

macOS reads targeted IOAccelerator properties directly through IOKit, releasing
all owned CF/IOKit handles. Apple silicon reports `Device Utilization %`,
`In use system memory` and the exposed core count. Shared memory is labelled as
part of RAM; allocation totals are not misrepresented as VRAM capacity. Linux
uses installed NVIDIA NVML and AMD amdgpu sysfs counters with DRM inventory
fallbacks. Windows uses NVML and DXGI inventory. NVML is dynamically loaded once
with a bounded retry after failure; no shell polling, driver installation or root
prompt is introduced. Windows non-NVIDIA and Linux non-AMD/non-NVIDIA adapters
currently provide inventory only. Unavailable temperature/capacity is explicit.

Added a hidden, borderless native overview window, a public pixel-ghost tray
icon (a macOS template), native context menu and an Overview toolbar entry. Its
rounded shadcn card includes CPU/RAM/GPU readings, network rates, free storage,
five top CPU processes with existing native-icon resolution, global pause/resume,
process-inspector navigation, Open and Quit. Icon visibility and optional
keep-running-on-close are saved; the default closing behavior still quits.
Tray creation/removal/destruction run on the main thread. Disabling the tray
also disables hiding on close. The Dock can reopen the main window on macOS.
Linux desktops without left-click tray events can use the native Show overview
menu or toolbar. Placement uses physical coordinates, monitor scale and bounds.

Both windows now share one native interval-gated frame and one bounded volatile
two-minute history (maximum 160 records), with capture timestamps rather than
frontend read timestamps. Expensive refresh runs off the UI thread. The panel
receives only summary stats and the five process identities/readings, and stops
polling and its freshness timer while hidden. Pause, missed reads, collection
errors and clock reversal keep honest gaps. Theme changes synchronize through
local app events. Existing saved settings get nested defaults. Network counter
resets now produce unavailable rates instead of underflowing. Process inspector
selection verifies PID plus start time; existing process features remain intact.
The companion receives only core permissions, without the main window's shell
or window-management permissions.

Validation: 18 Rust tests and 61 JavaScript tests passed, zero Svelte errors or
warnings. Four frontend recording tests were replaced by backend/shared-history
coverage because recording now lives in Rust. Tests include actual shared-frame
reuse, pause/resume, bounded history, GPU identities/nulls, Core Foundation
property types, AMD sysfs fixtures, network resets and scaled/negative-monitor
placement. The independently compiled Linux and Windows GPU adapters passed
`cargo check` with their respective targets; this is not physical hardware QA.
Native macOS checks observed real Apple M1 Pro utilization/shared memory, GPU
details, both theme modes, shared pause, process-inspector navigation, tray
preference toggles, rounded edges and optional close behavior. Direct physical
status-icon clicks, multi-monitor placement, other GPU hardware, OS sleep/wake
and long-session battery behavior remain manual/device checks.

Public references: [Tauri tray API](https://v2.tauri.app/learn/system-tray/),
[IORegistry property ownership](https://developer.apple.com/documentation/iokit/1514453-ioregistryentrycreatecfproperty),
[NVML wrapper](https://docs.rs/nvml-wrapper/0.13.0/nvml_wrapper/struct.Nvml.html),
and [amdgpu sysfs](https://www.kernel.org/doc/html/latest/gpu/amdgpu/driver-misc.html).
All implementation was created in the public repository without private Pro
code/assets. No release, push, merge, tag or version bump was performed.

The final optimized macOS preview passed strict local ad-hoc signature verification and reports **5.86 MiB**. Executable SHA-256: `86f796e511e966da1af506103fc4813d4dd30a6117ba681fd2c123db0e71cd20`. Native final review also verified the six-card layout at 1120×700, the GPU details sheet, light/dark companion screens with all five rows and their hint visible, shared pause/resume, and Escape dismissal. The preview was left at 1280×900 with Catppuccin and live updates. Screenshots and validation receipts are in `Documents/Codex/NeoHtop GPU and Menu Bar 2026-10-29/`, outside the public repository.


## Matching resource detail panels — 29 October 2026

Every resource card now has the GPU-style disclosure arrow. All six use one
shared shadcn Sheet composition with a heading, source timestamp, pause/error
state, scrolling body, fixed Pause/Resume and Done controls, and explicit focus
recovery to the originating arrow. The main app layout and all theme palettes
remain unchanged. CPU and memory header facts were rearranged to fit the arrow
without squeezing their labels or removing readings. Detail histories reuse the
same chart component at a larger size; opening a panel focuses its heading rather
than selecting and holding a chart sample.

CPU includes model, physical/logical counts, average history and readable per-core
meters. Memory includes RAM use/history, not-in-use RAM, OS unallocated pages and
swap; these are not presented as a pressure score. The overview's former Free
label is now Not in use, matching total minus reported use. The pinned sysinfo
macOS available-memory estimate can saturate to zero despite a nonzero free-page
count, so it is deliberately not exposed as a reliable available-memory reading.
Storage has a volume picker, mount path, filesystem, capacity, available/used
space and read-only mode, with shared-pool/APFS context. System includes OS,
kernel, runtime architecture, processor, uptime and supported load averages.
Network adds per-interface rates and OS lifetime totals, search, unused-interface
visibility and recoverable empty states. Relevant traffic is prioritized on open;
rows retain their order during updates. Search covers all reported interfaces.

The native snapshot now carries these details from the existing System, Disks
and Networks instances. Identity information is cached; opening a sheet adds no
collector, polling timer, file scan, dependency or permission. Receive/send rates
use each interface's own previous counter and a common interval. New/reset
interfaces are unknown until a baseline exists, removed interfaces do not create
spikes, and aggregate rates require complete current observations. Virtual and
loopback double counting and OS counter resets are explained in the network view.

Validation: **20 Rust tests**, **61 JavaScript tests**, zero Svelte errors/warnings,
formatting, whitespace, version checks and the optimized native build passed.
Native review covered all six sheets in Catppuccin and Ayu Light, mounted-volume
switching, interface search/no-results/clear recovery, unused-adapter toggling,
shared pause/resume across the main app and companion, 1120×700 scrolling with
fixed actions, and focus recovery on Escape/Done for every arrow. Screenshots and
receipts are in `Documents/Codex/NeoHtop Resource Details 2026-10-29/`. Other OS and
hardware combinations still need device testing; long-session/battery testing
was outside this panel pass.

The optimized local preview is **5.91 MiB** and passed strict ad-hoc signature
verification. Executable SHA-256: `56f0dbf5d44b89f4166bfb5168461e3f13d7b119e0b5a5fd6ca9f79fcb25fe91`. It is open at
1280×900 with Catppuccin and live monitoring. No public push, release, tag or
version bump was performed, and no private Pro implementation/assets were used.


## Process-inspector card spacing — 29 October 2026

The process sheet's flex layout allowed cards with overflow clipping to shrink
below their content height. This hid lower information rows, the root field and
bottom padding on longer inspectors. Direct sheet sections now retain their
natural height and the sheet scrolls. CPU/memory summary cards use one shared gap
and align their bottom content; duplicate flex-gap/space-y margins were removed
from command content. Empty child content and the last environment entry no
longer add a second layer of vertical spacing. Shared shadcn primitives and
resource-sheet layouts were not changed.

Native review covered the preview's own inspector at 1280×900 (including parent,
disk I/O, command/root and the empty child state) and Chrome's populated child
table at 1120×700. Svelte diagnostics, production build, formatting, whitespace and
strict local signature checks passed. This was a styling-only pass; backend tests
were not rerun. Evidence is in `Documents/Codex/NeoHtop Process Spacing 2026-10-29/`.
Final executable SHA-256: `127f176cb920737d8c528cf93968265a39a05c646c9326af482fd3d0565298e1`. The preview was restored to 1280×900,
Catppuccin, an empty search and live monitoring. No public release or push.


## App bar, process toolbar and Settings — 29 October 2026

Global monitoring controls now live in the app bar: live/paused status, pause,
companion, the quick theme chooser and a compact application menu. The process
area has one toolbar for search, filters, List/Tree and the View column menu.
Matching counts, active conditions and family context appear only when relevant;
rows per page and pagination remain in the footer. Cards, table behavior,
inspectors, process icons and all 24 original themes are retained.

Separate native Settings (General/Appearance) and About windows use shared
shadcn components. Settings holds refresh cadence, tray visibility and close
behavior, plus the same searchable theme gallery as the quick chooser. Native
menus provide Settings (Command+, / Ctrl+,), About, update checks, process views
and standard editing/window commands. Palette changes also update native title
bars. Auxiliary windows hide on close, and macOS activation preserves their
focus rather than redirecting shortcuts to the monitor.

Preferences now use one native, serialized, persisted source with a shared
defaults file, legacy localStorage migration, validated leaf patches, revisioned
window synchronization and retryable save errors. Failed writes retain the
previous file and roll back the optimistic control. Monitoring/tray configuration
is owned by the application and follows committed preferences; it no longer
depends on toolbar component mounting. Theme selection remains compatible with
existing saved theme identifiers.

Validation: Svelte diagnostics report zero errors/warnings, all **23 Rust and
62 JavaScript tests** pass, repository formatting and whitespace checks pass,
and the optimized **6.02 MiB** native preview builds and passes strict local
ad-hoc signature verification. Native review covered both appearances, small
windows, cross-window changes, relaunch persistence, menu-bar preferences,
keep-running/reopen, independent Settings/About closing, search/view/columns,
resource-sheet navigation, shared pause and update checks. An activation issue
found with Command+W was corrected and rechecked.

Screenshots and validation receipts are in
`Documents/Codex/NeoHtop Settings 2026-10-29/`. Executable SHA-256:
`ce04f199da5019d7f5037d035a0783fa9fbc342be745732c1e0528c406b6d55d`. The preview is open at 1280×900 with Catppuccin, 3-second
refresh, a live unfiltered List, 15 rows and the menu-bar icon enabled; closing
the main window quits unless the user opts in to keep-running. Windows/Linux
native device testing and long-session/battery checks remain outside this pass.
No public push, release, tag or version bump; no private Pro code or assets used.
