<script lang="ts">
  import { themeStore } from "$lib/stores";
  import { themes } from "$lib/styles";
  import { fade } from "svelte/transition";
  import Fa from "svelte-fa";
  import {
    faChevronDown,
    faChevronRight,
  } from "@fortawesome/free-solid-svg-icons";

  let showMenu = false;

  const themeGroups = [
    {
      label: "Dark",
      themes: ["catppuccin", "dracula", "monokaiPro", "tokyoNight"],
    },
    {
      label: "Light",
      themes: ["githubLight", "solarizedLight", "oneLight"],
    },
    {
      label: "Warm",
      themes: ["gruvbox"],
    },
    {
      label: "Cool",
      themes: ["nord", "oneDark"],
    },
    {
      label: "Fun",
      themes: [
        "bubblegum",
        "rosePine",
        "cottonCandy",
        "synthwave",
        "candyfloss",
      ],
    },
    {
      label: "Retro",
      themes: ["terminal", "amber", "ibmPC"],
    },
    {
      label: "Accessibility",
      themes: ["highContrast"],
    },
  ];
</script>

<div class="theme-switcher">
  <button
    class="theme-button"
    on:click={() => (showMenu = !showMenu)}
    aria-label="Toggle theme menu"
  >
    <div class="current-theme">
      <div class="theme-preview" style:background={$themeStore.colors.base}>
        <div
          class="preview-color"
          style:background={$themeStore.colors.blue}
        ></div>
        <div
          class="preview-color"
          style:background={$themeStore.colors.red}
        ></div>
        <div
          class="preview-color"
          style:background={$themeStore.colors.green}
        ></div>
      </div>
    </div>
    <span class="icon">
      {#if showMenu}
        <Fa icon={faChevronDown} />
      {:else}
        <Fa icon={faChevronRight} />
      {/if}
    </span>
  </button>

  {#if showMenu}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore element_invalid_self_closing_tag -->
    <div
      class="theme-menu"
      transition:fade={{ duration: 100 }}
      on:mouseleave={() => (showMenu = false)}
    >
      {#each themeGroups as group}
        <div class="theme-group">
          <div class="group-label">{group.label}</div>
          {#each group.themes as themeName}
            {@const theme = themes[themeName]}
            <button
              class="theme-option"
              class:active={$themeStore.name === theme.name}
              on:click={() => {
                themeStore.setTheme(theme.name);
                showMenu = false;
              }}
            >
              <div class="theme-preview" style:background={theme.colors.base}>
                <div
                  class="preview-color"
                  style:background={theme.colors.blue}
                />
                <div
                  class="preview-color"
                  style:background={theme.colors.red}
                />
                <div
                  class="preview-color"
                  style:background={theme.colors.green}
                />
              </div>
              {theme.label}
            </button>
          {/each}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .theme-switcher {
    position: relative;
  }

  .theme-button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    font-size: 12px;
    color: var(--text);
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .theme-button:hover {
    background: var(--surface1);
  }

  .icon {
    font-size: 10px;
    color: var(--subtext0);
  }

  .theme-menu {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 4px;
    padding: 8px;
    background: var(--base);
    border: 1px solid var(--surface0);
    border-radius: 6px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    z-index: 100;
    min-width: 200px;
    max-height: 400px;
    overflow-y: auto;
    scroll-behavior: smooth;
    scrollbar-width: thin;
    scrollbar-color: var(--surface2) var(--surface0);
  }

  .theme-menu::-webkit-scrollbar {
    width: 8px;
  }

  .theme-menu::-webkit-scrollbar-track {
    background: var(--surface0);
    border-radius: 4px;
  }

  .theme-menu::-webkit-scrollbar-thumb {
    background: var(--surface2);
    border-radius: 4px;
  }

  .theme-menu::-webkit-scrollbar-thumb:hover {
    background: var(--overlay0);
  }

  .theme-option {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    padding: 8px;
    border: none;
    background: transparent;
    color: var(--text);
    font-size: 13px;
    cursor: pointer;
    border-radius: 4px;
  }

  .theme-option:hover {
    background: var(--surface0);
  }

  .theme-option.active {
    background: var(--surface0);
  }

  .theme-preview {
    display: flex;
    gap: 4px;
    padding: 4px;
    border-radius: 4px;
    border: 1px solid var(--surface1);
  }

  .preview-color {
    width: 12px;
    height: 12px;
    border-radius: 2px;
  }

  .current-theme {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .theme-group {
    padding: 8px 0;
  }

  .theme-group:not(:last-child) {
    border-bottom: 1px solid var(--surface0);
  }

  .group-label {
    padding: 4px 8px;
    font-size: 11px;
    font-weight: 500;
    color: var(--subtext0);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
</style>
