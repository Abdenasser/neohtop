<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-shell";
  import { Modal } from "$lib/components";
  import type { Process, VTReport } from "$lib/types";

  export let show = false;
  export let process: Process | null = null;
  export let onClose: () => void;

  // ── State ──────────────────────────────────────────────────────────────────
  const API_KEY_STORAGE = "neohtop_vt_api_key";

  let apiKey: string = "";
  let phase: "idle" | "hashing" | "querying" | "done" | "error" = "idle";
  let hashValue = "";
  let report: VTReport | null = null;
  let errorMessage = "";

  // Load saved API key on mount
  $: if (show) {
    apiKey = localStorage.getItem(API_KEY_STORAGE) ?? "";
    // Reset result state when modal is opened for a new process
    phase = "idle";
    hashValue = "";
    report = null;
    errorMessage = "";
  }

  // ── Scan flow ──────────────────────────────────────────────────────────────
  async function startScan() {
    if (!process) return;

    // Persist API key across sessions
    if (apiKey.trim()) {
      localStorage.setItem(API_KEY_STORAGE, apiKey.trim());
    }

    try {
      // Step 1: Hash the executable
      phase = "hashing";
      errorMessage = "";
      hashValue = await invoke<string>("hash_process", { pid: process.pid });

      // Step 2: Query VirusTotal
      phase = "querying";
      report = await invoke<VTReport>("check_virustotal_hash", {
        hash: hashValue,
        apiKey: apiKey.trim(),
      });

      phase = "done";
    } catch (err: unknown) {
      errorMessage = typeof err === "string" ? err : String(err);
      phase = "error";
    }
  }

  function handleClose() {
    phase = "idle";
    report = null;
    errorMessage = "";
    onClose();
  }

  // ── Helpers ────────────────────────────────────────────────────────────────
  function verdictLabel(v: VTReport["verdict"]): string {
    const labels: Record<VTReport["verdict"], string> = {
      clean: "Clean",
      malicious: "Malicious",
      suspicious: "Suspicious",
      unknown: "Not Found",
    };
    return labels[v];
  }
</script>

<Modal
  {show}
  title={`VirusTotal Scan — ${process?.name ?? "Unknown"}`}
  maxWidth="520px"
  onClose={handleClose}
>
  {#if process}
    <div class="vt-content">
      <!-- API Key input -->
      {#if phase === "idle" || phase === "error"}
        <div class="api-section">
          <label for="vt-api-key" class="field-label">
            VirusTotal API Key
            <button
              class="get-key-link"
              on:click={() => open("https://www.virustotal.com/gui/my-apikey")}
              title="Get a free API key">Get free key ↗</button
            >
          </label>
          <input
            id="vt-api-key"
            type="password"
            class="api-input"
            bind:value={apiKey}
            placeholder="Paste your VirusTotal API key…"
            on:keydown={(e) => e.key === "Enter" && startScan()}
          />
        </div>

        <!-- Process target summary -->
        <div class="target-info">
          <span class="target-pid">PID {process.pid}</span>
          <span class="target-sep">·</span>
          <span class="target-name">{process.name}</span>
          {#if process.command}
            <span class="target-cmd">{process.command.split(" ")[0]}</span>
          {/if}
        </div>

        {#if phase === "error"}
          <div class="error-box">
            <span class="error-icon">⚠</span>
            <span>{errorMessage}</span>
          </div>
        {/if}

        <div class="actions">
          <button class="btn-secondary" on:click={handleClose}>Cancel</button>
          <button
            class="btn-scan"
            on:click={startScan}
            disabled={!apiKey.trim()}
          >
            🛡 Scan with VirusTotal
          </button>
        </div>

        <!-- Loading states -->
      {:else if phase === "hashing" || phase === "querying"}
        <div class="loading-state">
          <div class="spinner-large"></div>
          <p class="loading-label">
            {#if phase === "hashing"}
              Computing SHA-256 hash of <strong>{process.name}</strong>…
            {:else}
              Querying VirusTotal database…
            {/if}
          </p>
          <p class="loading-sub">This may take a few seconds.</p>
        </div>

        <!-- Result -->
      {:else if phase === "done" && report}
        {@const isUnknown = report.verdict === "unknown"}

        <!-- Verdict badge -->
        <div class="verdict-banner verdict-{report.verdict}">
          <span class="verdict-icon">
            {#if report.verdict === "clean"}✅
            {:else if report.verdict === "malicious"}🚨
            {:else if report.verdict === "suspicious"}⚠️
            {:else}❓
            {/if}
          </span>
          <span class="verdict-text">{verdictLabel(report.verdict)}</span>
          {#if isUnknown}
            <span class="verdict-sub">File not in VirusTotal database</span>
          {:else}
            <span class="verdict-sub">
              {report.malicious} / {report.total} engines detected a threat
            </span>
          {/if}
        </div>

        <!-- Hash display -->
        <div class="hash-row">
          <span class="hash-label">SHA-256</span>
          <code class="hash-value">{report.hash}</code>
        </div>

        <!-- Stats table (only if we have data) -->
        {#if !isUnknown}
          <div class="stats-grid">
            <div class="stat-box stat-malicious">
              <div class="stat-num">{report.malicious}</div>
              <div class="stat-cap">Malicious</div>
            </div>
            <div class="stat-box stat-suspicious">
              <div class="stat-num">{report.suspicious}</div>
              <div class="stat-cap">Suspicious</div>
            </div>
            <div class="stat-box stat-clean">
              <div class="stat-num">{report.undetected}</div>
              <div class="stat-cap">Undetected</div>
            </div>
            <div class="stat-box">
              <div class="stat-num">{report.total}</div>
              <div class="stat-cap">Total Engines</div>
            </div>
          </div>
        {/if}

        <!-- Actions -->
        <div class="actions">
          <button
            class="btn-secondary"
            on:click={() => {
              phase = "idle";
              report = null;
            }}
          >
            ← Scan Again
          </button>
          <button
            class="btn-vt-link"
            on:click={() => open(report?.permalink ?? "")}
          >
            View on VirusTotal ↗
          </button>
          <button class="btn-secondary" on:click={handleClose}>Close</button>
        </div>
      {/if}
    </div>
  {/if}
</Modal>

<style>
  .vt-content {
    display: flex;
    flex-direction: column;
    gap: 16px;
    font-size: 13px;
    color: var(--text);
  }

  /* ── API key ── */
  .api-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .field-label {
    display: flex;
    align-items: center;
    justify-content: space-between;
    color: var(--subtext0);
    font-size: 12px;
    font-weight: 500;
  }

  .get-key-link {
    background: none;
    border: none;
    padding: 0;
    cursor: pointer;
    color: var(--blue);
    text-decoration: none;
    font-size: 11px;
  }

  .get-key-link:hover {
    text-decoration: underline;
  }

  .api-input {
    width: 100%;
    box-sizing: border-box;
    padding: 8px 12px;
    background: var(--mantle);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    color: var(--text);
    font-size: 13px;
    outline: none;
    transition: border-color 0.2s;
  }

  .api-input:focus {
    border-color: var(--blue);
  }

  /* ── Target summary ── */
  .target-info {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    background: var(--surface0);
    border-radius: 6px;
    flex-wrap: wrap;
  }

  .target-pid {
    color: var(--blue);
    font-weight: 600;
    font-size: 12px;
  }

  .target-sep {
    color: var(--subtext0);
  }

  .target-name {
    color: var(--text);
    font-weight: 500;
  }

  .target-cmd {
    color: var(--subtext0);
    font-size: 11px;
    word-break: break-all;
  }

  /* ── Error box ── */
  .error-box {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    padding: 10px 12px;
    background: color-mix(in srgb, var(--red) 15%, var(--surface0));
    border: 1px solid color-mix(in srgb, var(--red) 40%, transparent);
    border-radius: 6px;
    color: var(--red);
    line-height: 1.5;
  }

  .error-icon {
    flex-shrink: 0;
    font-size: 14px;
  }

  /* ── Loading ── */
  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    padding: 24px 0;
  }

  .spinner-large {
    width: 36px;
    height: 36px;
    border: 3px solid var(--surface1);
    border-top-color: var(--blue);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .loading-label {
    margin: 0;
    color: var(--text);
    font-size: 14px;
    text-align: center;
  }

  .loading-sub {
    margin: 0;
    color: var(--subtext0);
    font-size: 12px;
  }

  /* ── Verdict banner ── */
  .verdict-banner {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 14px 16px;
    border-radius: 8px;
    border: 1px solid;
  }

  .verdict-clean {
    background: color-mix(in srgb, var(--green) 12%, var(--surface0));
    border-color: color-mix(in srgb, var(--green) 35%, transparent);
  }

  .verdict-malicious {
    background: color-mix(in srgb, var(--red) 15%, var(--surface0));
    border-color: color-mix(in srgb, var(--red) 40%, transparent);
  }

  .verdict-suspicious {
    background: color-mix(in srgb, var(--yellow) 12%, var(--surface0));
    border-color: color-mix(in srgb, var(--yellow) 35%, transparent);
  }

  .verdict-unknown {
    background: var(--surface0);
    border-color: var(--surface1);
  }

  .verdict-icon {
    font-size: 20px;
    flex-shrink: 0;
  }

  .verdict-text {
    font-size: 16px;
    font-weight: 700;
  }

  .verdict-clean .verdict-text {
    color: var(--green);
  }
  .verdict-malicious .verdict-text {
    color: var(--red);
  }
  .verdict-suspicious .verdict-text {
    color: var(--yellow);
  }
  .verdict-unknown .verdict-text {
    color: var(--subtext0);
  }

  .verdict-sub {
    font-size: 12px;
    color: var(--subtext0);
    margin-left: auto;
  }

  /* ── Hash row ── */
  .hash-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: var(--mantle);
    border-radius: 6px;
    overflow: hidden;
  }

  .hash-label {
    flex-shrink: 0;
    font-size: 11px;
    font-weight: 600;
    color: var(--subtext0);
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }

  .hash-value {
    font-family: ui-monospace, "Cascadia Code", "Fira Code", monospace;
    font-size: 11px;
    color: var(--blue);
    word-break: break-all;
    overflow-wrap: anywhere;
  }

  /* ── Stats grid ── */
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 8px;
  }

  .stat-box {
    background: var(--surface0);
    border-radius: 6px;
    padding: 10px 8px;
    text-align: center;
  }

  .stat-num {
    font-size: 20px;
    font-weight: 700;
    color: var(--text);
  }

  .stat-cap {
    font-size: 10px;
    color: var(--subtext0);
    margin-top: 2px;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .stat-malicious .stat-num {
    color: var(--red);
  }
  .stat-suspicious .stat-num {
    color: var(--yellow);
  }
  .stat-clean .stat-num {
    color: var(--green);
  }

  /* ── Actions row ── */
  .actions {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }

  .btn-secondary {
    padding: 7px 14px;
    font-size: 13px;
    color: var(--text);
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    cursor: pointer;
    transition: background 0.15s;
  }

  .btn-secondary:hover {
    background: var(--surface1);
  }

  .btn-scan {
    padding: 7px 16px;
    font-size: 13px;
    font-weight: 500;
    color: var(--base);
    background: var(--blue);
    border: none;
    border-radius: 6px;
    cursor: pointer;
    transition: opacity 0.15s;
  }

  .btn-scan:hover {
    opacity: 0.88;
  }

  .btn-scan:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .btn-vt-link {
    padding: 7px 14px;
    font-size: 13px;
    color: var(--teal);
    background: color-mix(in srgb, var(--teal) 12%, var(--surface0));
    border: 1px solid color-mix(in srgb, var(--teal) 30%, transparent);
    border-radius: 6px;
    text-decoration: none;
    transition: background 0.15s;
  }

  .btn-vt-link:hover {
    background: color-mix(in srgb, var(--teal) 20%, var(--surface0));
  }
</style>
