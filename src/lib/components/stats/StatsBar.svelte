<script lang="ts">
  import GpuPanel from "./GpuPanel.svelte";
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
  import type { SystemStats, ResourceSample, GpuDevice } from "$lib/types";
  import {
    CpuPanel,
    MemoryPanel,
    StoragePanel,
    SystemPanel,
    NetworkPanel,
  } from "$lib/components";
  export let systemStats: SystemStats | null = null;
  export let history: ResourceSample[] = [];
  export let gpus: GpuDevice[] = [];
  export let capturedAt = 0;
  export let isFrozen = false;
  export let unavailable = false;
</script>

{#if systemStats}<Tooltip.Provider delayDuration={150}
    ><section class="resource-grid shrink-0" aria-label="System resources">
      <CpuPanel
        stats={systemStats}
        {capturedAt}
        cpuUsage={systemStats.cpu_usage}
        {history}
        {isFrozen}
        {unavailable}
      /><MemoryPanel
        stats={systemStats}
        {capturedAt}
        {history}
        {isFrozen}
        {unavailable}
        memoryTotal={systemStats.memory_total}
        memoryUsed={systemStats.memory_used}
        memoryFree={systemStats.memory_free}
      /><GpuPanel
        stats={systemStats}
        {capturedAt}
        {gpus}
        {history}
        {isFrozen}
        {unavailable}
      /><StoragePanel
        stats={systemStats}
        {capturedAt}
        {isFrozen}
        {unavailable}
        diskTotalBytes={systemStats.disk_total_bytes}
        diskUsedBytes={systemStats.disk_used_bytes}
        diskFreeBytes={systemStats.disk_free_bytes}
      /><SystemPanel
        stats={systemStats}
        {capturedAt}
        {isFrozen}
        {unavailable}
        uptime={systemStats.uptime}
        loadAvg={systemStats.load_avg}
      /><NetworkPanel
        stats={systemStats}
        {capturedAt}
        {history}
        {isFrozen}
        {unavailable}
        networkRxBytes={systemStats.network_rx_bytes}
        networkTxBytes={systemStats.network_tx_bytes}
      />
    </section></Tooltip.Provider
  >{/if}
