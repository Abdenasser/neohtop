<script lang="ts">
  import {
    AppInfo,
    StatusFilter,
    SearchBox,
    RefreshControls,
    PaginationControls,
    ColumnToggle,
  } from "$lib/components";

  export let searchTerm: string;
  export let statusFilter: string = "all";
  export let itemsPerPage: number;
  export let currentPage: number;
  export let totalPages: number;
  export let totalResults: number;
  export let columns: Array<{
    id: string;
    label: string;
    visible: boolean;
    required?: boolean;
  }>;
  export let refreshRate: number;
  export let isFrozen: boolean;

  let windowWidth = window.innerWidth;
  window.onresize = () => (windowWidth = window.innerWidth);
</script>

<div class="toolbar">
  <div class="toolbar-content">
    <SearchBox bind:searchTerm />
    <StatusFilter bind:statusFilter />

    <div class="toolbar-spacer"></div>

    <PaginationControls
      bind:itemsPerPage
      bind:currentPage
      {totalPages}
      {totalResults}
    />
    <div class="toolbar-spacer"></div>

    <ColumnToggle {columns} />

    <RefreshControls bind:refreshRate bind:isFrozen />

    {#if windowWidth >= 1115}
      <AppInfo />
    {/if}
  </div>
</div>

<style>
  .toolbar {
    padding: 8px;
    border-bottom: 1px solid var(--surface0);
    background-color: var(--mantle);
  }

  .toolbar-content {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 0 12px;
    min-width: max-content;
  }

  .toolbar-spacer {
    flex: 1;
  }
</style>
