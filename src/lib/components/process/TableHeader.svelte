<script lang="ts">
  import { columnWidth } from "$lib/utils/table-layout.js";
  import type { Process, Column } from "$lib/types";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Table from "$lib/components/ui/table/index.js";
  import { ArrowDown, ArrowUp, ArrowDownUp } from "@lucide/svelte";
  export let columns: Column[];
  export let sortConfig: { field: keyof Process; direction: "asc" | "desc" };
  export let onToggleSort: (field: keyof Process) => void;
  export let treeView = false;

  const numeric = [
    "cpu_usage",
    "memory_usage",
    "virtual_memory",
    "disk_usage",
    "run_time",
  ];
</script>

<Table.Header
  ><Table.Row class="hover:bg-transparent"
    >{#each columns.filter((c) => c.visible) as column}<Table.Head
        class={`${numeric.includes(column.id) ? "text-right" : ""} ${treeView && column.id === "name" ? "tree-name" : ""} ${column.id === "name" ? "pl-4" : ""}`}
        style={`width:${columnWidth(column.id, treeView)}px`}
        aria-sort={sortConfig.field === column.id
          ? sortConfig.direction === "asc"
            ? "ascending"
            : "descending"
          : "none"}
        ><Button
          variant="ghost"
          size="sm"
          class={`h-8 max-w-full px-1 text-xs ${sortConfig.field === column.id ? "text-primary" : "text-muted-foreground"}`}
          onclick={() => onToggleSort(column.id)}
          title={treeView && column.id === "memory_usage"
            ? "Sum of resident memory for each family; shared pages can be counted more than once."
            : treeView && column.id === "cpu_usage"
              ? "Combined family CPU; may exceed 100% across cores."
              : `Sort by ${column.label}`}
          ><span
            >{treeView && column.id === "cpu_usage"
              ? "Family CPU"
              : treeView && column.id === "memory_usage"
                ? "Family RAM"
                : column.label}</span
          >{#if sortConfig.field === column.id}{#if sortConfig.direction === "asc"}<ArrowUp
                class="size-3"
              />{:else}<ArrowDown class="size-3" />{/if}{:else}<ArrowDownUp
              class="size-3 opacity-50"
            />{/if}</Button
        ></Table.Head
      >{/each}<Table.Head class="actions-heading w-28 pr-4 text-right"
      >Actions</Table.Head
    ></Table.Row
  ></Table.Header
>
