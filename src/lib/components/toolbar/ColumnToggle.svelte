<script lang="ts">
  import Fa from "svelte-fa";
  import {
    faChevronDown,
    faChevronRight,
  } from "@fortawesome/free-solid-svg-icons";
  import { settingsStore } from "$lib/stores";

  export let columns: Array<{
    id: string;
    label: string;
    visible: boolean;
    required?: boolean;
  }>;

  let showColumnMenu = false;

  function handleColumnVisibilityChange(columnId: string, visible: boolean) {
    settingsStore.updateConfig({
      appearance: {
        columnVisibility: {
          ...$settingsStore.appearance.columnVisibility,
          [columnId]: visible,
        },
      },
    });
  }
</script>

<div class="column-toggle">
  <button
    class="btn-toggle"
    on:click={() => (showColumnMenu = !showColumnMenu)}
    aria-label="Toggle columns"
  >
    Columns
    <span class="icon">
      {#if showColumnMenu}
        <Fa icon={faChevronDown} />
      {:else}
        <Fa icon={faChevronRight} />
      {/if}
    </span>
  </button>

  {#if showColumnMenu}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="column-menu" on:mouseleave={() => (showColumnMenu = false)}>
      {#each columns as column}
        <label class="column-option">
          <input
            type="checkbox"
            checked={column.visible}
            disabled={column.required}
            on:change={(e) =>
              handleColumnVisibilityChange(column.id, e.currentTarget.checked)}
          />
          <span>{column.label}</span>
        </label>
      {/each}
    </div>
  {/if}
</div>

<style>
  .column-toggle {
    position: relative;
  }

  .btn-toggle {
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
    min-width: 89px;
  }

  .btn-toggle:hover {
    background: var(--surface1);
  }

  .icon {
    font-size: 10px;
    color: var(--subtext0);
  }

  .column-menu {
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
    max-height: 300px;
    overflow-y: auto;
    min-width: 200px;
  }

  .column-menu::-webkit-scrollbar {
    width: 8px;
  }

  .column-menu::-webkit-scrollbar-track {
    background: var(--mantle);
    border-radius: 4px;
  }

  .column-menu::-webkit-scrollbar-thumb {
    background: var(--surface2);
    border-radius: 4px;
  }

  .column-menu::-webkit-scrollbar-thumb:hover {
    background: var(--surface1);
  }

  .column-option {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    font-size: 12px;
    color: var(--text);
    cursor: pointer;
    border-radius: 4px;
  }

  .column-option:hover {
    background: var(--surface0);
  }

  .column-option input[type="checkbox"] {
    appearance: none;
    width: 16px;
    height: 16px;
    border: 1px solid var(--surface2);
    border-radius: 4px;
    background: var(--mantle);
    cursor: pointer;
    position: relative;
  }

  .column-option input[type="checkbox"]:checked {
    background: var(--blue);
    border-color: var(--blue);
  }

  .column-option input[type="checkbox"]:checked::after {
    content: "✓";
    position: absolute;
    color: var(--base);
    font-size: 12px;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  .column-option input[type="checkbox"]:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
