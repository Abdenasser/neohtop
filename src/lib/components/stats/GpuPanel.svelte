<script lang="ts">
  import { faDisplay } from "@fortawesome/free-solid-svg-icons";
  import { PanelHeader, ProgressBar, StatItem } from "$lib/components";
  import { formatBytes, formatPercentage } from "$lib/utils";
  import type { GpuInfo } from "$lib/types";

  export let gpu: GpuInfo;
  export let title = "GPU";

  // A sensor that answered once can still come up empty on a later tick, and a
  // row that vanishes shifts every row under it. So each reading is latched:
  // it appears the first time its sensor answers anything, and from then on
  // holds the last value it gave rather than dropping out of the panel.
  const latest: Record<string, string | number> = {};

  function latch<T extends string | number>(key: string, value: T | null) {
    if (value != null) latest[key] = value;
    return (latest[key] as T | undefined) ?? null;
  }

  // Which readings a card publishes depends on its driver, so each one is
  // formatted up front and the rows with nothing behind them are left out
  $: utilization = latch("utilization", gpu.utilization);

  $: vramPercentage = latch(
    "vramPercentage",
    gpu.memory_total != null && gpu.memory_used != null && gpu.memory_total > 0
      ? (gpu.memory_used / gpu.memory_total) * 100
      : null,
  );

  $: vram = latch(
    "vram",
    gpu.memory_total != null && gpu.memory_used != null
      ? `${formatBytes(gpu.memory_used)} / ${formatBytes(gpu.memory_total)}`
      : null,
  );

  $: temperature = latch(
    "temperature",
    gpu.temperature != null ? `${gpu.temperature.toFixed(0)}°C` : null,
  );

  $: power = latch(
    "power",
    gpu.power_watts != null ? `${gpu.power_watts.toFixed(1)} W` : null,
  );

  // An idle card power-gates its shader clock, so 0 MHz is a reading like any
  // other rather than a missing one
  $: clock = latch(
    "clock",
    gpu.core_clock_mhz != null ? `${gpu.core_clock_mhz} MHz` : null,
  );
</script>

<div class="stat-panel">
  <PanelHeader
    icon={faDisplay}
    {title}
    usageValue={utilization != null ? formatPercentage(utilization) : null}
  />
  <div class="stats-content">
    <div class="gpu-model" title="{gpu.vendor} {gpu.name}">{gpu.name}</div>

    {#if utilization != null}
      <div class="stat-item with-progress">
        <ProgressBar
          label="Core"
          value={utilization}
          labelWidth="2.8rem"
          valueWidth="2.5rem"
        />
      </div>
    {/if}

    {#if vramPercentage != null}
      <div class="stat-item with-progress">
        <ProgressBar
          label="VRAM"
          value={vramPercentage}
          labelWidth="2.8rem"
          valueWidth="2.5rem"
        />
      </div>
    {/if}

    {#if vram}
      <StatItem label="Used" value={vram} />
    {/if}
    {#if temperature}
      <StatItem label="Temperature" value={temperature} />
    {/if}
    {#if power}
      <StatItem label="Power" value={power} />
    {/if}
    {#if clock}
      <StatItem label="Clock" value={clock} />
    {/if}
  </div>
</div>

<style>
  .stat-panel {
    flex: 1.6;
    min-width: 160px;
    background-color: var(--mantle);
    border-radius: 6px;
    padding: 0.75rem;
    display: flex;
    flex-direction: column;
  }

  .stats-content {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .gpu-model {
    font-size: 0.65rem;
    line-height: 1.2;
    color: var(--subtext0);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .stat-item.with-progress {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
  }
</style>
