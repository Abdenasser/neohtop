<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";
  import StatsBar from "$lib/components/StatsBar.svelte";
  import ToolBar from "$lib/components/ToolBar.svelte";
  import ProcessTable from "$lib/components/ProcessTable.svelte";
  import ProcessDetailsModal from "$lib/components/ProcessDetailsModal.svelte";
  import KillProcessModal from "$lib/components/KillProcessModal.svelte";
  import { formatMemorySize, formatStatus } from "$lib/utils";
  import { themeStore } from "$lib/stores";
  import type { Process, SystemStats, Column } from "$lib/types";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import { configStore } from "$lib/stores/config";

  let processes: Process[] = [];
  let systemStats: SystemStats | null = null;
  let intervalId: number;
  let error: string | null = null;
  let searchTerm = "";
  let isLoading = true;
  let currentPage = 1;
  let pinnedProcesses: Set<string> = new Set();
  let selectedProcess: Process | null = null;
  let showInfoModal = false;
  let showConfirmModal = false;
  let processToKill: Process | null = null;
  let isKilling = false;
  let isFrozen = false;
  let selectedProcessPid: number | null = null;

  let columnDefinitions: Column[] = [
    { id: "name", label: "Process Name", visible: true, required: true },
    { id: "pid", label: "PID", visible: true, required: false },
    {
      id: "status",
      label: "Status",
      visible: true,
      format: formatStatus,
    },
    { id: "user", label: "User", visible: true },
    {
      id: "cpu_usage",
      label: "CPU %",
      visible: true,
      format: (v) => v.toFixed(1) + "%",
    },
    {
      id: "memory_usage",
      label: "RAM",
      visible: true,
      format: (v) => (v / (1024 * 1024)).toFixed(1) + " MB",
    },
    {
      id: "virtual_memory",
      label: "VIRT",
      visible: true,
      format: (v) => formatMemorySize(v),
    },
    {
      id: "disk_usage",
      label: "Disk R/W",
      visible: true,
      format: (v) =>
        `${(v[0] / (1024 * 1024)).toFixed(1)} / ${(v[1] / (1024 * 1024)).toFixed(1)} MB`,
    },
    { id: "ppid", label: "Parent PID", visible: false },
    { id: "root", label: "Root", visible: false },
    { id: "command", label: "Command", visible: false },
    { id: "environ", label: "Environment Variables", visible: false },
    { id: "session_id", label: "Session ID", visible: false },
    {
      id: "start_time",
      label: "Start Time",
      visible: false,
      format: (v) => new Date(v * 1000).toLocaleString(), // v is the time where the process was started (in seconds) from epoch
    },
    {
      id: "run_time",
      label: "Run Time",
      visible: true,
      format: (v) => {
        const seconds = v; // v is the time the process has been running in seconds
        const hours = Math.floor(seconds / 3600);
        const minutes = Math.floor((seconds % 3600) / 60);
        const remainingSeconds = seconds % 60;
        return `${hours}h ${minutes}m ${remainingSeconds}s`; // Format as HH:MM:SS
      },
    },
  ];

  // Merge column definitions with stored visibility
  $: columns = columnDefinitions.map((col) => ({
    ...col,
    visible:
      col.required ||
      ($configStore.appearance.columnVisibility[col.id] ?? col.visible),
  }));
  $: itemsPerPage = $configStore.behavior.itemsPerPage;
  $: refreshRate = $configStore.behavior.refreshRate;
  $: statusFilter = $configStore.behavior.defaultStatusFilter;

  let sortConfig = {
    field: "cpu_usage" as keyof Process,
    direction: "desc" as "asc" | "desc",
  };

  $: filteredProcesses = processes.filter((process) => {
    let matchesSearch = searchTerm.length === 0;
    searchTerm
      .split(",")
      .map((term) => term.trim())
      .forEach((term) => {
        const nameSubstringMatch = process.name
          .toLowerCase()
          .includes(term.toLowerCase());
        const nameRegexMatch = (() => {
          try {
            return new RegExp(term, "i").test(process.name);
          } catch {
            return false;
          }
        })();
        const commandMatch = process.command
          .toLowerCase()
          .includes(term.toLowerCase());
        const pidMatch = process.pid.toString().includes(term);
        matchesSearch ||=
          nameSubstringMatch || nameRegexMatch || commandMatch || pidMatch;
      });

    const matchesStatus =
      statusFilter === "all" ? true : process.status === statusFilter;

    return matchesSearch && matchesStatus;
  });

  $: sortedProcesses = filteredProcesses.sort((a, b) => {
    const aPin = pinnedProcesses.has(a.command);
    const bPin = pinnedProcesses.has(b.command);
    if (aPin && !bPin) return -1;
    if (!aPin && bPin) return 1;

    const aValue = a[sortConfig.field];
    const bValue = b[sortConfig.field];
    const direction = sortConfig.direction === "asc" ? 1 : -1;

    if (typeof aValue === "string" && typeof bValue === "string") {
      return direction * aValue.localeCompare(bValue);
    }
    return direction * (Number(aValue) - Number(bValue));
  });

  $: totalPages = Math.ceil(filteredProcesses.length / itemsPerPage);
  $: paginatedProcesses = sortedProcesses.slice(
    (currentPage - 1) * itemsPerPage,
    currentPage * itemsPerPage,
  );

  $: {
    // Reset to first page when filtering or changing items per page
    if (searchTerm || itemsPerPage) {
      currentPage = 1;
    }
  }

  $: {
    if (intervalId) clearInterval(intervalId);
    if (!isFrozen) {
      intervalId = setInterval(() => {
        getProcesses();
      }, refreshRate);
    }
  }

  $: if (selectedProcessPid && processes.length > 0) {
    selectedProcess =
      processes.find((p) => p.pid === selectedProcessPid) || null;
  }

  async function getProcesses() {
    try {
      const result = await invoke<[Process[], SystemStats]>("get_processes");
      processes = result[0];
      systemStats = result[1];
      error = null;
    } catch (e: unknown) {
      if (e instanceof Error) {
        error = e.message;
      } else {
        error = String(e);
      }
    }
  }

  async function killProcess(pid: number) {
    try {
      const success = await invoke<boolean>("kill_process", { pid });
      if (success) {
        await getProcesses();
      }
    } catch (e: unknown) {
      if (e instanceof Error) {
        error = e.message;
      } else {
        error = String(e);
      }
    }
  }

  function toggleSort(field: keyof Process) {
    if (sortConfig.field === field) {
      sortConfig.direction = sortConfig.direction === "asc" ? "desc" : "asc";
    } else {
      sortConfig.field = field;
      sortConfig.direction = "desc";
    }
  }

  function togglePin(command: string) {
    if (pinnedProcesses.has(command)) {
      pinnedProcesses.delete(command);
    } else {
      pinnedProcesses.add(command);
    }
    pinnedProcesses = pinnedProcesses; // Trigger reactivity
  }

  function showProcessDetails(process: Process) {
    selectedProcessPid = process.pid;
    selectedProcess = process;
    showInfoModal = true;
  }

  function confirmKillProcess(process: Process) {
    processToKill = process;
    showConfirmModal = true;
  }

  async function handleConfirmKill() {
    if (processToKill) {
      isKilling = true;
      try {
        await killProcess(processToKill.pid);
      } finally {
        isKilling = false;
        showConfirmModal = false;
        processToKill = null;
      }
    }
  }

  function handleModalClose() {
    showInfoModal = false;
    selectedProcess = null;
    selectedProcessPid = null;
  }

  let minLoadingTimer: ReturnType<typeof setTimeout>;
  const MIN_LOADING_TIME = 2000; // Show loading screen for at least 2 seconds

  onMount(async () => {
    const loadingPromise = Promise.all([getProcesses()]);
    const timerPromise = new Promise((resolve) => {
      minLoadingTimer = setTimeout(resolve, MIN_LOADING_TIME);
    });

    try {
      // Wait for both the data to load AND the minimum time to pass
      await Promise.all([loadingPromise, timerPromise]);
    } finally {
      isLoading = false;
    }

    configStore.init();
    themeStore.init();
  });

  onDestroy(() => {
    if (intervalId) clearInterval(intervalId);
    if (minLoadingTimer) clearTimeout(minLoadingTimer);
  });
</script>

{#if isLoading}
  <div class="loading-container">
    <div class="loading-content">
      <div class="title-wrapper">
        <div class="neon-title">NeoHtop</div>
        <div class="neon-flare"></div>
      </div>
      <div class="cyber-spinner"></div>
      <span class="loading-text">System Initialization...</span>
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
        bind:searchTerm
        bind:statusFilter
        bind:itemsPerPage
        bind:currentPage
        bind:refreshRate
        bind:isFrozen
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
        onToggleSort={toggleSort}
        onTogglePin={togglePin}
        onShowDetails={showProcessDetails}
        onKillProcess={confirmKillProcess}
      />
    </main>
  </div>
{/if}

<ProcessDetailsModal
  show={showInfoModal}
  process={selectedProcess}
  onClose={handleModalClose}
/>

<KillProcessModal
  show={showConfirmModal}
  process={processToKill}
  {isKilling}
  onClose={() => {
    showConfirmModal = false;
    processToKill = null;
  }}
  onConfirm={handleConfirmKill}
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
    flex-direction: column;
    align-items: center;
    gap: 16px;
    z-index: 2;
  }

  @keyframes glitch {
    0%,
    100% {
      transform: translate(0);
    }
    20% {
      transform: translate(-2px, 2px);
    }
    40% {
      transform: translate(-2px, -2px);
    }
    60% {
      transform: translate(2px, 2px);
    }
    80% {
      transform: translate(2px, -2px);
    }
  }

  @keyframes glitch-2 {
    0%,
    100% {
      transform: translate(0);
    }
    25% {
      transform: translate(1px, 1px);
    }
    75% {
      transform: translate(-1px, -1px);
    }
  }

  @keyframes glitch-3 {
    0%,
    100% {
      transform: translate(0);
    }
    25% {
      transform: translate(-1px, -1px);
    }
    75% {
      transform: translate(1px, 1px);
    }
  }

  @keyframes scanlines {
    from {
      transform: translateY(0);
    }
    to {
      transform: translateY(4px);
    }
  }

  .loading-text {
    color: var(--subtext0);
    font-size: 14px;
    font-weight: 500;
    letter-spacing: 0.5px;
    animation: pulse 2s ease-in-out infinite;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 0.6;
    }
    50% {
      opacity: 1;
    }
  }

  .app-container {
    height: 100vh;
    display: flex;
    flex-direction: column;
  }

  .title-wrapper {
    position: relative;
    margin-bottom: 2rem;
  }

  .neon-title {
    font-family: "Courier New", monospace;
    font-size: 64px;
    font-weight: bold;
    color: var(--text);
    text-shadow:
      0 0 5px var(--text),
      0 0 10px var(--text),
      0 0 20px var(--blue),
      0 0 40px var(--blue),
      0 0 80px var(--blue);
    animation: pulse 4s ease-in-out infinite;
  }

  .neon-flare {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(
      45deg,
      transparent 45%,
      var(--blue) 48%,
      var(--text) 50%,
      var(--blue) 52%,
      transparent 55%
    );
    background-size: 200% 200%;
    animation: flare 4s ease-in-out infinite;
    opacity: 0.1;
    filter: blur(3px);
  }

  .cyber-spinner {
    width: 50px;
    height: 50px;
    margin: 20px;
    background: transparent;
    border: 3px solid var(--surface0);
    border-top: 3px solid var(--blue);
    border-right: 3px solid var(--blue);
    border-radius: 50%;
    position: relative;
    animation: spin 1s linear infinite;
  }

  .cyber-spinner::before {
    content: "";
    position: absolute;
    top: -3px;
    left: -3px;
    right: -3px;
    bottom: -3px;
    border-radius: 50%;
    border: 3px solid transparent;
    border-top-color: var(--blue);
    animation: spin 2s linear infinite;
  }

  .loading-text {
    color: var(--text);
    font-size: 16px;
    font-family: "Courier New", monospace;
    letter-spacing: 2px;
    animation: glow 2s ease-in-out infinite alternate;
  }

  @keyframes pulse {
    0%,
    100% {
      text-shadow:
        0 0 5px var(--text),
        0 0 10px var(--text),
        0 0 20px var(--blue),
        0 0 40px var(--blue),
        0 0 80px var(--blue);
    }
    50% {
      text-shadow:
        0 0 10px var(--text),
        0 0 20px var(--text),
        0 0 40px var(--blue),
        0 0 80px var(--blue),
        0 0 120px var(--blue);
    }
  }

  @keyframes flare {
    0%,
    100% {
      background-position: 200% 200%;
    }
    50% {
      background-position: 0% 0%;
    }
  }

  @keyframes glow {
    from {
      text-shadow:
        0 0 2px var(--text),
        0 0 4px var(--text),
        0 0 6px var(--blue);
    }
  }
</style>
