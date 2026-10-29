<script lang="ts">
  import SettingsWindow from "$lib/components/SettingsWindow.svelte";
  import AppInfo from "$lib/components/AppInfo.svelte";
  import { emit } from "@tauri-apps/api/event";
  import { get } from "svelte/store";
  import { settingsStatus } from "$lib/stores/settings";
  import {
    startMonitoringPreferences,
    monitoringPreferencesError,
    retryMonitoringPreferences,
  } from "$lib/stores/monitoring-preferences";
  import MenuBarOverview from "$lib/components/MenuBarOverview.svelte";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";
  import {
    debounce,
    filterProcesses,
    sortProcesses,
    searchHasPortQuery,
  } from "$lib/utils";
  import {
    StatsBar,
    ToolBar,
    PaginationControls,
    TitleBar,
    ProcessTable,
    ProcessDetailsModal,
    KillProcessModal,
  } from "$lib/components/index";
  import { themeStore, settingsStore, processStore } from "$lib/stores/index";
  import { column_definitions } from "$lib/definitions/columns";
  import type { Process } from "$lib/types";
  import {
    buildProcessTree,
    paginateProcessTree,
  } from "$lib/utils/process-tree.js";

  $: ({
    processes,
    systemStats,
    gpus,
    resourceHistory,
    portError,
    error,
    searchTerm,
    isLoading,
    currentPage,
    pinnedProcesses,
    selectedProcess,
    showInfoModal,
    showConfirmModal,
    processToKill,
    isKilling,
    isFrozen,
    sortConfig,
  } = $processStore);

  const view =
    typeof window !== "undefined"
      ? new URLSearchParams(window.location.search).get("view")
      : null;
  const companion = view === "companion";
  const auxiliary = companion || view === "settings" || view === "about";
  let mounted = false;
  let disposed = false;
  const listeners: UnlistenFn[] = [];
  let intervalId: NodeJS.Timeout;
  let cachedFilteredProcesses: Process[] = [];
  let cachedSortedProcesses: Process[] = [];
  let expandedProcesses = new Set<string>();

  function setProcessView(view: "flat" | "tree") {
    settingsStore.updateConfig({
      behavior: { ...$settingsStore.behavior, processView: view },
    });
    processStore.setCurrentPage(1);
  }

  function toggleExpansion(key: string) {
    const next = new Set(expandedProcesses);
    if (next.has(key)) next.delete(key);
    else next.add(key);
    expandedProcesses = next;
  }

  // Initialize filters object for the new FilterToggle
  let filters = {
    cpu: { operator: ">", value: 50, enabled: false },
    gpu: { operator: ">", value: 1, enabled: false },
    ram: { operator: ">", value: 100, enabled: false },
    runtime: { operator: ">", value: 60, enabled: false },
    status: { values: [], enabled: false },
  };

  $: columns = column_definitions.map((col) => ({
    ...col,
    visible:
      col.required ||
      ($settingsStore.appearance.columnVisibility[col.id] ?? col.visible),
  }));
  $: itemsPerPage = $settingsStore.behavior.itemsPerPage;
  $: refreshRate = $settingsStore.behavior.refreshRate;
  $: processStore.setGpuDetails(
    (!!$settingsStore.appearance.columnVisibility.gpu_usage ||
      filters.gpu.enabled) &&
      !auxiliary,
  );
  $: processView = $settingsStore.behavior.processView ?? "flat";
  $: searchActive =
    searchTerm.trim().length > 0 ||
    Object.values(filters).some((filter) => filter.enabled);

  // Throttled filtering to reduce CPU usage
  const debouncedFilter = debounce(
    (procs: Process[], term: string, activeFilters: typeof filters) => {
      cachedFilteredProcesses = filterProcesses(procs, term, activeFilters);
    },
    100,
  );

  const debouncedPortRefresh = debounce(() => {
    processStore.getProcesses();
  }, 150);

  // Recalculate when the process snapshot, search, or filters change
  $: {
    if (searchTerm || Object.values(filters).some((f) => f.enabled)) {
      debouncedFilter(processes, searchTerm, filters);
    } else {
      cachedFilteredProcesses = processes;
    }
  }

  // Collect sockets as soon as the query becomes a port search, not on the next tick
  $: if (mounted && !isFrozen && !auxiliary && searchHasPortQuery(searchTerm)) {
    debouncedPortRefresh();
  }

  // Cache sorted results to avoid re-sorting unchanged data
  $: if (cachedFilteredProcesses && (sortConfig || pinnedProcesses.size > 0)) {
    cachedSortedProcesses = sortProcesses(
      cachedFilteredProcesses,
      sortConfig,
      pinnedProcesses,
    );
  } else {
    cachedSortedProcesses = cachedFilteredProcesses;
  }

  $: tree =
    processView === "tree"
      ? buildProcessTree(
          sortProcesses(processes, sortConfig, pinnedProcesses),
          expandedProcesses,
          searchActive
            ? new Set(cachedFilteredProcesses.map((process) => process.pid))
            : null,
          { sortConfig, pinnedProcesses },
        )
      : { rows: [], rootCount: 0 };
  $: treePage = paginateProcessTree(tree, currentPage, itemsPerPage);
  $: totalPages =
    processView === "tree"
      ? treePage.totalPages
      : Math.max(1, Math.ceil(cachedFilteredProcesses.length / itemsPerPage));
  $: if (currentPage > totalPages) processStore.setCurrentPage(totalPages);
  $: paginatedProcesses = cachedSortedProcesses.slice(
    (currentPage - 1) * itemsPerPage,
    currentPage * itemsPerPage,
  );

  // Reset the actual store page when search, filtering or page size changes.
  // Keeping only the destructured local page at 1 lets the next store read undo it.
  $: {
    searchTerm;
    itemsPerPage;
    filters;
    processStore.setCurrentPage(1);
  }
  function clearSearchAndFilters() {
    processStore.setSearchTerm("");
    filters = {
      cpu: { ...filters.cpu, enabled: false },
      gpu: { ...filters.gpu, enabled: false },
      ram: { ...filters.ram, enabled: false },
      runtime: { ...filters.runtime, enabled: false },
      status: { values: [], enabled: false },
    };
  }

  $: {
    if (intervalId) clearInterval(intervalId);
    if (mounted && !auxiliary && !isFrozen) {
      // Use adaptive refresh rate - slightly slower for 1s to reduce CPU usage
      const adaptiveRefreshRate = refreshRate === 1000 ? 1500 : refreshRate;
      intervalId = setInterval(() => {
        processStore.getProcesses();
      }, adaptiveRefreshRate);
    }
  }

  onMount(() => {
    if (auxiliary) {
      if (!companion) themeStore.init();
      return;
    }
    themeStore.init();
    void (async () => {
      await settingsStore.init(true);
      const stopPreferences = await startMonitoringPreferences();
      const stopRuntimeError = monitoringPreferencesError.subscribe((error) => {
        void emit("preferences-runtime-error", error);
      });
      const stopRetry = await listen("preferences-runtime-retry", async () => {
        await settingsStore.retry(true);
        await retryMonitoringPreferences();
      });
      const stopStatus = await listen(
        "preferences-runtime-status-request",
        () => {
          void emit(
            "preferences-runtime-error",
            get(monitoringPreferencesError),
          );
        },
      );
      const stopCommand = await listen<string>("app-command", ({ payload }) => {
        if (payload === "view-list" || payload === "view-tree")
          setProcessView(payload === "view-tree" ? "tree" : "flat");
      });
      if (disposed) {
        [
          stopPreferences,
          stopRuntimeError,
          stopRetry,
          stopStatus,
          stopCommand,
        ].forEach((stop) => stop());
        return;
      }
      listeners.push(
        stopPreferences,
        stopRuntimeError,
        stopRetry,
        stopStatus,
        stopCommand,
      );
      mounted = true;
      const stopControl = await listen("monitor-control", () => {
        void processStore.refreshControl();
      });
      const stopOpen = await listen<{ pid: number; start_time: number }>(
        "open-process",
        async ({ payload }) => {
          await processStore.refreshControl();
          const process = $processStore.processes.find(
            (p) => p.pid === payload.pid && p.start_time === payload.start_time,
          );
          if (process) processStore.showProcessDetails(process);
          else
            processStore.update((state) => ({
              ...state,
              error: "This process is no longer available.",
            }));
        },
      );
      if (disposed) {
        stopControl();
        stopOpen();
        return;
      }
      listeners.push(stopControl, stopOpen);
      await processStore.getProcesses();
      if (!disposed) processStore.setIsLoading(false);
    })();
  });

  onDestroy(() => {
    disposed = true;
    if (intervalId) clearInterval(intervalId);
    listeners.forEach((stop) => stop());
  });
</script>

{#if view === "settings"}
  <SettingsWindow />
{:else if view === "about"}
  <AppInfo />
{:else if companion}
  <MenuBarOverview />
{:else if isLoading}
  <div class="loading-container">
    <div class="loading-content">
      <img src="128x128.png" alt="NeoHtop Logo" class="logo" />
    </div>
  </div>
{:else}
  <div class="app-container">
    <TitleBar {isFrozen} onPause={processStore.setIsFrozen} />
    <main>
      {#if systemStats}
        <StatsBar
          {systemStats}
          {gpus}
          capturedAt={$processStore.capturedAt}
          history={resourceHistory}
          {isFrozen}
          unavailable={!!error}
        />
      {/if}

      <ToolBar
        bind:searchTerm={$processStore.searchTerm}
        totalProcesses={processes.length}
        matchingProcesses={cachedFilteredProcesses.length}
        viewMode={processView}
        expandedCount={expandedProcesses.size}
        onViewChange={setProcessView}
        onCollapseAll={() => (expandedProcesses = new Set())}
        onClearFilters={clearSearchAndFilters}
        bind:filters
        {columns}
      />

      {#if error || portError || $settingsStatus.error || $monitoringPreferencesError}
        <div class="alert" role="alert">
          {error ||
            portError ||
            $settingsStatus.error ||
            $monitoringPreferencesError}
        </div>
      {/if}

      <ProcessTable
        onClearFilters={clearSearchAndFilters}
        processes={paginatedProcesses}
        {columns}
        {systemStats}
        {sortConfig}
        {pinnedProcesses}
        viewMode={processView}
        treeRows={treePage.rows}
        {searchActive}
        onToggleExpand={toggleExpansion}
        onToggleSort={processStore.toggleSort}
        onTogglePin={processStore.togglePin}
        onShowDetails={processStore.showProcessDetails}
        onKillProcess={processStore.confirmKillProcess}
      />
      <PaginationControls
        bind:itemsPerPage
        bind:currentPage={$processStore.currentPage}
        {totalPages}
        totalResults={processView === "tree"
          ? tree.rootCount
          : cachedFilteredProcesses.length}
        resultLabel={processView === "tree"
          ? tree.rootCount === 1
            ? "family"
            : "families"
          : cachedFilteredProcesses.length === 1
            ? "process"
            : "processes"}
      />
    </main>
  </div>
{/if}

<ProcessDetailsModal
  show={showInfoModal}
  process={selectedProcess}
  {processes}
  onClose={processStore.closeProcessDetails}
  onShowDetails={processStore.showProcessDetails}
/>

<KillProcessModal
  show={showConfirmModal}
  process={processToKill}
  {isKilling}
  onClose={processStore.closeConfirmKill}
  onConfirm={processStore.handleConfirmKill}
/>

<style>
  main {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
    min-width: 0;
    overflow: hidden;
  }
  .app-container {
    height: 100vh;
    display: flex;
    flex-direction: column;
  }
  .alert {
    margin: 8px 16px;
    padding: 8px 12px;
    border: 1px solid var(--destructive);
    border-radius: var(--radius);
    background: color-mix(in srgb, var(--destructive) 8%, var(--background));
    color: var(--destructive);
    font-size: 13px;
  }
  .loading-container {
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--background);
  }
  .loading-content {
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .logo {
    width: 96px;
    height: 96px;
    filter: drop-shadow(
      0 0 18px color-mix(in srgb, var(--primary) 35%, transparent)
    );
  }
</style>
