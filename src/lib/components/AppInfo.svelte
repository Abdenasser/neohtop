<script lang="ts">
  import { getVersion } from "@tauri-apps/api/app";
  import { onMount } from "svelte";
  import ThemeSwitcher from "./ThemeSwitcher.svelte";
  import { faInfo } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";

  let version = "";
  let latestVersion = "";
  let showInfo = false;
  let hasUpdate = false;

  const ASCII_ART = `
    ███╗   ██╗███████╗ ██████╗ ██╗  ██╗████████╗ ██████╗ ██████╗ 
    ████╗  ██║██╔════╝██╔═══██╗██║  ██║╚══██╔══╝██╔═══██╗██╔══██╗
    ██╔██╗ ██║█████╗  ██║   ██║███████║   ██║   ██║   ██║██████╔╝
    ██║╚██╗██║██╔══╝  ██║   ██║██╔══██║   ██║   ██║   ██║██╔═══╝ 
    ██║ ╚████║███████╗╚██████╔╝██║  ██║   ██║   ╚██████╔╝██║     
    ╚═╝  ╚═══╝╚══════╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝    ╚═════╝ ╚═╝     
  `;

  const APP_INFO = {
    name: "NeoHtop",
    developer: "Abdenasser",
    github: "https://github.com/Abdenasser/neohtop",
    stack: ["Tauri", "Rust", "Svelte", "TypeScript"],
  };

  async function checkLatestVersion() {
    try {
      const response = await fetch(
        "https://api.github.com/repos/abdenasser/neohtop/releases/latest",
      );
      const data = await response.json();

      // Extract version number from tag (e.g., "1.0.6" from "macos-nightly-1.0.6")
      const versionMatch = data.tag_name.match(/\d+\.\d+\.\d+/);
      if (!versionMatch) {
        console.warn(
          "Unexpected version format in latest release:",
          data.tag_name,
        );
        return;
      }

      latestVersion = versionMatch[0];

      // Extract version number from current version
      const currentVersionMatch = version.match(/\d+\.\d+\.\d+/);
      if (!currentVersionMatch) {
        console.warn("Unexpected current version format:", version);
        return;
      }

      // Compare only the version numbers
      hasUpdate = currentVersionMatch[0] !== latestVersion;
    } catch (error) {
      console.error("Failed to check latest version:", error);
      latestVersion = "";
      hasUpdate = false;
    }
  }

  onMount(async () => {
    try {
      version = await getVersion();
      await checkLatestVersion();
    } catch (error) {
      console.error("Failed to initialize version info:", error);
      version = "";
    }
  });
</script>

<div class="app-info">
  <ThemeSwitcher />
  <button
    class:info-button={true}
    class:has-update={hasUpdate}
    on:click={() => (showInfo = !showInfo)}
    aria-label="Toggle app info"
  >
    <span class="icon" class:update-available={hasUpdate}>
      <Fa icon={faInfo} />
    </span>
  </button>

  {#if showInfo}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="info-panel" on:mouseleave={() => (showInfo = false)}>
      <div class="info-content">
        <pre class="ascii-art">{ASCII_ART}</pre>
        <div class="details">
          <div class="detail-row">
            <span>NeoHtop v{version}</span>
            {#if hasUpdate}
              <a
                href={`https://github.com/abdenasser/neohtop/releases/latest`}
                class="update-button"
                target="_blank"
                rel="noopener noreferrer"
              >
                Update to v{latestVersion}
              </a>
            {/if}
          </div>
          <div class="detail-row">
            <span class="label">app</span>
            <span class="separator">::</span>
            <span class="value">{APP_INFO.name}</span>
          </div>
          <div class="detail-row">
            <span class="label">source</span>
            <span class="separator">::</span>
            <a
              href={APP_INFO.github}
              class="value"
              target="_blank"
              rel="noopener noreferrer"
            >
              {APP_INFO.github}
            </a>
          </div>
          <div class="detail-row">
            <span class="label">stack</span>
            <span class="separator">::</span>
            <span class="value">{APP_INFO.stack.join(", ")}</span>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .app-info {
    display: flex;
    gap: 8px;
    position: relative;
  }

  .info-button,
  :global(.theme-button) {
    height: 31px;
    padding: 6px 12px;
    font-size: 12px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--text);
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .info-button:hover,
  :global(.theme-button:hover) {
    background: var(--surface1);
  }

  .icon {
    display: inline-flex;
    align-items: center;
    font-size: 10px;
    color: var(--subtext0);
  }

  .icon.update-available {
    color: var(--red);
  }

  .info-panel {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 4px;
    padding: 16px;
    background: var(--base);
    border: 1px solid var(--surface0);
    border-radius: 6px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    z-index: 100;
    min-width: 400px;
  }

  .info-content {
    display: flex;
    gap: 24px;
  }

  .ascii-art {
    font-family: monospace;
    font-size: 8px;
    line-height: 1;
    color: var(--mauve);
    margin: 0;
    padding: 0;
    white-space: pre;
  }

  .details {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 8px 0;
  }

  .detail-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-family: monospace;
    font-size: 13px;
  }

  .label {
    color: var(--green);
    min-width: 80px;
  }

  .separator {
    color: var(--subtext0);
  }

  .value {
    color: var(--text);
  }

  .detail-row span {
    color: var(--text);
    font-weight: 500;
  }

  .info-button.has-update {
    border-color: var(--red);
  }

  .info-button.has-update:hover {
    background: color-mix(in srgb, var(--red) 10%, transparent);
  }

  .info-button.has-update .icon {
    color: var(--red);
  }

  .update-button {
    display: inline-flex;
    align-items: center;
    padding: 4px 8px;
    font-size: 12px;
    color: var(--base);
    background: var(--red);
    border-radius: 4px;
    text-decoration: none;
    margin-left: 8px;
    transition: all 0.2s ease;
  }

  .update-button:hover {
    background: color-mix(in srgb, var(--red) 90%, white);
    transform: translateY(-1px);
  }
</style>
