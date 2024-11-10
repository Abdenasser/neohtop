<script lang="ts">
  import {
    faThumbtack,
    faInfoCircle,
    faXmark,
  } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";
  import type { Process, Column } from "$lib/types";
  import * as SimpleIcons from "simple-icons";

  export let processes: Process[];
  export let columns: Column[];
  export let systemStats: { memory_total: number } | null;
  export let sortConfig: { field: keyof Process; direction: "asc" | "desc" };
  export let pinnedProcesses: Set<string>;

  export let onToggleSort: (field: keyof Process) => void;
  export let onTogglePin: (command: string) => void;
  export let onShowDetails: (process: Process) => void;
  export let onKillProcess: (process: Process) => void;

  function getSortIndicator(field: keyof Process) {
    if (sortConfig.field !== field) return "↕";
    return sortConfig.direction === "asc" ? "↑" : "↓";
  }

  function handleImageError(event: Event) {
    const img = event.target as HTMLImageElement;
    img.src = getIconForProcess("default"); // Fallback to default icon
    img.onerror = null; // Prevent infinite loop if default icon also fails
  }

  function getIconForProcess(name: string): string {
    // First try with com.company.something pattern
    if (name.startsWith("com.")) {
      const companyName = name.replace(/^com\.([^.]+)\..*$/, "$1");
      const formattedCompanyName =
        companyName.charAt(0).toUpperCase() + companyName.slice(1);
      const companyIconKey = `si${formattedCompanyName}`;
      const companyIcon =
        SimpleIcons[companyIconKey as keyof typeof SimpleIcons];

      if (companyIcon) {
        // Use theme color instead of brand color
        const color = getComputedStyle(document.documentElement)
          .getPropertyValue("--text")
          .trim();
        const svg =
          typeof companyIcon === "object" && "svg" in companyIcon
            ? companyIcon.svg
            : "";
        const svgWithColor = svg.replace("<svg", `<svg fill="${color}"`);
        return `data:image/svg+xml;base64,${btoa(svgWithColor)}`;
      }
    }

    // If no company icon found, fall back to original implementation
    const cleanName = name
      .replace(/\.(app|exe)$/i, "")
      .replace(/[-_./\\]/g, " ")
      .split(" ")[0]
      .trim()
      .toLowerCase();

    const formattedName =
      cleanName.charAt(0).toUpperCase() + cleanName.slice(1);
    const iconKey = `si${formattedName}`;
    let simpleIcon = SimpleIcons[iconKey as keyof typeof SimpleIcons];

    // Default icon if no match found
    if (!simpleIcon) {
      simpleIcon = SimpleIcons.siGhostery;
    }

    // Use theme color instead of brand color
    const color = getComputedStyle(document.documentElement)
      .getPropertyValue("--text")
      .trim();

    const svg =
      typeof simpleIcon === "object" && "svg" in simpleIcon
        ? simpleIcon.svg
        : "";
    const svgWithColor = svg.replace("<svg", `<svg fill="${color}"`);
    return `data:image/svg+xml;base64,${btoa(svgWithColor)}`;
  }
</script>

<div class="table-container">
  <table>
    <thead>
      <tr>
        {#each columns.filter((col) => col.visible) as column}
          <th class="sortable" on:click={() => onToggleSort(column.id)}>
            <div class="th-content">
              {column.label}
              <span
                class="sort-indicator"
                class:active={sortConfig.field === column.id}
              >
                {getSortIndicator(column.id)}
              </span>
            </div>
          </th>
        {/each}
        <th>Actions</th>
      </tr>
    </thead>
    <tbody>
      {#each processes as process (process.pid)}
        <tr
          class:high-usage={process.cpu_usage > 50 ||
            process.memory_usage / (systemStats?.memory_total || 0) > 0.1}
          class:pinned={pinnedProcesses.has(process.command)}
        >
          {#each columns.filter((col) => col.visible) as column}
            <td class="truncate">
              {#if column.id === "name"}
                <div class="name-cell">
                  <img
                    class="process-icon"
                    src={getIconForProcess(process.name)}
                    alt=""
                    height="16"
                    width="16"
                    on:error={handleImageError}
                  />
                  <span class="process-name">{process.name}</span>
                </div>
              {:else if column.format}
                {@html column.format(process[column.id])}
              {:else}
                {process[column.id]}
              {/if}
            </td>
          {/each}
          <td class="col-actions">
            <div class="action-buttons">
              <button
                class="btn-action pin-btn"
                class:pinned={pinnedProcesses.has(process.command)}
                on:click={() => onTogglePin(process.command)}
                title={pinnedProcesses.has(process.command) ? "Unpin" : "Pin"}
              >
                <Fa icon={faThumbtack} />
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
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .table-container {
    flex: 1;
    overflow-x: auto;
    overflow-y: auto;

    /* Scrollbar styles */
    scrollbar-width: thin; /* Firefox */
    scrollbar-color: var(--surface2) var(--mantle); /* Firefox */
  }

  /* Webkit scrollbar styles (Chrome, Safari, Edge) */
  .table-container::-webkit-scrollbar {
    width: 8px;
    height: 8px;
  }

  .table-container::-webkit-scrollbar-track {
    background: var(--mantle);
    border-radius: 4px;
  }

  .table-container::-webkit-scrollbar-thumb {
    background: var(--surface2);
    border-radius: 4px;
    transition: background 0.2s ease;
  }

  .table-container::-webkit-scrollbar-thumb:hover {
    background: var(--surface1);
  }

  .table-container::-webkit-scrollbar-corner {
    background: var(--mantle);
  }

  /* When both scrollbars are present, add some padding to prevent overlap */
  .table-container::-webkit-scrollbar-corner {
    background-color: var(--mantle);
  }

  table {
    width: max-content;
    min-width: 100%;
    table-layout: fixed;
    border-collapse: collapse;
    font-size: 13px;
  }

  th {
    position: sticky;
    top: 0;
    background: var(--mantle);
    text-align: left;
    padding: 8px 12px;
    font-weight: 500;
    color: var(--subtext0);
    border-bottom: 1px solid var(--surface0);
    transition: background-color 0.2s ease;
    z-index: 3;
  }

  td {
    padding: 6px 12px;
    border-bottom: 1px solid var(--surface0);
    color: var(--text);
    z-index: 1;
  }

  tr:hover {
    background-color: var(--surface0);
  }

  .truncate {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 0;
  }

  .sortable {
    cursor: pointer;
    user-select: none;
  }

  .th-content {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .sort-indicator {
    color: var(--overlay0);
    font-size: 12px;
    opacity: 0.5;
    transition: all 0.2s ease;
  }

  .sort-indicator.active {
    color: var(--blue);
    opacity: 1;
  }

  .sortable:hover .sort-indicator {
    opacity: 1;
  }

  .high-usage {
    background-color: color-mix(in srgb, var(--red) 10%, transparent);
  }

  .high-usage:hover {
    background-color: color-mix(in srgb, var(--red) 15%, transparent);
  }

  tr.pinned {
    background-color: color-mix(in srgb, var(--blue) 10%, transparent);
  }

  tr.pinned:hover {
    background-color: color-mix(in srgb, var(--blue) 15%, transparent);
  }

  th:last-child {
    width: 120px;
    min-width: 120px;
    max-width: 120px;
  }

  td:last-child {
    width: 120px;
    min-width: 120px;
    max-width: 120px;
    padding: 6px 8px;
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

  .process-icon {
    width: 16px;
    height: 16px;
    object-fit: contain;
  }

  .name-cell {
    display: flex;
    align-items: center;
    gap: 8px;
  }
</style>
