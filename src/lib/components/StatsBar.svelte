<script lang="ts">
  import Fa from "svelte-fa";
  import {
    faMicrochip,
    faMemory,
    faServer,
    faNetworkWired,
    faHardDrive,
  } from "@fortawesome/free-solid-svg-icons";
  import type { SystemStats } from "$lib/types";
  import {
    formatUptime,
    formatMemorySize,
    formatPercentage,
    getUsageClass,
  } from "$lib/utils";

  export let systemStats: SystemStats | null = null;
  $: memoryPercentage = systemStats
    ? (systemStats.memory_used / systemStats.memory_total) * 100
    : 0;

  function formatBytes(bytes: number): string {
    const units = ["B", "KB", "MB", "GB", "TB"];
    let value = bytes;
    let unitIndex = 0;

    while (value >= 1024 && unitIndex < units.length - 1) {
      value /= 1024;
      unitIndex++;
    }

    return `${value.toFixed(1)} ${units[unitIndex]}`;
  }

  $: diskUsagePercentage = systemStats
    ? (systemStats.disk_used_bytes / systemStats.disk_total_bytes) * 100
    : 0;
</script>

<div class="dashboard-stats">
  {#if systemStats}
    <div class="stats-layout">
      <!-- CPU Panel -->
      <div class="stat-panel">
        <div class="panel-header">
          <Fa icon={faMicrochip} />
          <h3>CPU Usage</h3>
          <div class="usage-pill">
            {formatPercentage(
              systemStats.cpu_usage.reduce((a, b) => a + b, 0) /
                systemStats.cpu_usage.length,
            )}
          </div>
        </div>
        <div class="stats-content cpu-grid">
          {#each systemStats.cpu_usage as usage, i}
            <div class="stat-item with-progress">
              <div class="progress-container">
                <span class="label">Core {i}</span>
                <div class="bar-container">
                  <div
                    class="usage-bar {getUsageClass(usage)}"
                    style="transform: translateX({usage - 100}%);"
                  ></div>
                </div>
                <span class="value">{Math.round(usage)}%</span>
              </div>
            </div>
          {/each}
        </div>
      </div>

      <!-- Memory Panel -->
      <div class="stat-panel">
        <div class="panel-header">
          <Fa icon={faMemory} />
          <h3>Memory</h3>
          <div class="usage-pill">{formatPercentage(memoryPercentage)}</div>
        </div>
        <div class="stats-content">
          <div class="stat-item with-progress">
            <div class="memory-progress-container">
              <span class="label">Memory usage</span>
              <div class="bar-container">
                <div
                  class="usage-bar {getUsageClass(memoryPercentage)}"
                  style="transform: translateX({memoryPercentage - 100}%);"
                ></div>
              </div>
              <span class="value">{formatPercentage(memoryPercentage)}</span>
            </div>
          </div>
          <div class="stat-item">
            <span>Total</span>
            <span>{formatMemorySize(systemStats.memory_total)}</span>
          </div>
          <div class="stat-item">
            <span>Used</span>
            <span>{formatMemorySize(systemStats.memory_used)}</span>
          </div>
          <div class="stat-item">
            <span>Free</span>
            <span>{formatMemorySize(systemStats.memory_free)}</span>
          </div>
        </div>
      </div>

      <!-- Storage Panel -->
      <div class="stat-panel">
        <div class="panel-header">
          <Fa icon={faHardDrive} />
          <h3>Storage</h3>
          <div class="usage-pill">
            {formatPercentage(diskUsagePercentage)}
          </div>
        </div>
        <div class="stats-content">
          <div class="stat-item">
            <span>Total</span>
            <span>{formatBytes(systemStats.disk_total_bytes)}</span>
          </div>
          <div class="stat-item">
            <span>Used</span>
            <span>{formatBytes(systemStats.disk_used_bytes)}</span>
          </div>
          <div class="stat-item">
            <span>Free</span>
            <span>{formatBytes(systemStats.disk_free_bytes)}</span>
          </div>
        </div>
      </div>

      <!-- System Panel -->
      <div class="stat-panel">
        <div class="panel-header">
          <Fa icon={faServer} />
          <h3>System</h3>
        </div>
        <div class="system-grid">
          <div class="stat-item">
            <span>Uptime</span>
            <span>{formatUptime(systemStats.uptime)}</span>
          </div>
          <div class="stat-item">
            <span>1m Load</span>
            <span>{systemStats.load_avg[0].toFixed(2)}</span>
          </div>
          <div class="stat-item">
            <span>5m Load</span>
            <span>{systemStats.load_avg[1].toFixed(2)}</span>
          </div>
          <div class="stat-item">
            <span>15m Load</span>
            <span>{systemStats.load_avg[2].toFixed(2)}</span>
          </div>
        </div>
      </div>

      <!-- Network Panel -->
      <div class="stat-panel">
        <div class="panel-header">
          <Fa icon={faNetworkWired} />
          <h3>Network I/O</h3>
        </div>
        <div class="network-stats">
          <div class="stat-item">
            <span>↓ Receiving</span>
            <span>{formatBytes(systemStats.network_rx_bytes)}/s</span>
          </div>
          <div class="stat-item">
            <span>↑ Sending</span>
            <span>{formatBytes(systemStats.network_tx_bytes)}/s</span>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .dashboard-stats {
    background: var(--base);
    padding: 0.5rem;
    border-radius: 8px;
    overflow-x: auto;
  }

  .stats-layout {
    display: flex;
    gap: 0.75rem;
    width: 100%;
  }

  .stat-panel {
    flex: 1;
    min-width: 0;
    background: var(--mantle);
    border-radius: 6px;
    padding: 0.75rem;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
    display: flex;
    flex-direction: column;
  }

  .panel-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.75rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--surface0);
    flex-shrink: 0;
  }

  .panel-header h3 {
    font-size: 0.8rem;
    font-weight: 600;
    margin: 0;
    color: var(--text);
  }

  .panel-header :global(svg) {
    color: var(--blue);
    width: 0.8rem;
    height: 0.8rem;
  }

  .usage-pill {
    margin-left: auto;
    background: var(--surface0);
    padding: 0.15rem 0.5rem;
    border-radius: 1rem;
    font-size: 0.7rem;
    font-weight: 500;
  }

  .cpu-layout {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.75rem;
    overflow-y: auto;
    padding-right: 0.25rem;
  }

  .cpu-row {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
  }

  .cpu-info {
    display: flex;
    justify-content: space-between;
    font-size: 0.7rem;
  }

  .bar-container {
    height: 8px;
    background: var(--surface0);
    border-radius: 4px;
    overflow: hidden;
    position: relative;
  }

  .bar-container.main-bar {
    height: 6px;
    margin-bottom: 0.5rem;
  }

  .usage-bar {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 100%;
    transition: transform 0.3s ease;
    transform-origin: left;
  }

  .usage-bar.low {
    background: var(--blue);
  }
  .usage-bar.medium {
    background: var(--yellow);
  }
  .usage-bar.high {
    background: var(--peach);
  }
  .usage-bar.critical {
    background: var(--red);
  }

  .memory-grid,
  .disk-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.75rem;
    align-content: start;
  }

  .system-grid,
  .network-grid {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    flex: 1;
  }

  .load-stats {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .network-grid {
    display: grid;
    grid-template-columns: 1fr;
  }

  .stat-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.7rem;
    line-height: 1.2;
    margin: 0;
    padding: 0;
  }

  .stat-item span:first-child {
    color: var(--subtext0);
  }

  .stat-item span:last-child {
    color: var(--text);
    font-weight: 500;
  }

  .full-width {
    grid-column: 1 / -1;
  }

  .network-stats {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .stats-content {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .stat-item.with-progress {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
  }

  .stat-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.7rem;
    gap: 1rem;
  }

  .progress-container {
    width: 100%;
    display: grid;
    grid-template-columns: 4rem 1fr 3rem;
    align-items: center;
    gap: 0.75rem;
    font-size: 0.7rem;
  }

  .memory-progress-container {
    width: 100%;
    display: grid;
    grid-template-columns: 5rem 1fr 2.5rem;
    align-items: center;
    gap: 0.3rem;
    font-size: 0.7rem;
  }

  .memory-progress-container .label {
    color: var(--subtext0);
    white-space: nowrap;
  }

  .memory-progress-container .value {
    color: var(--text);
    text-align: right;
    white-space: nowrap;
  }

  .progress-container .label {
    color: var(--subtext0);
    white-space: nowrap;
  }

  .progress-container .value {
    color: var(--text);
    text-align: right;
    white-space: nowrap;
  }

  .bar-container {
    height: 8px;
    background: var(--surface0);
    border-radius: 4px;
    overflow: hidden;
    position: relative;
  }

  .usage-bar {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 100%;
    transition: transform 0.3s ease;
    transform-origin: left;
  }

  .cpu-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.4rem 2rem;
    height: auto;
    overflow: visible;
  }

  .progress-container {
    width: 100%;
    display: grid;
    grid-template-columns: 2.5rem 1fr 2.5rem;
    align-items: center;
    gap: 0.3rem;
    font-size: 0.7rem;
  }

  /* Make CPU panel take more space */
  .stat-panel:first-child {
    flex: 2;
  }

  /* Customize flex distribution for each panel */
  .stat-panel:nth-child(1) {
    flex: 2.5; /* CPU: more space for 2 columns */
  }
  .stat-panel:nth-child(2) {
    flex: 2; /* Memory: more space for details */
  }
  .stat-panel:nth-child(3),
  .stat-panel:nth-child(4),
  .stat-panel:nth-child(5) {
    flex: 0.8; /* Storage, System, and Network: less space */
    min-width: 125px;
  }
</style>
