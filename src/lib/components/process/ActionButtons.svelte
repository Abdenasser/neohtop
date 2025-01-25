<script lang="ts">
  import {
    faThumbtack,
    faInfoCircle,
    faXmark,
    faFolderOpen,
  } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import type { Process } from "$lib/types";

  export let process: Process;
  export let isPinned: boolean;
  export let onTogglePin: (command: string) => void;
  export let openProcessDirectory: (process: Process) => void;
  export let onShowDetails: (process: Process) => void;
  export let onKillProcess: (process: Process) => void;
</script>

<td class="col-actions">
  <div class="action-buttons">
    <button
      class="btn-action pin-btn"
      class:pinned={isPinned}
      on:click={() => onTogglePin(process.command)}
      title={isPinned ? "Unpin" : "Pin"}
    >
      <Fa icon={faThumbtack} />
    </button>
    <button
      class="btn-action open-btn"
      on:click={() => openProcessDirectory(process)}
      title="Open Directory"
    >
      <Fa icon={faFolderOpen} />
    </button>
    <button
      class="btn-action info-btn"
      on:click={() => onShowDetails(process)}
      title="Show Details"
    >
      <Fa icon={faInfoCircle} />
    </button>
    <button
      class="btn-action kill-btn"
      on:click={() => onKillProcess(process)}
      title="End Process"
    >
      <Fa icon={faXmark} />
    </button>
  </div>
</td>

<style>
  td {
    padding: 6px 12px;
    border-bottom: 1px solid var(--surface0);
    color: var(--text);
    z-index: 1;
  }
  .col-actions {
    position: sticky;
    right: 0;
    z-index: 2;
    background: var(--base);
    border-left: 1px solid var(--surface0);
    width: 120px;
  }

  .action-buttons {
    display: flex;
    gap: 4px;
    align-items: center;
    justify-content: center;
  }

  .btn-action {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border: none;
    cursor: pointer;
    background: transparent;
    border-radius: 6px;
    width: 28px;
    height: 28px;
    transition: all 0.2s ease;
    position: relative;
    overflow: hidden;
  }

  .btn-action::before {
    content: "";
    position: absolute;
    inset: 0;
    opacity: 0.1;
    transition: opacity 0.2s ease;
  }

  .btn-action:hover::before {
    opacity: 0.15;
  }

  .pin-btn {
    color: var(--sapphire);
  }

  .pin-btn::before {
    background: var(--sapphire);
  }

  .pin-btn.pinned {
    color: var(--blue);
    transform: rotate(45deg);
  }

  .pin-btn.pinned::before {
    background: var(--blue);
    opacity: 0.15;
  }

  .open-btn {
    color: var(--green);
  }

  .open-btn::before {
    background: var(--green);
  }

  .info-btn {
    color: var(--lavender);
  }

  .info-btn::before {
    background: var(--lavender);
  }

  .kill-btn {
    color: var(--red);
    border: 1px solid color-mix(in srgb, var(--red) 30%, transparent);
  }

  .kill-btn:hover {
    color: var(--base);
    background: var(--red);
  }

  .kill-btn:hover::before {
    opacity: 1;
  }

  .btn-action:hover {
    box-shadow: 0 0 12px color-mix(in srgb, currentColor 20%, transparent);
  }

  .btn-action:active {
    transform: translateY(1px);
  }

  .pin-btn.pinned:active {
    transform: rotate(45deg) translateY(1px);
  }

  .btn-action:focus {
    outline: none;
    box-shadow: 0 0 0 2px color-mix(in srgb, currentColor 30%, transparent);
  }

  .btn-action:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-action:disabled:hover {
    transform: none;
    box-shadow: none;
  }

  .btn-action:disabled::before {
    display: none;
  }
</style>
