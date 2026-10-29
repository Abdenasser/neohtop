<script lang="ts">
  import * as Card from "$lib/components/ui/card/index.js";
  import ResourceDetails from "./ResourceDetails.svelte";
  import type { SystemStats } from "$lib/types";
  export let stats: SystemStats;
  export let capturedAt = 0;
  import * as NativeSelect from "$lib/components/ui/native-select/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { Gpu } from "@lucide/svelte";
  import { formatBytes } from "$lib/utils";
  import type { GpuDevice, ResourceSample } from "$lib/types";
  import ResourceTrend from "./ResourceTrend.svelte";
  export let gpus: GpuDevice[] = [];
  export let history: ResourceSample[] = [];
  export let isFrozen = false;
  export let unavailable = false;
  let selectedId = "";
  function reconcile(devices: GpuDevice[]) {
    if (!devices.some((g) => g.id === selectedId))
      selectedId =
        devices.find((g) => g.utilization !== null)?.id ?? devices[0]?.id ?? "";
  }
  $: reconcile(gpus);
  $: device =
    gpus.find((g) => g.id === selectedId) ??
    gpus.find((g) => g.utilization !== null) ??
    gpus[0];
  const percent = (v: number | null | undefined) =>
    v == null ? "—" : `${v.toFixed(1)}%`;
  const bytes = (v: number | null | undefined) =>
    v == null ? "Unavailable" : formatBytes(v);
</script>

<Card.Root size="sm" class="h-60 gap-3">
  <Card.Header class="grid-cols-[1fr_auto] items-center gap-2">
    <Card.Title
      class="flex items-center gap-2 text-sm font-medium text-muted-foreground"
      ><Badge
        variant="secondary"
        class="size-5 rounded-md border-0 p-0 bg-primary/10 text-primary [&>svg]:size-3!"
        style="--primary:var(--chart-2)"
        aria-hidden="true"><Gpu /></Badge
      >GPU</Card.Title
    >
    <ResourceDetails kind="gpu" {stats} {capturedAt} {isFrozen} {unavailable}
      >{#snippet children()}
        {#if gpus.length > 1}<div class="space-y-2">
            <label for="gpu-device" class="text-sm font-medium"
              >Graphics adapter</label
            ><NativeSelect.Root
              id="gpu-device"
              value={device?.id ?? ""}
              onchange={(event) => (selectedId = event.currentTarget.value)}
              class="w-full"
              >{#each gpus as gpu}<option value={gpu.id}>{gpu.name}</option
                >{/each}</NativeSelect.Root
            >
          </div>{/if}
        {#if device}
          <div>
            <h3 class="text-lg font-semibold">{device.name}</h3>
            <p class="text-sm text-muted-foreground">
              {device.vendor}{device.cores
                ? ` · ${device.cores} graphics cores`
                : ""}
            </p>
          </div>
          <Card.Root size="sm"
            ><Card.Content class="divide-y divide-border">
              {#each [["GPU utilization", device.utilization === null ? "Unavailable" : percent(device.utilization)], [device.memory_kind === "shared" ? "Shared memory in use" : "GPU memory in use", bytes(device.memory_used)], ["GPU memory capacity", device.memory_kind === "shared" ? "Uses system memory" : bytes(device.memory_total)], ["Temperature", device.temperature === null ? "Unavailable" : `${device.temperature.toFixed(0)} °C`], ["Power", device.power_watts == null ? "Unavailable" : `${device.power_watts.toFixed(1)} W`], ["Core clock", device.core_clock_mhz == null ? "Unavailable" : `${device.core_clock_mhz} MHz`]] as [label, value]}<div
                  class="flex justify-between gap-4 py-3 first:pt-0 last:pb-0"
                >
                  <span class="text-muted-foreground">{label}</span><span
                    class="text-right font-medium tabular-nums">{value}</span
                  >
                </div>{/each}
            </Card.Content></Card.Root
          >
          <ResourceTrend
            {history}
            metric="gpu"
            expanded
            label={`${device.name} utilization`}
            gpuId={device.id}
            {isFrozen}
            {unavailable}
            unsupported={device.utilization === null}
          />
          <div
            class="rounded-lg border border-border bg-muted/40 p-4 text-sm leading-relaxed space-y-2"
          >
            <p class="font-medium">About these readings</p>
            <p class="text-muted-foreground">
              {device.source}. {device.memory_kind === "shared"
                ? "Shared GPU memory is part of your system memory, not additional VRAM. "
                : ""}Unavailable counters are not reported as zero. Support
              depends on the GPU and its driver.
            </p>
            <p class="text-xs text-muted-foreground">
              {isFrozen
                ? "Updates paused in both windows."
                : "History covers the last two minutes and stays in memory."}
            </p>
          </div>
        {:else}<div class="space-y-3 py-6">
            <Gpu class="size-8 text-muted-foreground" />
            <h3 class="font-semibold">GPU readings unavailable</h3>
            <p class="text-sm leading-relaxed text-muted-foreground">
              NeoHtop reads macOS graphics drivers, NVIDIA NVML, and Linux AMD
              driver counters. Other adapters may provide inventory only. No
              elevated permissions or extra monitoring agent is required.
            </p>
          </div>{/if}
      {/snippet}</ResourceDetails
    >
  </Card.Header>
  <Card.Content class="min-h-0 flex-1 gap-2">
    <div>
      <div class="text-2xl leading-8 font-semibold tracking-tight tabular-nums">
        {percent(device?.utilization)}
      </div>
      <p
        class="truncate text-xs leading-4 text-muted-foreground"
        title={device?.name}
      >
        {device?.name ?? "No supported GPU detected"}
      </p>
    </div>
    <p
      class="text-xs leading-4 text-muted-foreground"
      title="GPU memory in use. Shared memory is part of system memory."
    >
      {#if device?.memory_used != null}{device.memory_kind === "shared"
          ? "Shared"
          : "Memory"} ·
        <span class="font-medium text-foreground"
          >{formatBytes(device.memory_used)}</span
        >{:else}Live counters vary by driver{/if}
    </p>
    <ResourceTrend
      {history}
      metric="gpu"
      label={`${device?.name ?? "GPU"} utilization`}
      gpuId={device?.id}
      {isFrozen}
      {unavailable}
      unsupported={device?.utilization == null}
    />
  </Card.Content>
</Card.Root>
