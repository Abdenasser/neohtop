<script lang="ts">
  import type { Process, Column } from "$lib/types";
  import { TableHeader, ProcessRow } from "$lib/components";

  export let processes: Process[];
  export let columns: Column[];
  export let systemStats: { memory_total: number } | null;
  export let sortConfig: { field: keyof Process; direction: "asc" | "desc" };
  export let pinnedProcesses: Set<string>;
  export let selectedProcessPid: number | null;

  export let onToggleSort: (field: keyof Process) => void;
  export let onTogglePin: (command: string) => void;
  export let onShowDetails: (process: Process) => void;
  export let onKillProcess: (process: Process) => void;
  export let onSelectProcess: (process: Process) => void;
</script>

<div class="table-container">
  <table>
    <TableHeader {columns} {sortConfig} {onToggleSort} />
    <tbody>
      {#each processes as process (process.pid)}
        <ProcessRow
          {process}
          {columns}
          isPinned={pinnedProcesses.has(process.command)}
          isSelected={selectedProcessPid === process.pid}
          isHighUsage={process.cpu_usage > 50 ||
            process.memory_usage / (systemStats?.memory_total || 0) > 0.1}
          {onTogglePin}
          {onShowDetails}
          {onKillProcess}
          {onSelectProcess}
        />
      {/each}
    </tbody>
  </table>
</div>

<style>
  .table-container {
    flex: 1;
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
</style>
