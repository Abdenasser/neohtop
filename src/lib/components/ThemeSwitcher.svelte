<script lang="ts">
  import { themeStore } from "$lib/stores";
  import { overlayStore } from "$lib/stores/overlay";
  import { themes } from "$lib/definitions";
  import Fa from "svelte-fa";
  import {
    faChevronDown,
    faChevronRight,
    faChevronLeft,
  } from "@fortawesome/free-solid-svg-icons";
  import { platform } from "@tauri-apps/plugin-os";
  import { THEME_GROUPS } from "$lib/constants";
  import { onDestroy } from "svelte";

  let containerElement: HTMLDivElement;
  let overlayElement: HTMLDivElement;
  let optionsContainer: HTMLDivElement;
  let canScrollLeft = false;
  let canScrollRight = false;

  $: showMenu = $overlayStore === "theme";

  const themeGroups = [
    ...THEME_GROUPS,
    ...(platform() === "windows" || platform() === "macos"
      ? [
          {
            label: "Glassy",
            themes: ["glassy"],
          },
        ]
      : []),
  ];

  function updateOverlayPosition() {
    if (overlayElement && containerElement) {
      const toolbarContent = containerElement.closest(".toolbar-content");
      if (toolbarContent) {
        const toolbarRect = toolbarContent.getBoundingClientRect();
        const containerRect = containerElement.getBoundingClientRect();

        const leftOffset = containerRect.left - toolbarRect.left;
        const rightOffset = toolbarRect.right - containerRect.right;
        const topOffset = containerRect.top - toolbarRect.top;

        overlayElement.style.left = `-${leftOffset}px`;
        overlayElement.style.right = `-${rightOffset}px`;
        overlayElement.style.top = `-${topOffset}px`;
      }
    }
  }

  function updateScrollButtons() {
    if (optionsContainer) {
      canScrollLeft = optionsContainer.scrollLeft > 0;
      canScrollRight =
        optionsContainer.scrollLeft <
        optionsContainer.scrollWidth - optionsContainer.clientWidth;
    }
  }

  function scrollLeft() {
    if (optionsContainer) {
      optionsContainer.scrollBy({ left: -200, behavior: "smooth" });
      setTimeout(updateScrollButtons, 100);
    }
  }

  function scrollRight() {
    if (optionsContainer) {
      optionsContainer.scrollBy({ left: 200, behavior: "smooth" });
      setTimeout(updateScrollButtons, 100);
    }
  }

  function toggleThemeMenu(event: Event) {
    event.stopPropagation();
    if (showMenu) {
      overlayStore.close();
    } else {
      overlayStore.open("theme");
      setTimeout(() => {
        updateOverlayPosition();
        updateScrollButtons();
      }, 0);
    }
  }

  function selectTheme(themeName: string) {
    themeStore.setTheme(themeName);
    overlayStore.close();
  }

  function handleClickOutside(event: MouseEvent) {
    if (
      showMenu &&
      containerElement &&
      !containerElement.contains(event.target as Node)
    ) {
      overlayStore.close();
    }
  }

  function setupClickOutside() {
    if (typeof document !== "undefined") {
      document.addEventListener("click", handleClickOutside);
    }
  }

  function cleanupClickOutside() {
    if (typeof document !== "undefined") {
      document.removeEventListener("click", handleClickOutside);
    }
  }

  $: if (showMenu) {
    setTimeout(setupClickOutside, 0);
  } else {
    cleanupClickOutside();
  }

  onDestroy(() => {
    cleanupClickOutside();
  });
</script>

<div
  class="theme-switcher"
  class:active={showMenu}
  bind:this={containerElement}
>
  <button
    class="theme-button"
    class:active={showMenu}
    on:click={toggleThemeMenu}
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
    <div
      class="touchbar-full-overlay"
      bind:this={overlayElement}
      on:click={() => overlayStore.close()}
      on:keydown={(e) => e.key === "Escape" && overlayStore.close()}
      role="dialog"
      aria-label="Theme selection"
      tabindex="-1"
    >
      {#if canScrollLeft}
        <button
          class="scroll-chevron scroll-left"
          on:click|stopPropagation={scrollLeft}
        >
          <Fa icon={faChevronLeft} />
        </button>
      {/if}

      <div
        class="touchbar-horizontal-options"
        bind:this={optionsContainer}
        on:scroll={updateScrollButtons}
      >
        {#each themeGroups as group}
          {#each group.themes as themeName}
            {@const theme = themes[themeName]}
            <button
              class="touchbar-option"
              class:active={$themeStore.name === theme.name}
              on:click|stopPropagation={() => selectTheme(theme.name)}
              title={theme.label}
            >
              <div class="theme-preview" style:background={theme.colors.base}>
                <div
                  class="preview-color"
                  style:background={theme.colors.blue}
                ></div>
                <div
                  class="preview-color"
                  style:background={theme.colors.red}
                ></div>
                <div
                  class="preview-color"
                  style:background={theme.colors.green}
                ></div>
              </div>
              <span class="theme-label">{theme.label}</span>
            </button>
          {/each}
        {/each}
      </div>

      {#if canScrollRight}
        <button
          class="scroll-chevron scroll-right"
          on:click|stopPropagation={scrollRight}
        >
          <Fa icon={faChevronRight} />
        </button>
      {/if}
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
    padding: 0 12px;
    height: 28px;
    font-size: 12px;
    color: var(--text);
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    box-sizing: border-box;
  }

  .theme-button:hover {
    background: var(--surface1);
    border-color: var(--blue);
  }

  .theme-button.active {
    background: var(--surface1);
    border-color: var(--blue);
  }

  .icon {
    font-size: 10px;
    color: var(--subtext0);
  }

  .touchbar-full-overlay {
    position: absolute;
    top: -0px;
    height: 44px;
    background: var(--mantle);
    border: none;
    border-radius: 0;
    box-shadow: none;
    z-index: 1000;
    display: flex;
    align-items: center;
    padding: 0 8px;
    gap: 8px;
  }

  .scroll-chevron {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    color: var(--text);
    cursor: pointer;
    transition: all 0.15s ease;
    flex-shrink: 0;
    font-size: 10px;
    animation: optionSlideIn 0.2s ease-out;
    animation-fill-mode: both;
    animation-delay: 0ms;
  }

  .scroll-chevron:hover {
    background: var(--surface1);
    border-color: var(--blue);
  }

  .touchbar-horizontal-options {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
    overflow-x: auto;
    overflow-y: hidden;
    scrollbar-width: none;
    -ms-overflow-style: none;
    padding: 0 4px;
  }

  .touchbar-horizontal-options::-webkit-scrollbar {
    display: none;
  }

  .touchbar-option {
    padding: 0 8px;
    height: 28px;
    font-size: 12px;
    color: var(--text);
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.15s ease;
    white-space: nowrap;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    min-width: fit-content;
    box-sizing: border-box;
    animation: optionSlideIn 0.2s ease-out;
    animation-fill-mode: both;
  }

  .touchbar-option:nth-child(1) {
    animation-delay: 20ms;
  }
  .touchbar-option:nth-child(2) {
    animation-delay: 40ms;
  }
  .touchbar-option:nth-child(3) {
    animation-delay: 60ms;
  }
  .touchbar-option:nth-child(4) {
    animation-delay: 80ms;
  }
  .touchbar-option:nth-child(5) {
    animation-delay: 100ms;
  }
  .touchbar-option:nth-child(6) {
    animation-delay: 120ms;
  }
  .touchbar-option:nth-child(7) {
    animation-delay: 140ms;
  }
  .touchbar-option:nth-child(8) {
    animation-delay: 160ms;
  }
  .touchbar-option:nth-child(9) {
    animation-delay: 180ms;
  }
  .touchbar-option:nth-child(10) {
    animation-delay: 200ms;
  }

  @keyframes optionSlideIn {
    from {
      opacity: 0;
      transform: translateY(-8px) scale(0.9);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .touchbar-option:hover:not(.disabled) {
    background: var(--surface1);
    border-color: var(--blue);
  }

  .touchbar-option.active {
    background: var(--blue);
    color: var(--base);
    border-color: var(--blue);
  }

  .theme-preview {
    display: flex;
    gap: 2px;
    padding: 2px;
    border-radius: 3px;
    border: 1px solid var(--surface1);
  }

  .preview-color {
    width: 8px;
    height: 8px;
    border-radius: 1px;
  }

  .theme-label {
    font-size: 12px;
  }

  .current-theme {
    display: flex;
    align-items: center;
    gap: 8px;
  }
</style>
