export interface AppConfig {
  appearance: {
    columnVisibility: Record<string, boolean>;
  };
  behavior: {
    itemsPerPage: number;
    refreshRate: number;
    defaultStatusFilter: string;
  };
}

export const DEFAULT_CONFIG: AppConfig = {
  appearance: {
    columnVisibility: {
      name: true,
      pid: true,
      status: true,
      user: true,
      cpu_usage: true,
      memory_usage: true,
      virtual_memory: true,
      disk_usage: true,
      ppid: false,
      root: false,
      command: false,
      environ: false,
      session_id: false,
      start_time: false,
      run_time: true,
    },
  },
  behavior: {
    itemsPerPage: 15,
    refreshRate: 1000,
    defaultStatusFilter: "all",
  },
};
