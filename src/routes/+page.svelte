<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    StatsBar,
    ToolBar,
    TitleBar,
    ProcessTable,
    ProcessDetailsModal,
    KillProcessModal,
  } from "$lib/components/index";
  import { themeStore, settingsStore, processStore } from "$lib/stores/index";
  import { column_definitions } from "$lib/definitions/columns";
  import { filterProcesses, sortProcesses } from "$lib/utils";

  $: ({
    processes,
    systemStats,
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

  let intervalId: NodeJS.Timeout;

  $: columns = column_definitions.map((col) => ({
    ...col,
    visible:
      col.required ||
      ($settingsStore.appearance.columnVisibility[col.id] ?? col.visible),
  }));
  $: itemsPerPage = $settingsStore.behavior.itemsPerPage;
  $: refreshRate = $settingsStore.behavior.refreshRate;
  $: statusFilter = $settingsStore.behavior.defaultStatusFilter;

  $: filteredProcesses = filterProcesses(processes, searchTerm, statusFilter);

  $: sortedProcesses = sortProcesses(
    filteredProcesses,
    sortConfig,
    pinnedProcesses,
  );

  $: totalPages = Math.ceil(filteredProcesses.length / itemsPerPage);
  $: paginatedProcesses = sortedProcesses.slice(
    (currentPage - 1) * itemsPerPage,
    currentPage * itemsPerPage,
  );

  $: {
    if (searchTerm || itemsPerPage) {
      currentPage = 1;
    }
  }

  $: {
    if (intervalId) clearInterval(intervalId);
    if (!isFrozen) {
      intervalId = setInterval(() => {
        processStore.getProcesses();
      }, refreshRate);
    }
  }

  onMount(async () => {
    try {
      await processStore.getProcesses();
    } catch (error) {
      console.error("Failed to load processes:", error);
    } finally {
      processStore.setIsLoading(false);
    }

    settingsStore.init();
    themeStore.init();
  });

  onDestroy(() => {
    if (intervalId) clearInterval(intervalId);
  });
</script>

{#if isLoading}
  <div class="loading-container">
    <div class="loading-content">
      <img src="128x128.png" alt="NeoHtop Logo" class="logo" />
    </div>
  </div>
{:else}
  <div class="app-container">
    <TitleBar />
    <main>
      {#if systemStats}
        <StatsBar {systemStats} />
      {/if}

      <ToolBar
        bind:searchTerm={$processStore.searchTerm}
        bind:statusFilter
        bind:itemsPerPage
        bind:currentPage={$processStore.currentPage}
        bind:refreshRate
        bind:isFrozen={$processStore.isFrozen}
        {totalPages}
        totalResults={filteredProcesses.length}
        bind:columns
      />

      {#if error}
        <div class="alert">{error}</div>
      {/if}

      <ProcessTable
        processes={paginatedProcesses}
        {columns}
        {systemStats}
        {sortConfig}
        {pinnedProcesses}
        onToggleSort={processStore.toggleSort}
        onTogglePin={processStore.togglePin}
        onShowDetails={processStore.showProcessDetails}
        onKillProcess={processStore.confirmKillProcess}
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
  :global(:root) {
    --base: #1e1e2e;
    --mantle: #181825;
    --crust: #11111b;
    --text: #cdd6f4;
    --subtext0: #a6adc8;
    --subtext1: #bac2de;
    --surface0: #313244;
    --surface1: #45475a;
    --surface2: #585b70;
    --overlay0: #6c7086;
    --overlay1: #7f849c;
    --blue: #89b4fa;
    --lavender: #b4befe;
    --sapphire: #74c7ec;
    --sky: #89dceb;
    --red: #f38ba8;
    --maroon: #eba0ac;
    --peach: #fab387;
    --yellow: #f9e2af;
    --green: #a6e3a1;
    --teal: #94e2d5;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial,
      sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
    background-color: var(--base);
    color: var(--text);
    -webkit-font-smoothing: antialiased;
    overflow: hidden;
    user-select: none;
  }

  main {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: min-content;
    overflow: hidden;
  }

  .app-container {
    height: 100vh;
    display: flex;
    flex-direction: column;
  }

  .alert {
    margin: 8px;
    padding: 8px 12px;
    background-color: var(--surface0);
    border: 1px solid var(--red);
    border-radius: 6px;
    color: var(--red);
    font-size: 13px;
  }

  .loading-container {
    width: 100vw;
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, var(--base) 0%, var(--mantle) 100%);
    position: relative;
    overflow: hidden;
  }

  .loading-content {
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2;
  }

  .logo {
    width: 128px;
    height: 128px;
    filter: drop-shadow(0 0 5px var(--text)) drop-shadow(0 0 10px var(--text))
      drop-shadow(0 0 20px var(--blue)) drop-shadow(0 0 40px var(--blue));
    animation: neonPulse 2s ease-in-out infinite;
  }

  @keyframes neonPulse {
    0%,
    100% {
      filter: drop-shadow(0 0 5px var(--text)) drop-shadow(0 0 10px var(--text))
        drop-shadow(0 0 20px var(--blue)) drop-shadow(0 0 40px var(--blue));
    }
    50% {
      filter: drop-shadow(0 0 10px var(--text))
        drop-shadow(0 0 20px var(--text)) drop-shadow(0 0 40px var(--blue))
        drop-shadow(0 0 80px var(--blue));
    }
  }
</style>
