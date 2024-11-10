export interface ProcessStatus {
  label: string;
  emoji: string;
  color: string;
}

export const statusMap: Record<string, ProcessStatus> = {
  Running: {
    label: "Running",
    emoji: "🏃",
    color: "var(--green)",
  },
  Sleeping: {
    label: "Sleeping",
    emoji: "😴",
    color: "var(--blue)",
  },
  Idle: {
    label: "Idle",
    emoji: "⌛",
    color: "var(--overlay0)",
  },
  Unknown: {
    label: "Unknown",
    emoji: "❓",
    color: "var(--overlay0)",
  },
};

export function formatStatus(status: string): string {
  const processStatus = statusMap[status] || statusMap.Unknown;
  return `<span class="status-badge" style="--status-color: ${processStatus.color}">
    ${processStatus.label}
  </span>`;
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
