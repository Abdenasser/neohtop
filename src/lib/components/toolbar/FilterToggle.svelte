<script lang="ts">
  import Fa from "svelte-fa";
  import { faFilter } from "@fortawesome/free-solid-svg-icons";
  import { overlayStore } from "$lib/stores/overlay";
  import { onDestroy } from "svelte";

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

  let containerElement: HTMLDivElement;
  let overlayElement: HTMLDivElement;

  $: showFilters = $overlayStore === "filters";
  $: hasActiveFilters = Object.values(filters).some((f) => f.enabled);
  $: activeFilterCount = Object.values(filters).filter((f) => f.enabled).length;

  const operators = [
    { value: ">", label: ">" },
    { value: "<", label: "<" },
  ];

  const statusOptions = [
    { value: "Running", label: "Running", color: "var(--green)" },
    { value: "Sleeping", label: "Sleeping", color: "var(--blue)" },
    { value: "Stopped", label: "Stopped", color: "var(--red)" },
    { value: "Zombie", label: "Zombie", color: "var(--yellow)" },
  ];

  function updateOverlayPosition() {
    if (overlayElement && containerElement) {
      const toolbarContent = containerElement.closest(".toolbar-content");
      if (toolbarContent) {
        const toolbarRect = toolbarContent.getBoundingClientRect();
        const containerRect = containerElement.getBoundingClientRect();

        const leftOffset = containerRect.left - toolbarRect.left;
        const rightOffset = toolbarRect.right - containerRect.right;
        const topOffset = containerRect.top - toolbarRect.top;

        overlayElement.style.left = `-${leftOffset}px`;
        overlayElement.style.right = `-${rightOffset}px`;
        overlayElement.style.top = `-${topOffset}px`;
      }
    }
  }

  function toggleFilters(event: Event) {
    event.stopPropagation();
    if (showFilters) {
      overlayStore.close();
    } else {
      overlayStore.open("filters");
      setTimeout(updateOverlayPosition, 0);
    }
  }

  function toggleFilter(type: keyof typeof filters) {
    filters[type].enabled = !filters[type].enabled;
    filters = { ...filters };
  }

  function updateFilter(
    type: keyof typeof filters,
    field: string,
    value: string | number,
  ) {
    if (type === "status" && field === "values") {
      // Handle status array toggle
      const statusValue = value as string;
      const currentValues = filters.status.values;
      if (currentValues.includes(statusValue)) {
        filters.status.values = currentValues.filter((v) => v !== statusValue);
      } else {
        filters.status.values = [...currentValues, statusValue];
      }
      filters.status.enabled = filters.status.values.length > 0;
    } else if (field === "operator") {
      (filters[type as keyof Omit<typeof filters, "status">] as any)[field] =
        value as string;
    } else if (field === "value") {
      (filters[type as keyof Omit<typeof filters, "status">] as any)[field] =
        value as number;
    }
    filters = { ...filters };
  }

  function clearAllFilters() {
    Object.keys(filters).forEach((key) => {
      const filterKey = key as keyof typeof filters;
      if (filterKey === "status") {
        filters[filterKey].values = [];
      }
      filters[filterKey].enabled = false;
    });
    filters = { ...filters };
  }

  function getFilterLabel(type: keyof typeof filters): string {
    const labels = {
      cpu: "CPU %",
      ram: "RAM MB",
      runtime: "Runtime min",
      status: "Status",
    };
    return labels[type];
  }

  function handleClickOutside(event: MouseEvent) {
    if (
      showFilters &&
      containerElement &&
      !containerElement.contains(event.target as Node)
    ) {
      overlayStore.close();
    }
  }

  function setupClickOutside() {
    if (typeof document !== "undefined") {
      document.addEventListener("click", handleClickOutside);
    }
  }

  function cleanupClickOutside() {
    if (typeof document !== "undefined") {
      document.removeEventListener("click", handleClickOutside);
    }
  }

  $: if (showFilters) {
    setTimeout(setupClickOutside, 0);
  } else {
    cleanupClickOutside();
  }

  onDestroy(() => {
    cleanupClickOutside();
  });
</script>

<div class="filter-toggle" bind:this={containerElement}>
  <button
    class="filter-button"
    class:active={showFilters}
    class:has-filters={hasActiveFilters}
    on:click={toggleFilters}
    aria-label="Toggle filters"
  >
    <Fa icon={faFilter} />
    Filters
    {#if hasActiveFilters}
      <span class="filter-count">{activeFilterCount}</span>
    {/if}
  </button>

  {#if showFilters}
    <div
      class="touchbar-full-overlay"
      bind:this={overlayElement}
      on:click={() => overlayStore.close()}
      on:keydown={(e) => e.key === "Escape" && overlayStore.close()}
      role="dialog"
      aria-label="Filter options overlay"
      tabindex="-1"
    >
      <div class="filter-content">
        <div class="filter-sections">
          <!-- Numeric Filters -->
          <div class="filter-section">
            <span class="section-label">Performance:</span>
            <div class="filter-controls">
              {#each [["cpu", "CPU %"], ["ram", "RAM MB"], ["runtime", "Runtime min"]] as [type, label]}
                {@const filterKey = type as "cpu" | "ram" | "runtime"}
                <div class="filter-control">
                  <button
                    class="filter-toggle-btn"
                    class:active={filters[filterKey].enabled}
                    on:click|stopPropagation={() => toggleFilter(filterKey)}
                  >
                    {label}
                  </button>
                  {#if filters[filterKey].enabled}
                    <select
                      class="operator-select"
                      bind:value={filters[filterKey].operator}
                      on:change={(e) =>
                        updateFilter(
                          filterKey,
                          "operator",
                          (e.target as HTMLSelectElement).value,
                        )}
                      on:click|stopPropagation
                    >
                      {#each operators as op}
                        <option value={op.value}>{op.label}</option>
                      {/each}
                    </select>
                    <input
                      type="number"
                      class="value-input"
                      bind:value={filters[filterKey].value}
                      on:input={(e) =>
                        updateFilter(
                          filterKey,
                          "value",
                          parseInt((e.target as HTMLInputElement).value),
                        )}
                      on:click|stopPropagation
                      on:focus|stopPropagation
                      placeholder={type === "cpu"
                        ? "50"
                        : type === "ram"
                          ? "100"
                          : "60"}
                    />
                    {#if type === "ram"}
                      <span class="unit">MB</span>
                    {:else if type === "runtime"}
                      <span class="unit">min</span>
                    {:else}
                      <span class="unit">%</span>
                    {/if}
                  {/if}
                </div>
              {/each}
            </div>
          </div>

          <!-- Status Filter -->
          <div class="filter-section">
            <span class="section-label">Status:</span>
            <div class="status-controls">
              {#each statusOptions as status}
                <button
                  class="status-toggle"
                  class:active={filters.status.values.includes(status.value)}
                  style="--status-color: {status.color}"
                  on:click|stopPropagation={() =>
                    updateFilter("status", "values", status.value)}
                >
                  {status.label}
                </button>
              {/each}
            </div>
          </div>

          <!-- Actions -->
          <div class="filter-actions">
            {#if hasActiveFilters}
              <button
                class="clear-all-btn"
                on:click|stopPropagation={clearAllFilters}
              >
                <Fa icon={faFilter} />
                Clear All
              </button>
            {/if}
            <div class="filter-summary">
              {#if hasActiveFilters}
                <span
                  >{activeFilterCount} filter{activeFilterCount > 1 ? "s" : ""} active</span
                >
              {:else}
                <span>No filters applied</span>
              {/if}
            </div>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .filter-toggle {
    position: relative;
  }

  .filter-button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 0 12px;
    height: 28px;
    font-size: 12px;
    color: var(--text);
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    box-sizing: border-box;
  }

  .filter-button:hover {
    background: var(--surface1);
    border-color: var(--blue);
  }

  .filter-button.active {
    background: var(--surface1);
    border-color: var(--blue);
  }

  .filter-button.has-filters {
    border-color: var(--blue);
    background: var(--surface1);
  }

  .filter-count {
    background: var(--blue);
    color: var(--base);
    border-radius: 10px;
    padding: 2px 6px;
    font-size: 10px;
    font-weight: 600;
    min-width: 16px;
    text-align: center;
  }

  .touchbar-full-overlay {
    position: absolute;
    top: -0px;
    height: 44px;
    background: var(--mantle);
    border: none;
    border-radius: 0;
    box-shadow: none;
    z-index: 1000;
    display: flex;
    align-items: center;
    padding: 0 16px;
    gap: 16px;
    overflow-x: auto;
    overflow-y: hidden;
    scrollbar-width: none;
    -ms-overflow-style: none;
  }

  .touchbar-full-overlay::-webkit-scrollbar {
    display: none;
  }

  .filter-content {
    display: flex;
    align-items: center;
    gap: 32px;
    width: 100%;
    animation: slideIn 0.3s ease-out;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(-8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .filter-sections {
    display: flex;
    align-items: center;
    gap: 32px;
    flex: 1;
  }

  .filter-section {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-shrink: 0;
  }

  .section-label {
    font-size: 12px;
    font-weight: 500;
    color: var(--subtext0);
    flex-shrink: 0;
  }

  .filter-controls {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .filter-control {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .filter-toggle-btn {
    padding: 4px 8px;
    height: 26px;
    font-size: 11px;
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 4px;
    color: var(--text);
    cursor: pointer;
    transition: all 0.15s ease;
    white-space: nowrap;
  }

  .filter-toggle-btn:hover {
    background: var(--surface1);
    border-color: var(--blue);
  }

  .filter-toggle-btn.active {
    background: var(--blue);
    color: var(--base);
    border-color: var(--blue);
  }

  .operator-select {
    padding: 4px 8px;
    height: 26px;
    font-size: 11px;
    border: 1px solid var(--surface1);
    border-radius: 4px;
    background: var(--surface0);
    color: var(--text);
    cursor: pointer;
    transition: all 0.15s ease;
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    text-align: center;
    min-width: 32px;
  }

  .operator-select:hover {
    background: var(--surface1);
    border-color: var(--blue);
  }

  .operator-select:focus {
    outline: none;
    border-color: var(--blue);
    background: var(--surface1);
  }

  .value-input {
    width: 50px;
    padding: 2px 6px;
    height: 22px;
    font-size: 11px;
    border: 1px solid var(--surface1);
    border-radius: 3px;
    background: var(--surface0);
    color: var(--text);
    text-align: center;
  }

  .value-input:focus {
    outline: none;
    border-color: var(--blue);
  }

  .unit {
    font-size: 10px;
    color: var(--subtext0);
  }

  .status-controls {
    display: flex;
    gap: 6px;
    align-items: center;
  }

  .status-toggle {
    padding: 4px 8px;
    height: 26px;
    font-size: 11px;
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 4px;
    color: var(--text);
    cursor: pointer;
    transition: all 0.15s ease;
    white-space: nowrap;
  }

  .status-toggle:hover {
    background: var(--surface1);
    border-color: var(--status-color);
  }

  .status-toggle.active {
    background: var(--status-color);
    color: var(--base);
    border-color: var(--status-color);
  }

  .filter-actions {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-shrink: 0;
  }

  .clear-all-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 8px;
    height: 26px;
    font-size: 11px;
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 4px;
    color: var(--text);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .clear-all-btn:hover {
    background: var(--red);
    border-color: var(--red);
    color: var(--base);
  }

  .filter-summary {
    font-size: 11px;
    color: var(--subtext0);
  }
</style>
