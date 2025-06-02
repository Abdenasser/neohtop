import type { Process } from "$lib/types";
import type { SortConfig } from "$lib/types";

export interface ProcessStatus {
  label: string;
  emoji: string;
  color: string;
}

export function formatMemorySize(bytes: number): string {
  const gb = bytes / (1024 * 1024 * 1024);
  return `${gb.toFixed(1)} GB`;
}

export function formatPercentage(value: number): string {
  return `${value.toFixed(1)}%`;
}

export function formatUptime(seconds: number): string {
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  return `${days}d ${hours}h ${minutes}m`;
}

export function getUsageClass(percentage: number): string {
  if (percentage >= 90) return "critical";
  if (percentage >= 60) return "high";
  if (percentage >= 30) return "medium";
  return "low";
}

export function formatBytes(bytes: number): string {
  const units = ["B", "KB", "MB", "GB", "TB"];
  let value = bytes;
  let unitIndex = 0;

  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024;
    unitIndex++;
  }

  return `${value.toFixed(1)} ${units[unitIndex]}`;
}

export function formatDate(timestamp: number) {
  return new Date(timestamp * 1000).toLocaleString();
}

// Debounce utility to reduce frequency of expensive operations
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number,
): (...args: Parameters<T>) => void {
  let timeout: NodeJS.Timeout;
  return (...args: Parameters<T>) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => func(...args), wait);
  };
}

// Cache for compiled regex patterns
const regexCache = new Map<string, RegExp>();

export function filterProcesses(
  processes: Process[],
  searchTerm: string,
  filters: {
    cpu: { operator: string; value: number; enabled: boolean };
    ram: { operator: string; value: number; enabled: boolean };
    runtime: { operator: string; value: number; enabled: boolean };
    status: { values: string[]; enabled: boolean };
  },
): Process[] {
  // Early return for empty search and no active filters
  if (
    searchTerm.length === 0 &&
    !Object.values(filters).some((f) => f.enabled)
  ) {
    return processes;
  }

  // Pre-process search terms once
  const terms =
    searchTerm.length > 0
      ? searchTerm.split(",").map((term) => term.trim())
      : [];

  return processes.filter((process) => {
    // Apply status filter
    if (filters.status.enabled && filters.status.values.length > 0) {
      if (!filters.status.values.includes(process.status)) {
        return false;
      }
    }

    // Apply CPU filter
    if (filters.cpu.enabled) {
      const cpuValue = process.cpu_usage;
      if (!compareValue(cpuValue, filters.cpu.operator, filters.cpu.value)) {
        return false;
      }
    }

    // Apply RAM filter (convert bytes to MB)
    if (filters.ram.enabled) {
      const ramMB = process.memory_usage / (1024 * 1024);
      if (!compareValue(ramMB, filters.ram.operator, filters.ram.value)) {
        return false;
      }
    }

    // Apply runtime filter (convert to minutes)
    if (filters.runtime.enabled) {
      const runtimeMin = process.run_time / 60;
      if (
        !compareValue(
          runtimeMin,
          filters.runtime.operator,
          filters.runtime.value,
        )
      ) {
        return false;
      }
    }

    // Skip search if no terms
    if (terms.length === 0) {
      return true;
    }

    // Cache lowercase values
    const processNameLower = process.name.toLowerCase();
    const processCommandLower = process.command.toLowerCase();
    const processPidString = process.pid.toString();

    // Check each term
    return terms.some((term) => {
      const termLower = term.toLowerCase();

      // Try exact matches first (faster)
      if (
        processNameLower.includes(termLower) ||
        processCommandLower.includes(termLower) ||
        processPidString.includes(term)
      ) {
        return true;
      }

      // Try regex match last (slower)
      try {
        let regex = regexCache.get(term);
        if (!regex) {
          regex = new RegExp(term, "i");
          regexCache.set(term, regex);
        }
        return regex.test(process.name);
      } catch {
        return false;
      }
    });
  });
}

// Helper function to compare values based on operator
function compareValue(
  value: number,
  operator: string,
  target: number,
): boolean {
  switch (operator) {
    case ">":
      return value > target;
    case "<":
      return value < target;
    case "=":
      return value === target;
    case ">=":
      return value >= target;
    case "<=":
      return value <= target;
    default:
      return true;
  }
}

// Create a Map for quick pinned status lookup
const isPinned = new Map<string, boolean>();

export function sortProcesses(
  processes: Process[],
  sortConfig: SortConfig,
  pinnedProcesses: Set<string>,
): Process[] {
  // Clear the cache before sorting
  isPinned.clear();

  return [...processes].sort((a, b) => {
    // Cache pinned status
    let aPin = pinnedProcesses.has(a.command);
    isPinned.set(a.command, aPin);

    let bPin = pinnedProcesses.has(b.command);
    isPinned.set(b.command, bPin);

    // Quick pin comparison
    if (aPin !== bPin) {
      return aPin ? -1 : 1;
    }

    // Only compute direction once
    const direction = sortConfig.direction === "asc" ? 1 : -1;
    const aValue = a[sortConfig.field];
    const bValue = b[sortConfig.field];

    // Special handling for disk_usage which is an array [read_bytes, written_bytes]
    if (sortConfig.field === "disk_usage") {
      const aRead = (aValue as [number, number])[0];
      const aWrite = (aValue as [number, number])[1];
      const bRead = (bValue as [number, number])[0];
      const bWrite = (bValue as [number, number])[1];

      // Smart sorting: analyze if this is a read-heavy or write-heavy comparison
      const totalReads = aRead + bRead;
      const totalWrites = aWrite + bWrite;

      if (totalWrites > totalReads * 1.5) {
        // Write-heavy scenario: prioritize writes, use reads as tiebreaker
        if (aWrite !== bWrite) {
          return direction * (aWrite - bWrite);
        }
        return direction * (aRead - bRead);
      } else if (totalReads > totalWrites * 1.5) {
        // Read-heavy scenario: prioritize reads, use writes as tiebreaker
        if (aRead !== bRead) {
          return direction * (aRead - bRead);
        }
        return direction * (aWrite - bWrite);
      } else {
        // Balanced I/O: sort by total, use max as tiebreaker
        const aTotalDisk = aRead + aWrite;
        const bTotalDisk = bRead + bWrite;
        if (aTotalDisk !== bTotalDisk) {
          return direction * (aTotalDisk - bTotalDisk);
        }
        // Tiebreaker: use the dominant operation
        const aMaxDisk = Math.max(aRead, aWrite);
        const bMaxDisk = Math.max(bRead, bWrite);
        return direction * (aMaxDisk - bMaxDisk);
      }
    }

    // Type-specific comparisons
    if (typeof aValue === "string") {
      return direction * aValue.localeCompare(bValue as string);
    }
    return direction * (Number(aValue) - Number(bValue));
  });
}
