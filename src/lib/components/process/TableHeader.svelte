<script lang="ts">
  import type { Process, Column } from "$lib/types";

  export let columns: Column[];
  export let sortConfig: { field: keyof Process; direction: "asc" | "desc" };
  export let onToggleSort: (field: keyof Process) => void;
  export let treeView = false;

  function getSortIndicator(field: keyof Process) {
    if (sortConfig.field !== field) return "↕";
    return sortConfig.direction === "asc" ? "↑" : "↓";
  }
</script>

<thead>
  <tr>
    {#each columns.filter((col) => col.visible) as column}
      <th
        class="sortable"
        on:click={() => onToggleSort(column.id)}
        title={treeView && column.id === "memory_usage"
          ? "Sum of reported resident memory for each process family; shared pages can be counted more than once."
          : treeView && column.id === "cpu_usage"
            ? "Combined CPU usage for this process and all descendants; may exceed 100% across CPU cores."
            : undefined}
      >
        <div class="th-content">
          {treeView && column.id === "cpu_usage"
            ? "Family CPU"
            : treeView && column.id === "memory_usage"
              ? "Family RAM"
              : column.label}
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

<style>
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

  th:last-child {
    right: 0;
    z-index: 4;
    width: 120px;
    min-width: 120px;
    max-width: 120px;
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
</style>
