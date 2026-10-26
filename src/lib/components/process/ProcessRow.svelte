<script lang="ts">
  import type { Process, Column, ProcessTreeRow } from "$lib/types";
  import { ProcessIcon, ActionButtons } from "$lib/components";

  export let process: Process;
  export let columns: Column[];
  export let isPinned: boolean;
  export let isHighUsage: boolean;
  export let tree: ProcessTreeRow | null = null;
  export let expansionLocked = false;
  export let onToggleExpand: (key: string) => void = () => {};

  export let onTogglePin: (command: string) => void;
  export let onShowDetails: (process: Process) => void;
  export let onKillProcess: (process: Process) => void;
</script>

<tr class:high-usage={isHighUsage} class:pinned={isPinned}>
  {#each columns.filter((col) => col.visible) as column}
    <td
      class="truncate"
      class:tree-name={tree !== null && column.id === "name"}
    >
      {#if column.id === "name"}
        <div
          class="name-cell"
          style:padding-left={tree
            ? `${Math.min(tree.depth, 10) * 16}px`
            : undefined}
        >
          {#if tree}
            {#if tree.childCount > 0}
              <button
                class="branch-toggle"
                disabled={expansionLocked}
                title={expansionLocked
                  ? "Clear search and filters to collapse this branch"
                  : undefined}
                aria-expanded={tree.expanded}
                aria-label={`${tree.expanded ? "Collapse" : "Expand"} ${process.name} children`}
                on:click={() => tree && onToggleExpand(tree.key)}
                >{tree.expanded ? "▾" : "▸"}</button
              >
            {:else}
              <span class="branch-spacer" aria-hidden="true"></span>
            {/if}
          {/if}
          <ProcessIcon processName={process.name} />
          <span class="process-name" title={process.name}>{process.name}</span>
          {#if tree && tree.familyProcessCount > 1}
            <span
              class="child-count"
              title={`${tree.familyProcessCount} processes in this family`}
              >{tree.familyProcessCount}</span
            >
          {/if}
        </div>
      {:else if tree && column.id === "cpu_usage"}
        <span title={`Combined CPU for ${tree.familyProcessCount} processes`}
          >{tree.familyCpuUsage.toFixed(1)}%</span
        >
      {:else if tree && column.id === "memory_usage"}
        <span
          title="Sum of resident memory for this process and all descendants. Shared pages may be counted more than once."
          >{(tree.familyMemoryUsage / (1024 * 1024)).toFixed(1)} MB</span
        >
      {:else if column.format}
        {@html column.format(process[column.id])}
      {:else}
        {process[column.id]}
      {/if}
    </td>
  {/each}
  <ActionButtons
    {process}
    {isPinned}
    {onTogglePin}
    {onShowDetails}
    {onKillProcess}
  />
</tr>

<style>
  td {
    padding: 6px 12px;
    border-bottom: 1px solid var(--surface0);
    color: var(--text);
    z-index: 1;
  }

  .truncate {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 0;
  }

  tr:hover {
    background-color: var(--surface0);
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

  .name-cell {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .tree-name {
    position: sticky;
    left: 0;
    z-index: 2;
    background: var(--base);
    border-right: 1px solid var(--surface0);
  }

  .high-usage .tree-name {
    background: color-mix(in srgb, var(--red) 10%, var(--base));
  }

  .pinned .tree-name {
    background: color-mix(in srgb, var(--blue) 10%, var(--base));
  }

  tr:hover .tree-name {
    background: var(--surface0);
  }

  .process-name {
    overflow: hidden;
    text-overflow: ellipsis;
    min-width: 0;
  }

  .branch-toggle,
  .branch-spacer {
    width: 18px;
    height: 24px;
    flex-shrink: 0;
  }

  .branch-toggle {
    padding: 0;
    border: 0;
    border-radius: 4px;
    background: transparent;
    color: var(--subtext0);
    cursor: pointer;
    font-size: 15px;
  }

  .branch-toggle:hover:not(:disabled) {
    color: var(--blue);
    background: var(--surface0);
  }

  .branch-toggle:disabled {
    cursor: default;
    opacity: 0.6;
  }

  .branch-toggle:focus-visible {
    outline: 2px solid var(--blue);
    outline-offset: 2px;
  }

  .child-count {
    border-radius: 4px;
    padding: 1px 5px;
    background: var(--surface0);
    color: var(--subtext0);
    font-size: 10px;
    flex-shrink: 0;
  }
</style>
