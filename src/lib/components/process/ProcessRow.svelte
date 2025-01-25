<script lang="ts">
  import type { Process, Column } from "$lib/types";
  import { ProcessIcon, ActionButtons } from "$lib/components";

  export let process: Process;
  export let columns: Column[];
  export let isPinned: boolean;
  export let isHighUsage: boolean;

  export let onTogglePin: (command: string) => void;
  export let onShowDetails: (process: Process) => void;
  export let onKillProcess: (process: Process) => void;
  export let openProcessDirectory: (process: Process) => void;
</script>

<tr class:high-usage={isHighUsage} class:pinned={isPinned}>
  {#each columns.filter((col) => col.visible) as column}
    <td class="truncate">
      {#if column.id === "name"}
        <div class="name-cell">
          <ProcessIcon processName={process.name} />
          <span class="process-name">{process.name}</span>
        </div>
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
    {openProcessDirectory}
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
</style>
