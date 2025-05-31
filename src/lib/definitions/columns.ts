import type { Column } from "$lib/types";
import { formatMemorySize } from "$lib/utils";

export let column_definitions: Column[] = [
  { id: "name", label: "Process Name", visible: true, required: true },
  { id: "pid", label: "PID", visible: true, required: false },
  {
    id: "status",
    label: "Status",
    visible: true,
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
    label: "Disk I/O (R/W)",
    visible: true,
    format: (v) => {
      const readMB = (v[0] / (1024 * 1024)).toFixed(1);
      const writeMB = (v[1] / (1024 * 1024)).toFixed(1);
      return `${readMB}/${writeMB} MB`;
    },
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
