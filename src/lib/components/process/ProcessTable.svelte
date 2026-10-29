<script lang="ts">
  import type { Process, Column, ProcessTreeRow } from "$lib/types";
  import { tableWidth } from "$lib/utils/table-layout.js";
  import { TableHeader, ProcessRow } from "$lib/components";
  import { processIdentity } from "$lib/utils/process-tree.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Table from "$lib/components/ui/table/index.js";
  import { Search } from "@lucide/svelte";
  export let onClearFilters: () => void = () => {};

  export let processes: Process[];
  export let columns: Column[];
  export let systemStats: { memory_total: number } | null;
  export let sortConfig: { field: keyof Process; direction: "asc" | "desc" };
  export let pinnedProcesses: Set<string>;
  export let viewMode: "flat" | "tree" = "flat";
  export let treeRows: ProcessTreeRow[] = [];
  export let searchActive = false;
  export let onToggleExpand: (key: string) => void = () => {};

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

<section class="flex min-h-0 min-w-0 flex-1 flex-col" aria-label="Processes">
  <!-- svelte-ignore a11y_no_noninteractive_tabindex a11y_no_noninteractive_element_interactions (The labeled scroll region supports WebKit keyboard scrolling.) -->
  <div
    class="process-table-region focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-ring"
    role="region"
    aria-label="Process table"
    tabindex="0"
    onkeydown={(event) => {
      if (
        event.target === event.currentTarget &&
        (event.key === "ArrowLeft" || event.key === "ArrowRight")
      ) {
        event.preventDefault();
        event.currentTarget.scrollBy({
          left: event.key === "ArrowRight" ? 160 : -160,
        });
      }
    }}
  >
    <Table.Root
      style={`width:max(100%,${tableWidth(columns, viewMode === "tree")}px)`}
      ><TableHeader
        {columns}
        {sortConfig}
        {onToggleSort}
        treeView={viewMode === "tree"}
      /><Table.Body
        >{#each rows as row (row.key)}<ProcessRow
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
          />{:else}<Table.Row
            ><Table.Cell
              colspan={columns.filter((c) => c.visible).length + 1}
              class="h-48 text-center"
              ><Search class="mx-auto mb-3 size-6 text-muted-foreground" />
              <p class="text-sm font-medium">
                {searchActive
                  ? "No matching processes"
                  : "No processes in this snapshot"}
              </p>
              <p class="mt-1 text-xs text-muted-foreground">
                {searchActive
                  ? "Try another name, PID or condition."
                  : "Wait for the next update or resume monitoring."}
              </p>
              {#if searchActive}<Button
                  variant="link"
                  size="sm"
                  onclick={onClearFilters}>Clear search and filters</Button
                >{/if}</Table.Cell
            ></Table.Row
          >{/each}</Table.Body
      ></Table.Root
    >
  </div>
</section>
