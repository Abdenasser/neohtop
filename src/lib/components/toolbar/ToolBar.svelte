<script lang="ts">
  import {
    AppInfo,
    SearchBox,
    RefreshControls,
    PaginationControls,
    ColumnToggle,
    FilterToggle,
  } from "$lib/components";
  import { overlayStore } from "$lib/stores/overlay";

  export let searchTerm: string;
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
  export let filters: {
    cpu: { operator: string; value: number; enabled: boolean };
    ram: { operator: string; value: number; enabled: boolean };
    runtime: { operator: string; value: number; enabled: boolean };
    status: { values: string[]; enabled: boolean };
  } = {
    cpu: { operator: ">", value: 50, enabled: false },
    ram: { operator: ">", value: 100, enabled: false },
    runtime: { operator: ">", value: 60, enabled: false },
    status: { values: [], enabled: false },
  };

  $: isAnyOverlayOpen = $overlayStore !== null;
  $: activeOverlayType = $overlayStore;
</script>

<div class="toolbar">
  <div class="toolbar-content" class:overlay-mode={isAnyOverlayOpen}>
    <div class:hidden={isAnyOverlayOpen && activeOverlayType !== "searchHelp"}>
      <SearchBox bind:searchTerm />
    </div>

    <div class:hidden={isAnyOverlayOpen && activeOverlayType !== "filters"}>
      <FilterToggle bind:filters />
    </div>

    <div class="toolbar-spacer" class:hidden={isAnyOverlayOpen}></div>

    <div class:hidden={isAnyOverlayOpen && activeOverlayType !== "pagination"}>
      <PaginationControls
        bind:itemsPerPage
        bind:currentPage
        {totalPages}
        {totalResults}
      />
    </div>
    <div class="toolbar-spacer" class:hidden={isAnyOverlayOpen}></div>

    <div class:hidden={isAnyOverlayOpen && activeOverlayType !== "columns"}>
      <ColumnToggle {columns} />
    </div>

    <div class:hidden={isAnyOverlayOpen && activeOverlayType !== "refresh"}>
      <RefreshControls bind:refreshRate bind:isFrozen />
    </div>

    <div class:hidden={isAnyOverlayOpen && activeOverlayType !== "theme"}>
      <AppInfo />
    </div>
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
    height: 44px;
    position: relative;
  }

  .toolbar-content > div {
    display: flex;
    align-items: center;
  }

  .toolbar-content :global(.hidden) {
    opacity: 0;
    pointer-events: none;
  }

  .toolbar-spacer {
    flex: 1;
  }
</style>
