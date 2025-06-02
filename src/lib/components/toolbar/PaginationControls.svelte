<script lang="ts">
  import type { AppConfig } from "$lib/types";
  import { settingsStore } from "$lib/stores/index";
  import { overlayStore } from "$lib/stores/overlay";
  import { ITEMS_PER_PAGE_OPTIONS } from "$lib/constants";
  import { onDestroy } from "svelte";

  export let itemsPerPage: number;
  export let currentPage: number;
  export let totalPages: number;
  export let totalResults: number;

  let containerElement: HTMLDivElement;
  let overlayElement: HTMLDivElement;

  $: isExpanded = $overlayStore === "pagination";

  function changePage(page: number) {
    if (page >= 1 && page <= totalPages) {
      currentPage = page;
    }
  }

  function updateBehaviorConfig(key: keyof AppConfig["behavior"], value: any) {
    settingsStore.updateConfig({
      behavior: {
        ...$settingsStore.behavior,
        [key]: value,
      },
    });
  }

  function selectItemsPerPage(value: number) {
    itemsPerPage = value;
    updateBehaviorConfig("itemsPerPage", itemsPerPage);
    overlayStore.close();
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
      overlayStore.open("pagination");
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

<div class="pagination-controls">
  <div
    class="pagination-per-page"
    class:active={isExpanded}
    bind:this={containerElement}
  >
    <button
      class="touchbar-trigger"
      class:active={isExpanded}
      on:click={toggleExpanded}
    >
      {itemsPerPage} per page
    </button>

    {#if isExpanded}
      <div
        class="touchbar-full-overlay"
        bind:this={overlayElement}
        on:click={() => overlayStore.close()}
        on:keydown={(e) => e.key === "Escape" && overlayStore.close()}
        role="dialog"
        aria-label="Pagination controls"
        tabindex="-1"
      >
        <div class="touchbar-horizontal-options">
          {#each ITEMS_PER_PAGE_OPTIONS as option}
            <button
              class="touchbar-option"
              class:active={option === itemsPerPage}
              on:click|stopPropagation={() => selectItemsPerPage(option)}
            >
              {option}
            </button>
          {/each}
        </div>
      </div>
    {/if}
  </div>

  <div class="pagination">
    <button
      class="btn-page"
      disabled={currentPage === 1}
      on:click={() => changePage(1)}
    >
      ««
    </button>
    <button
      class="btn-page"
      disabled={currentPage === 1}
      on:click={() => changePage(currentPage - 1)}
    >
      «
    </button>
    <div class="page-info">
      <span>Page {currentPage} of {totalPages}</span>
      <span class="results-info">({totalResults} processes)</span>
    </div>
    <button
      class="btn-page"
      disabled={currentPage === totalPages}
      on:click={() => changePage(currentPage + 1)}
    >
      »
    </button>
    <button
      class="btn-page"
      disabled={currentPage === totalPages}
      on:click={() => changePage(totalPages)}
    >
      »»
    </button>
  </div>
</div>

<style>
  .pagination-controls {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .pagination-per-page {
    display: flex;
    align-items: center;
    position: relative;
  }

  .touchbar-trigger {
    padding: 6px 12px;
    height: 28px;
    font-size: 12px;
    color: var(--text);
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    white-space: nowrap;
    display: flex;
    align-items: center;
    box-sizing: border-box;
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

  .pagination {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .btn-page {
    padding: 6px 10px;
    font-size: 12px;
    color: var(--text);
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-page:hover:not(:disabled) {
    background: var(--surface1);
  }

  .btn-page:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .page-info {
    font-size: 12px;
    color: var(--subtext0);
    display: flex;
    flex-direction: column;
    align-items: center;
    flex-shrink: 0;
  }

  .page-info span {
    display: block;
  }

  .results-info {
    color: var(--overlay0);
  }
</style>
