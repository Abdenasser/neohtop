import { writable } from "svelte/store";
import type {
  Process,
  SystemStats,
  ResourceSample,
  GpuDevice,
  MonitorSnapshot,
} from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

interface ProcessStore {
  resourceHistory: ResourceSample[];
  processes: Process[];
  systemStats: SystemStats | null;
  gpus: GpuDevice[];
  capturedAt: number;
  error: string | null;
  isLoading: boolean;
  searchTerm: string;
  currentPage: number;
  pinnedProcesses: Set<string>;
  selectedProcess: Process | null;
  showInfoModal: boolean;
  showConfirmModal: boolean;
  processToKill: Process | null;
  isKilling: boolean;
  isFrozen: boolean;
  selectedProcessPid: number | null;
  sortConfig: {
    field: keyof Process;
    direction: "asc" | "desc";
  };
}

const initialState: ProcessStore = {
  resourceHistory: [],
  processes: [],
  systemStats: null,
  gpus: [],
  capturedAt: 0,
  error: null,
  isLoading: true,
  searchTerm: "",
  currentPage: 1,
  pinnedProcesses: new Set(),
  selectedProcess: null,
  showInfoModal: false,
  showConfirmModal: false,
  processToKill: null,
  isKilling: false,
  isFrozen: false,
  selectedProcessPid: null,
  sortConfig: {
    field: "cpu_usage",
    direction: "desc",
  },
};

function createProcessStore() {
  const { subscribe, set, update } = writable<ProcessStore>(initialState);

  // Define all methods first
  const setIsLoading = (isLoading: boolean) =>
    update((state) => ({ ...state, isLoading }));

  let inFlight: Promise<void> | null = null;
  const readProcesses = async () => {
    try {
      const result = await invoke<MonitorSnapshot>("get_processes");
      update((state) => ({
        ...state,
        processes: result.frame.processes,
        systemStats: result.frame.stats,
        gpus: result.frame.gpus,
        capturedAt: result.frame.captured_at,
        resourceHistory: result.history,
        isFrozen: result.paused,
        error: result.error,
        selectedProcess: state.selectedProcess
          ? (result.frame.processes.find(
              (p) =>
                p.pid === state.selectedProcess?.pid &&
                p.start_time === state.selectedProcess?.start_time,
            ) ?? null)
          : null,
      }));
    } catch (e: unknown) {
      update((state) => ({
        ...state,
        error: e instanceof Error ? e.message : String(e),
      }));
    }
  };

  // Share an in-flight read so manual actions and timer ticks cannot reorder history.
  const getProcesses = () => {
    if (inFlight) return inFlight;
    inFlight = readProcesses().finally(() => {
      inFlight = null;
    });
    return inFlight;
  };

  const killProcess = async (pid: number) => {
    try {
      update((state) => ({ ...state, isKilling: true }));
      const success = await invoke<boolean>("kill_process", { pid });
      if (success) {
        await getProcesses();
      } else {
        throw new Error("Failed to kill process");
      }
    } catch (e: unknown) {
      update((state) => ({
        ...state,
        error: e instanceof Error ? e.message : String(e),
      }));
    } finally {
      update((state) => ({ ...state, isKilling: false }));
    }
  };

  const toggleSort = (field: keyof Process) => {
    update((state) => ({
      ...state,
      sortConfig: {
        field,
        direction:
          state.sortConfig.field === field
            ? state.sortConfig.direction === "asc"
              ? "desc"
              : "asc"
            : "desc",
      },
    }));
  };

  const togglePin = (command: string) => {
    update((state) => {
      const newPinnedProcesses = new Set(state.pinnedProcesses);
      if (newPinnedProcesses.has(command)) {
        newPinnedProcesses.delete(command);
      } else {
        newPinnedProcesses.add(command);
      }
      return { ...state, pinnedProcesses: newPinnedProcesses };
    });
  };

  const setSearchTerm = (searchTerm: string) =>
    update((state) => ({ ...state, searchTerm, currentPage: 1 }));

  const refreshControl = async () => {
    if (inFlight) await inFlight;
    await getProcesses();
  };

  const setIsFrozen = async (isFrozen: boolean) => {
    try {
      await invoke("set_monitor_paused", { paused: isFrozen });
      await refreshControl();
    } catch (e: unknown) {
      update((state) => ({
        ...state,
        error: e instanceof Error ? e.message : String(e),
      }));
    }
  };

  const setCurrentPage = (currentPage: number) =>
    update((state) => ({ ...state, currentPage }));

  const showProcessDetails = (process: Process) => {
    update((state) => ({
      ...state,
      selectedProcessPid: process.pid,
      selectedProcess: process,
      showInfoModal: true,
    }));
  };

  const closeProcessDetails = () => {
    update((state) => ({
      ...state,
      showInfoModal: false,
      selectedProcess: null,
      selectedProcessPid: null,
    }));
  };

  const confirmKillProcess = (process: Process) => {
    update((state) => ({
      ...state,
      processToKill: process,
      showConfirmModal: true,
    }));
  };

  const closeConfirmKill = () => {
    update((state) => ({
      ...state,
      showConfirmModal: false,
      processToKill: null,
    }));
  };

  const handleConfirmKill = async () => {
    let processToKill: Process | null = null;

    let currentState: ProcessStore | undefined;
    const unsubscribe = subscribe((state) => {
      currentState = state;
    });
    unsubscribe();

    if (currentState?.processToKill && "pid" in currentState.processToKill) {
      processToKill = currentState.processToKill;
    }

    if (!processToKill?.pid) {
      return;
    }

    try {
      await killProcess(processToKill.pid);
    } finally {
      update((state) => ({
        ...state,
        showConfirmModal: false,
        processToKill: null,
      }));
    }
  };

  // Return all methods
  return {
    subscribe,
    set,
    update,
    setIsLoading,
    getProcesses,
    refreshControl,
    killProcess,
    toggleSort,
    togglePin,
    setSearchTerm,
    setIsFrozen,
    setCurrentPage,
    showProcessDetails,
    closeProcessDetails,
    confirmKillProcess,
    closeConfirmKill,
    handleConfirmKill,
  };
}

export const processStore = createProcessStore();
