<script lang="ts">
  import { STATUS_OPTIONS } from "$lib/constants";
  import type { AppConfig } from "$lib/types";
  import { settingsStore } from "$lib/stores/index";
  import { overlayStore } from "$lib/stores/overlay";
  import { onDestroy } from "svelte";

  export let statusFilter: string = "all";

  let containerElement: HTMLDivElement;
  let overlayElement: HTMLDivElement;

  $: isExpanded = $overlayStore === "status";

  function updateBehaviorConfig(key: keyof AppConfig["behavior"], value: any) {
    settingsStore.updateConfig({
      behavior: {
        ...$settingsStore.behavior,
        [key]: value,
      },
    });
  }

  function selectOption(value: string) {
    statusFilter = value;
    updateBehaviorConfig("defaultStatusFilter", statusFilter);
    overlayStore.close();
  }

  function getCurrentLabel() {
    return (
      STATUS_OPTIONS.find((opt) => opt.value === statusFilter)?.label ||
      "All Statuses"
    );
  }

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

  function toggleExpanded(event: Event) {
    event.stopPropagation();
    if (isExpanded) {
      overlayStore.close();
    } else {
      overlayStore.open("status");
      setTimeout(updateOverlayPosition, 0);
    }
  }

  function handleClickOutside(event: MouseEvent) {
    if (
      isExpanded &&
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

  $: if (isExpanded) {
    setTimeout(setupClickOutside, 0);
  } else {
    cleanupClickOutside();
  }

  onDestroy(() => {
    cleanupClickOutside();
  });
</script>

<div
  class="status-filter"
  class:active={isExpanded}
  bind:this={containerElement}
>
  <button
    class="touchbar-trigger"
    class:active={isExpanded}
    on:click={toggleExpanded}
  >
    Status: {getCurrentLabel()}
  </button>

  {#if isExpanded}
    <div
      class="touchbar-full-overlay"
      bind:this={overlayElement}
      on:click={() => overlayStore.close()}
      on:keydown={(e) => e.key === "Escape" && overlayStore.close()}
      role="dialog"
      aria-label="Status filter options"
      tabindex="-1"
    >
      <div class="touchbar-horizontal-options">
        {#each STATUS_OPTIONS as option}
          <button
            class="touchbar-option"
            class:active={option.value === statusFilter}
            on:click|stopPropagation={() => selectOption(option.value)}
          >
            {option.label}
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .status-filter {
    display: flex;
    align-items: center;
    position: relative;
  }

  .touchbar-trigger {
    height: 28px;
    padding: 0 12px;
    border: 1px solid var(--surface1);
    border-radius: 6px;
    background: var(--surface0);
    color: var(--text);
    font-size: 13px;
    cursor: pointer;
    transition: all 0.2s ease;
    white-space: nowrap;
  }

  .touchbar-trigger:hover {
    background-color: var(--surface1);
    border-color: var(--blue);
  }

  .touchbar-trigger.active {
    background: var(--surface1);
    border-color: var(--blue);
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
    padding: 0 12px;
    gap: 12px;
  }

  .touchbar-horizontal-options {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    width: 100%;
    overflow-x: auto;
    overflow-y: hidden;
    scrollbar-width: none;
    -ms-overflow-style: none;
  }

  .touchbar-horizontal-options::-webkit-scrollbar {
    display: none;
  }

  .touchbar-option {
    padding: 0 12px;
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
    min-width: fit-content;
    box-sizing: border-box;
    animation: optionSlideIn 0.2s ease-out;
    animation-fill-mode: both;
  }

  .touchbar-option:nth-child(1) {
    animation-delay: 0ms;
  }
  .touchbar-option:nth-child(2) {
    animation-delay: 40ms;
  }
  .touchbar-option:nth-child(3) {
    animation-delay: 80ms;
  }
  .touchbar-option:nth-child(4) {
    animation-delay: 120ms;
  }
  .touchbar-option:nth-child(5) {
    animation-delay: 160ms;
  }
  .touchbar-option:nth-child(6) {
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
</style>
