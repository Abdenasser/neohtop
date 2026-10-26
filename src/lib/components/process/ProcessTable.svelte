<script lang="ts">
  import type { Process, Column, ProcessTreeRow } from "$lib/types";
  import { TableHeader, ProcessRow } from "$lib/components";
  import { processIdentity } from "$lib/utils/process-tree.js";
  import Fa from "svelte-fa";
  import { faList, faSitemap } from "@fortawesome/free-solid-svg-icons";

  export let processes: Process[];
  export let columns: Column[];
  export let systemStats: { memory_total: number } | null;
  export let sortConfig: { field: keyof Process; direction: "asc" | "desc" };
  export let pinnedProcesses: Set<string>;
  export let viewMode: "flat" | "tree" = "flat";
  export let treeRows: ProcessTreeRow[] = [];
  export let searchActive = false;
  export let expandedCount = 0;
  export let onViewChange: (view: "flat" | "tree") => void = () => {};
  export let onToggleExpand: (key: string) => void = () => {};
  export let onCollapseAll: () => void = () => {};

  export let onToggleSort: (field: keyof Process) => void;
  export let onTogglePin: (command: string) => void;
  export let onShowDetails: (process: Process) => void;
  export let onKillProcess: (process: Process) => void;

  $: rows =
    viewMode === "tree"
      ? treeRows
      : processes.map((process) => ({
          process,
          key: processIdentity(process),
          depth: 0,
          childCount: 0,
          expanded: false,
          rootIndex: 0,
          familyCpuUsage: process.cpu_usage,
          familyMemoryUsage: process.memory_usage,
          familyProcessCount: 1,
        }));
</script>

<section class="process-browser" aria-label="Processes">
  <div class="view-controls">
    <div class="view-toggle" role="group" aria-label="Process view">
      <button
        aria-pressed={viewMode === "flat"}
        on:click={() => onViewChange("flat")}
      >
        <Fa icon={faList} size="sm" /> List
      </button>
      <button
        aria-pressed={viewMode === "tree"}
        on:click={() => onViewChange("tree")}
      >
        <Fa icon={faSitemap} size="sm" /> Tree
      </button>
    </div>
    {#if viewMode === "tree"}
      <span
        class="tree-hint"
        title="Family CPU sums process CPU usage. Family RAM sums reported resident memory; shared pages may be counted more than once."
      >
        {searchActive
          ? "Matching branches are expanded. Totals include hidden children."
          : "Explore app families and their combined CPU and memory."}
      </span>
      <button
        class="collapse-all"
        on:click={onCollapseAll}
        disabled={searchActive || expandedCount === 0}>Collapse all</button
      >
    {/if}
  </div>
  <div class="table-container">
    <table
      class:tree-view={viewMode === "tree"}
      style:min-width={viewMode === "tree"
        ? `max(100%, ${440 + Math.max(0, columns.filter((column) => column.visible).length - 1) * 100}px)`
        : undefined}
    >
      <TableHeader
        {columns}
        {sortConfig}
        {onToggleSort}
        treeView={viewMode === "tree"}
      />
      <tbody>
        {#each rows as row (row.key)}
          <ProcessRow
            process={row.process}
            {columns}
            tree={viewMode === "tree" ? row : null}
            expansionLocked={searchActive}
            {onToggleExpand}
            isPinned={pinnedProcesses.has(row.process.command)}
            isHighUsage={(viewMode === "tree"
              ? row.familyCpuUsage
              : row.process.cpu_usage) > 50 ||
              (viewMode === "tree"
                ? row.familyMemoryUsage
                : row.process.memory_usage) /
                (systemStats?.memory_total || 0) >
                0.1}
            {onTogglePin}
            {onShowDetails}
            {onKillProcess}
          />
        {/each}
      </tbody>
    </table>
  </div>
</section>

<style>
  .process-browser {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    min-width: 0;
  }

  .view-controls {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 12px;
    background: var(--mantle);
    border-bottom: 1px solid var(--surface0);
    font-size: 12px;
  }

  .view-toggle {
    display: flex;
    gap: 2px;
    padding: 3px;
    border: 1px solid var(--surface0);
    border-radius: 8px;
    background: var(--base);
  }

  .view-toggle button,
  .collapse-all {
    display: flex;
    align-items: center;
    gap: 6px;
    min-height: 28px;
    padding: 4px 12px;
    color: var(--subtext0);
    background: transparent;
    border: 0;
    border-radius: 5px;
    cursor: pointer;
    font-size: 12px;
  }

  .view-toggle button[aria-pressed="true"] {
    color: var(--text);
    background: var(--surface0);
  }

  .view-toggle button:hover,
  .collapse-all:hover:not(:disabled) {
    color: var(--text);
    background: var(--surface0);
  }

  button:focus-visible {
    outline: 2px solid var(--blue);
    outline-offset: 2px;
  }

  .tree-hint {
    color: var(--subtext0);
    flex: 1;
  }

  .collapse-all:disabled {
    opacity: 0.4;
    cursor: default;
  }

  .table-container {
    flex: 1;
    min-width: 0;
    overflow-x: auto;
    overflow-y: auto;
    scrollbar-width: thin;
    scrollbar-color: var(--surface2) var(--mantle);
  }

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

  table {
    width: max-content;
    min-width: 100%;
    table-layout: fixed;
    border-collapse: collapse;
    font-size: 13px;
  }
  .tree-view :global(th:first-child) {
    width: 320px;
    left: 0;
    z-index: 4;
  }
</style>
