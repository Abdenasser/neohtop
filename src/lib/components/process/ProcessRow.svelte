<script lang="ts">
  import type { Process, Column, ProcessTreeRow } from "$lib/types";
  import { ProcessIcon, ActionButtons } from "$lib/components";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import * as Table from "$lib/components/ui/table/index.js";
  import { ChevronRight, ChevronDown, Pin } from "@lucide/svelte";
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
  const numeric = [
    "cpu_usage",
    "memory_usage",
    "virtual_memory",
    "disk_usage",
    "run_time",
  ];
</script>

<Table.Row data-pinned={isPinned} data-high-usage={isHighUsage}
  >{#each columns.filter((c) => c.visible) as column}<Table.Cell
      class={`${numeric.includes(column.id) ? "text-right font-mono text-xs" : ""} ${column.id === "name" ? "pl-4" : ""} ${tree && column.id === "name" ? "tree-name" : ""}`}
      ><div
        class={column.id === "name" ? "" : "max-w-72 truncate"}
        title={column.id === "name"
          ? undefined
          : column.format
            ? column.format(process[column.id])
            : String(process[column.id] ?? "")}
      >
        {#if column.id === "name"}<div
            class="flex min-w-0 items-center gap-2"
            style:padding-left={tree
              ? `${Math.min(tree.depth, 10) * 16}px`
              : undefined}
          >
            {#if tree}{#if tree.childCount > 0}<Button
                  variant="ghost"
                  size="icon-xs"
                  disabled={expansionLocked}
                  aria-expanded={tree.expanded}
                  aria-label={`${tree.expanded ? "Collapse" : "Expand"} ${process.name} children`}
                  title={expansionLocked
                    ? "Clear search and filters to collapse this branch"
                    : undefined}
                  onclick={() => tree && onToggleExpand(tree.key)}
                  >{#if tree.expanded}<ChevronDown />{:else}<ChevronRight
                    />{/if}</Button
                >{:else}<span class="size-6 shrink-0" aria-hidden="true"
                ></span>{/if}{/if}<ProcessIcon
              pid={process.pid}
              startTime={process.start_time}
            /><Button
              variant="link"
              class="h-auto min-w-0 shrink justify-start truncate p-0 text-left text-[13px] font-normal text-foreground"
              title={process.name}
              onclick={() => onShowDetails(process)}
              ><span class="max-w-64 truncate">{process.name}</span></Button
            >{#if tree && tree.familyProcessCount > 1}<Badge
                variant="secondary"
                class="px-1.5 text-[10px] font-normal"
                title={`${tree.familyProcessCount} processes in this family`}
                >{tree.familyProcessCount}</Badge
              >{/if}{#if isPinned}<Pin
                class="size-3 shrink-0 fill-primary/20 text-primary"
              />{/if}
          </div>
        {:else if column.id === "cpu_usage"}{@const cpu =
            tree?.familyCpuUsage ?? process.cpu_usage}<span
            class={cpu > 50
              ? "rounded bg-destructive/10 px-1.5 py-1 text-destructive"
              : ""}
            title={tree
              ? `Combined CPU for ${tree.familyProcessCount} processes`
              : undefined}>{cpu.toFixed(1)}%</span
          >
        {:else if tree && column.id === "memory_usage"}<span
            title="Sum of resident memory for this process and all descendants; shared pages may overlap."
            >{(tree.familyMemoryUsage / (1024 * 1024)).toFixed(1)} MB</span
          >
        {:else if column.id === "status"}<span
            class="flex items-center gap-1.5 text-xs text-muted-foreground"
            ><span
              class="size-1.5 shrink-0 rounded-full"
              style:background={process.status === "Running"
                ? "var(--status-running)"
                : process.status === "Zombie" || process.status === "Stopped"
                  ? "var(--status-stopped)"
                  : "var(--status-idle)"}
            ></span>{process.status}</span
          >
        {:else if column.format}{column.format(process[column.id])}
        {:else}<span
            class={["pid", "ppid", "user", "session_id"].includes(column.id)
              ? "font-mono text-xs text-muted-foreground"
              : ""}>{process[column.id] ?? "—"}</span
          >{/if}
      </div></Table.Cell
    >{/each}<ActionButtons
    {process}
    {isPinned}
    {onTogglePin}
    {onShowDetails}
    {onKillProcess}
  /></Table.Row
>
