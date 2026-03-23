// Create a new types file to centralize interfaces
export interface Process {
  pid: number;
  ppid: number;
  name: string;
  cpu_usage: number;
  memory_usage: number;
  status: string;
  user: string;
  command: string;
  threads?: number;
  environ: string[];
  root: string;
  virtual_memory: number;
  start_time: number;
  run_time: number;
  disk_usage: [number, number]; // [read_bytes, written_bytes]
  session_id?: number;
}

export interface SystemStats {
  cpu_usage: number[];
  memory_total: number;
  memory_used: number;
  memory_free: number;
  memory_cached: number;
  uptime: number;
  load_avg: [number, number, number];
  network_rx_bytes: number;
  network_tx_bytes: number;
  disk_total_bytes: number;
  disk_used_bytes: number;
  disk_free_bytes: number;
}

export interface Column {
  id: keyof Process;
  label: string;
  visible: boolean;
  required?: boolean;
  format?: (value: any) => string;
}

export interface Theme {
  name: string;
  label: string;
  colors: {
    base: string;
    mantle: string;
    crust: string;
    text: string;
    subtext0: string;
    subtext1: string;
    surface0: string;
    surface1: string;
    surface2: string;
    overlay0: string;
    overlay1: string;
    blue: string;
    lavender: string;
    sapphire: string;
    sky: string;
    red: string;
    maroon: string;
    peach: string;
    yellow: string;
    green: string;
    teal: string;
  };
}

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

export interface ColumnDefinition {
  id: string;
  label: string;
  visible: boolean;
  required?: boolean;
}

export interface StatusOption {
  value: string;
  label: string;
}

export interface RefreshRateOption {
  value: number;
  label: string;
}

export interface ToolBarProps {
  searchTerm: string;
  statusFilter: string;
  itemsPerPage: number;
  currentPage: number;
  totalPages: number;
  totalResults: number;
  columns: ColumnDefinition[];
  refreshRate: number;
  isFrozen: boolean;
}

export interface SortConfig {
  field: keyof Process;
  direction: "asc" | "desc";
}

export interface VTReport {
  /** SHA-256 hex hash of the executable */
  hash: string;
  /** Overall verdict: clean | malicious | suspicious | unknown */
  verdict: "clean" | "malicious" | "suspicious" | "unknown";
  /** Number of AV engines that flagged as malicious */
  malicious: number;
  /** Number of AV engines that flagged as suspicious */
  suspicious: number;
  /** Number of AV engines that returned undetected */
  undetected: number;
  /** Total number of AV engines in the scan */
  total: number;
  /** Direct link to the VirusTotal report */
  permalink: string;
}
