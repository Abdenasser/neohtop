<script lang="ts">
  import Modal from "./Modal.svelte";
  import { formatStatus, formatUptime } from "$lib/utils";
  import type { Process } from "$lib/types";
  import Fa from "svelte-fa";
  import {
    faClock,
    faMemory,
    faMicrochip,
    faHardDrive,
    faNetworkWired,
  } from "@fortawesome/free-solid-svg-icons";

  export let show = false;
  export let process: Process | null = null;
  export let onClose: () => void;

  $: currentProcess = process;

  function formatMemory(bytes: number) {
    return (bytes / (1024 * 1024)).toFixed(1) + " MB";
  }

  function formatBytes(bytes: number) {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    if (bytes < 1024 * 1024 * 1024)
      return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
    return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
  }

  function formatDate(timestamp: number) {
    return new Date(timestamp * 1000).toLocaleString();
  }
</script>

<Modal {show} title="Process Details" maxWidth="700px" {onClose}>
  {#if currentProcess}
    <div class="process-details">
      <!-- Basic Info Section -->
      <section class="detail-section">
        <h3>Basic Information</h3>
        <div class="detail-grid">
          <div class="detail-row">
            <span class="detail-label">Name:</span>
            <span class="detail-value">{currentProcess.name}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">PID:</span>
            <span class="detail-value">{currentProcess.pid}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Parent PID:</span>
            <span class="detail-value">{currentProcess.ppid}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">User:</span>
            <span class="detail-value">{currentProcess.user}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Status:</span>
            <span class="detail-value">
              {@html formatStatus(currentProcess.status)}
            </span>
          </div>
        </div>
      </section>

      <!-- Resource Usage Section -->
      <section class="detail-section">
        <h3>Resource Usage</h3>
        <div class="resource-grid">
          <!-- CPU Usage -->
          <div class="resource-card">
            <div class="resource-header">
              <Fa icon={faMicrochip} />
              <span>CPU Usage</span>
            </div>
            <div class="resource-value">
              <div class="progress-bar">
                <div
                  class="progress-fill"
                  style="width: {currentProcess.cpu_usage}%"
                  class:high={currentProcess.cpu_usage > 50}
                  class:critical={currentProcess.cpu_usage > 80}
                ></div>
              </div>
              <span>{currentProcess.cpu_usage.toFixed(1)}%</span>
            </div>
          </div>

          <!-- Memory Usage -->
          <div class="resource-card">
            <div class="resource-header">
              <Fa icon={faMemory} />
              <span>Memory Usage</span>
            </div>
            <div class="resource-stats">
              <div>Physical: {formatBytes(currentProcess.memory_usage)}</div>
              <div>Virtual: {formatBytes(currentProcess.virtual_memory)}</div>
            </div>
          </div>

          <!-- Disk I/O -->
          <div class="resource-card">
            <div class="resource-header">
              <Fa icon={faHardDrive} />
              <span>Disk I/O</span>
            </div>
            <div class="resource-stats">
              <div>Read: {formatBytes(currentProcess.disk_usage[0])}</div>
              <div>Written: {formatBytes(currentProcess.disk_usage[1])}</div>
            </div>
          </div>

          <!-- Time Info -->
          <div class="resource-card">
            <div class="resource-header">
              <Fa icon={faClock} />
              <span>Time Information</span>
            </div>
            <div class="resource-stats">
              <div>Started: {formatDate(currentProcess.start_time)}</div>
              <div>Running: {formatUptime(currentProcess.run_time)}</div>
            </div>
          </div>
        </div>
      </section>

      <!-- Command & Environment Section -->
      <section class="detail-section">
        <h3>Process Details</h3>
        <div class="detail-grid">
          <div class="detail-row full-width">
            <span class="detail-label">Command:</span>
            <span class="detail-value command">{currentProcess.command}</span>
          </div>
          <div class="detail-row full-width">
            <span class="detail-label">Root:</span>
            <span class="detail-value path">{currentProcess.root}</span>
          </div>
          {#if currentProcess.environ.length > 0}
            <div class="detail-row full-width">
              <span class="detail-label">Environment:</span>
              <div class="detail-value env-vars">
                {#each currentProcess.environ as env}
                  <div class="env-var">{env}</div>
                {/each}
              </div>
            </div>
          {/if}
        </div>
      </section>
    </div>
  {/if}
</Modal>

<style>
  .process-details {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .detail-section {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .detail-section h3 {
    font-size: 14px;
    font-weight: 600;
    color: var(--text);
    margin: 0;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--surface0);
  }

  .detail-grid {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .detail-row {
    display: flex;
    gap: 12px;
    padding: 8px;
    border-radius: 4px;
    background: var(--surface0);
  }

  .detail-row.full-width {
    flex-direction: column;
    gap: 8px;
  }

  .detail-label {
    display: flex;
    color: var(--subtext0);
    font-weight: 500;
  }

  .detail-value {
    flex: 1;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
      monospace;
    word-break: break-all;
  }

  .detail-value.command,
  .detail-value.path {
    font-size: 12px;
    color: var(--subtext1);
    padding: 8px;
    background: var(--mantle);
    border-radius: 4px;
  }

  .resource-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 16px;
  }

  .resource-card {
    background: var(--surface0);
    padding: 16px;
    border-radius: 8px;
  }

  .resource-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 12px;
    color: var(--subtext0);
    font-weight: 500;
  }

  .resource-header :global(svg) {
    width: 14px;
    height: 14px;
    color: var(--blue);
  }

  .resource-value {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .progress-bar {
    flex: 1;
    height: 8px;
    background: var(--surface1);
    border-radius: 4px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: var(--blue);
    transition: width 0.3s ease;
  }

  .progress-fill.high {
    background: var(--yellow);
  }

  .progress-fill.critical {
    background: var(--red);
  }

  .resource-stats {
    display: flex;
    flex-direction: column;
    gap: 8px;
    font-size: 13px;
    color: var(--text);
  }

  .env-vars {
    display: flex;
    flex-direction: column;
    gap: 4px;
    max-height: 200px;
    overflow-y: auto;
    padding: 8px;
    background: var(--mantle);
    border-radius: 4px;
  }

  .env-var {
    font-size: 12px;
    color: var(--subtext1);
    padding: 4px 8px;
    border-radius: 4px;
  }

  .env-var:hover {
    background: var(--surface1);
  }
</style>
