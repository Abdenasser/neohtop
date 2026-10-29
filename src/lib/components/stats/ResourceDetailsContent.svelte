<script lang="ts">
  import { onMount } from "svelte";
  import { Checkbox } from "$lib/components/ui/checkbox/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import * as NativeSelect from "$lib/components/ui/native-select/index.js";
  import { Progress } from "$lib/components/ui/progress/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { ArrowDown, ArrowUp, HardDrive, Network } from "@lucide/svelte";
  import { formatBytes, formatUptime } from "$lib/utils";
  import type { SystemStats, ResourceSample } from "$lib/types";
  import ResourceTrend from "./ResourceTrend.svelte";
  let {
    kind,
    stats,
    history,
    isFrozen,
    unavailable,
  }: {
    kind: "cpu" | "memory" | "storage" | "system" | "network";
    stats: SystemStats;
    history: ResourceSample[];
    isFrozen: boolean;
    unavailable: boolean;
  } = $props();
  let volumeMount = $state("");
  let interfaceSearch = $state("");
  let includeUnused = $state(false);
  let interfaceOrder = $state<string[]>([]);
  // Prioritize observed traffic on open, then keep rows stable while reading.
  onMount(() => {
    interfaceOrder = [...stats.details.interfaces]
      .sort(
        (a, b) =>
          (b.received ?? 0) +
            (b.sent ?? 0) -
            ((a.received ?? 0) + (a.sent ?? 0)) ||
          b.received_total + b.sent_total - (a.received_total + a.sent_total) ||
          a.name.localeCompare(b.name),
      )
      .map((i) => i.name);
  });
  const details = $derived(stats.details);
  const identity = $derived(details.identity);
  const cpu = $derived(
    stats.cpu_usage.length
      ? stats.cpu_usage.reduce((a, b) => a + b, 0) / stats.cpu_usage.length
      : null,
  );
  const memory = $derived(
    stats.memory_total > 0
      ? Math.min(100, (stats.memory_used / stats.memory_total) * 100)
      : null,
  );
  const volume = $derived(
    details.volumes.find((v) => v.mount === volumeMount) ??
      details.volumes.find((v) => v.mount === "/") ??
      details.volumes[0],
  );
  const volumeUsage = $derived(
    volume && volume.total > 0
      ? Math.max(
          0,
          Math.min(
            100,
            ((volume.total - volume.available) / volume.total) * 100,
          ),
        )
      : null,
  );
  const interfaceRanks = $derived(
    new Map(interfaceOrder.map((name, index) => [name, index])),
  );
  const interfaces = $derived(
    details.interfaces
      .filter(
        (i) =>
          (includeUnused ||
            interfaceSearch.trim() ||
            i.received_total > 0 ||
            i.sent_total > 0 ||
            i.received === null ||
            i.sent === null) &&
          i.name.toLowerCase().includes(interfaceSearch.trim().toLowerCase()),
      )
      .sort(
        (a, b) =>
          (interfaceRanks.get(a.name) ?? Infinity) -
            (interfaceRanks.get(b.name) ?? Infinity) ||
          a.name.localeCompare(b.name),
      ),
  );
  const percent = (value: number | null) =>
    value === null || !Number.isFinite(value)
      ? "Unavailable"
      : `${value.toFixed(1)}%`;
  const rate = (value: number | null) =>
    value === null ? "Unavailable" : `${formatBytes(value)}/s`;
</script>

{#snippet facts(items: [string, string][])}
  <Card.Root size="sm"
    ><Card.Content class="gap-0 divide-y divide-border">
      {#each items as [label, value]}<div
          class="flex justify-between gap-4 py-3 first:pt-0 last:pb-0"
        >
          <span class="text-sm text-muted-foreground">{label}</span><span
            class="min-w-0 max-w-[65%] break-words text-right text-sm font-medium tabular-nums"
            data-selectable>{value}</span
          >
        </div>{/each}
    </Card.Content></Card.Root
  >
{/snippet}
{#snippet note(text: string)}<p
    class="rounded-lg border border-border bg-muted/40 p-4 text-sm leading-relaxed text-muted-foreground"
  >
    {text}
  </p>{/snippet}

{#if kind === "cpu"}
  <div>
    <h3 class="text-lg font-semibold">{identity.cpu_model ?? "Processor"}</h3>
    <p class="mt-1 text-sm text-muted-foreground">
      {stats.cpu_usage.length} logical processors · {identity.architecture}
    </p>
  </div>
  {@render facts([
    ["Average utilization", percent(cpu)],
    [
      "Physical cores",
      identity.physical_cores === null
        ? "Unavailable"
        : String(identity.physical_cores),
    ],
    ["Logical processors", String(stats.cpu_usage.length)],
  ])}
  <section class="space-y-2" aria-label="CPU history">
    <h3 class="text-sm font-medium">Utilization over time</h3>
    <ResourceTrend
      {history}
      metric="cpu"
      label="CPU utilization"
      expanded
      {isFrozen}
      {unavailable}
    />
  </section>
  <Card.Root size="sm"
    ><Card.Header
      ><Card.Title class="text-sm">Per-core activity</Card.Title
      ><Card.Description>Each core is scaled from 0 to 100%.</Card.Description
      ></Card.Header
    ><Card.Content class="grid grid-cols-2 gap-x-5 gap-y-4">
      {#each stats.cpu_usage as value, index}<div class="min-w-0 space-y-1.5">
          <div class="flex justify-between gap-2 text-xs">
            <span class="text-muted-foreground">Core {index}</span><span
              class="tabular-nums">{percent(value)}</span
            >
          </div>
          <Progress
            value={Number.isFinite(value)
              ? Math.max(0, Math.min(100, value))
              : null}
            aria-label={`Core ${index} utilization`}
            aria-valuetext={percent(value)}
            style="--primary:var(--chart-1)"
          />
        </div>{:else}<p class="col-span-2 text-sm text-muted-foreground">
          Core readings are unavailable.
        </p>{/each}
    </Card.Content></Card.Root
  >
  {@render note(
    "The overview averages utilization across logical processors. An individual process can exceed 100% CPU when it uses several cores.",
  )}
{:else if kind === "memory"}
  <div>
    <p class="text-3xl font-semibold tracking-tight tabular-nums">
      {formatBytes(stats.memory_used)}
    </p>
    <p class="mt-1 text-sm text-muted-foreground">
      RAM in use · {percent(memory)} of {formatBytes(stats.memory_total)}
    </p>
  </div>
  <ResourceTrend
    {history}
    metric="memory"
    label="Memory utilization"
    expanded
    {isFrozen}
    {unavailable}
  />
  {@render facts([
    ["Physical memory", formatBytes(stats.memory_total)],
    ["In use", formatBytes(stats.memory_used)],
    ["Not in use", formatBytes(stats.memory_free)],
    ["Unallocated", formatBytes(details.memory_unallocated)],
  ])}
  <Card.Root size="sm"
    ><Card.Header
      ><Card.Title class="text-sm">Swap</Card.Title><Card.Description
        >Memory the operating system stores on disk.</Card.Description
      ></Card.Header
    ><Card.Content>
      {#if details.swap_total > 0}<div
          class="flex justify-between gap-3 text-sm"
        >
          <span class="tabular-nums">{formatBytes(details.swap_used)} used</span
          ><span class="text-muted-foreground"
            >of {formatBytes(details.swap_total)}</span
          >
        </div>
        <Progress
          value={Math.min(100, (details.swap_used / details.swap_total) * 100)}
          aria-label="Swap used"
          style="--primary:var(--chart-3)"
        />{:else}<p class="text-sm text-muted-foreground">
          No swap space is currently allocated.
        </p>{/if}
    </Card.Content></Card.Root
  >
  {@render note(
    "Not in use is total RAM minus reported use, matching the overview. Unallocated is the operating system’s free-page count. These figures can overlap. Swap usage alone does not measure memory pressure.",
  )}
{:else if kind === "storage"}
  {#if volume}
    <div class="space-y-2">
      <label for="resource-volume" class="text-sm font-medium">Volume</label
      ><NativeSelect.Root
        id="resource-volume"
        value={volume.mount}
        onchange={(event) => (volumeMount = event.currentTarget.value)}
        class="w-full min-w-0"
        title={volume.mount}
        >{#each details.volumes as item}<option value={item.mount}
            >{item.mount} · {item.name || "Volume"}</option
          >{/each}</NativeSelect.Root
      >
    </div>
    <div>
      <h3 class="break-words text-lg font-semibold">
        {volume.name || "Volume"}
      </h3>
      <p class="mt-1 break-all text-xs text-muted-foreground" data-selectable>
        {volume.mount}
      </p>
    </div>
    <Card.Root size="sm"
      ><Card.Content
        ><div>
          <p class="text-3xl font-semibold tracking-tight tabular-nums">
            {volume.total > 0 ? formatBytes(volume.available) : "Unavailable"}
          </p>
          <p class="mt-1 text-sm text-muted-foreground">
            Available on this volume
          </p>
        </div>
        <Progress
          value={volumeUsage}
          aria-label="Selected volume capacity used"
          aria-valuetext={percent(volumeUsage)}
          style="--primary:var(--chart-4)"
        />
        <div class="flex justify-between text-xs text-muted-foreground">
          <span>{percent(volumeUsage)} used</span><span
            >{volumeUsage === null
              ? ""
              : `${(100 - volumeUsage).toFixed(1)}% available`}</span
          >
        </div></Card.Content
      ></Card.Root
    >
    {@render facts([
      [
        "Capacity",
        volume.total > 0 ? formatBytes(volume.total) : "Unavailable",
      ],
      [
        "Used",
        volume.total > 0
          ? formatBytes(Math.max(0, volume.total - volume.available))
          : "Unavailable",
      ],
      ["File system", volume.file_system || "Unavailable"],
      ["Access", volume.read_only ? "Read only" : "Read and write"],
    ])}
    {@render note(
      "These are filesystem capacity readings, not a scan of files or folders. Volumes can share a physical storage pool, especially with APFS, so adding their capacities can count the same space more than once.",
    )}
  {:else}<div class="space-y-3 py-6">
      <HardDrive class="size-8 text-muted-foreground" />
      <h3 class="font-semibold">No volumes reported</h3>
      <p class="text-sm text-muted-foreground">
        Mounted-volume information is unavailable in the current system reading.
      </p>
    </div>{/if}
{:else if kind === "system"}
  <div>
    <h3 class="break-words text-lg font-semibold">
      {identity.os_version ?? "Operating system unavailable"}
    </h3>
    <p class="mt-1 text-sm text-muted-foreground">{identity.architecture}</p>
  </div>
  {@render facts([
    ["Uptime", formatUptime(stats.uptime)],
    ["Kernel", identity.kernel_version ?? "Unavailable"],
    ["Architecture", identity.architecture],
    ["Processor", identity.cpu_model ?? "Unavailable"],
    ["Logical processors", String(stats.cpu_usage.length)],
  ])}
  <Card.Root size="sm"
    ><Card.Header
      ><Card.Title class="text-sm">Load average</Card.Title><Card.Description
        >Queued and running work over time.</Card.Description
      ></Card.Header
    ><Card.Content>
      {#if identity.platform !== "windows"}<dl class="grid grid-cols-3 gap-3">
          {#each stats.load_avg as value, index}<div>
              <dt class="text-xs text-muted-foreground">
                {[1, 5, 15][index]} minute{index ? "s" : ""}
              </dt>
              <dd class="mt-1 text-xl font-semibold tabular-nums">
                {Number.isFinite(value) ? value.toFixed(2) : "Unavailable"}
              </dd>
            </div>{/each}
        </dl>{:else}<p class="text-sm text-muted-foreground">
          Load averages are not provided on Windows.
        </p>{/if}
    </Card.Content></Card.Root
  >
  {@render note(
    "Load is not a CPU percentage. Compare its trend on this machine alongside CPU utilization. Uptime is the time since the operating system started, not how long NeoHtop has been open.",
  )}
{:else if kind === "network"}
  <Card.Root size="sm"
    ><Card.Content class="grid grid-cols-2 gap-4"
      >{#each [{ label: "Received", value: stats.network_rx_bytes, icon: ArrowDown, color: "var(--chart-1)" }, { label: "Sent", value: stats.network_tx_bytes, icon: ArrowUp, color: "var(--chart-4)" }] as metric}<div
          class="min-w-0"
        >
          <p class="flex items-center gap-1.5 text-xs text-muted-foreground">
            <metric.icon
              class="size-3"
              style={`color:${metric.color}`}
            />{metric.label}
          </p>
          <p class="mt-1 break-words text-xl font-semibold tabular-nums">
            {rate(metric.value)}
          </p>
        </div>{/each}</Card.Content
    ></Card.Root
  >
  <ResourceTrend
    {history}
    metric="network"
    label="Network receive and send"
    expanded
    {isFrozen}
    {unavailable}
  />
  <section class="space-y-3" aria-label="Network interfaces">
    <div class="flex items-center justify-between gap-2">
      <h3 class="text-sm font-medium">Interfaces</h3>
      <span class="text-xs text-muted-foreground"
        >{interfaces.length} of {details.interfaces.length}</span
      >
    </div>
    <Input
      type="search"
      placeholder="Search all interfaces…"
      aria-label="Find a network interface"
      bind:value={interfaceSearch}
    />
    <div class="flex items-center gap-2">
      <Checkbox
        id="include-unused-interfaces"
        checked={includeUnused}
        onCheckedChange={(value) => (includeUnused = value === true)}
      /><Label
        for="include-unused-interfaces"
        class="text-xs font-normal text-muted-foreground"
        >Include unused interfaces</Label
      >
    </div>
    {#each interfaces as item (item.name)}<Card.Root size="sm" class="gap-3"
        ><Card.Header
          ><Card.Title class="break-all font-mono text-sm"
            >{item.name}</Card.Title
          ></Card.Header
        ><Card.Content
          ><div class="grid grid-cols-2 gap-4">
            <div>
              <p class="text-xs text-muted-foreground">↓ Received /s</p>
              <p class="mt-1 text-sm font-medium tabular-nums">
                {rate(item.received)}
              </p>
            </div>
            <div>
              <p class="text-xs text-muted-foreground">↑ Sent /s</p>
              <p class="mt-1 text-sm font-medium tabular-nums">
                {rate(item.sent)}
              </p>
            </div>
          </div>
          <dl
            class="grid grid-cols-2 gap-4 border-t border-border pt-2 text-xs"
          >
            <div>
              <dt class="text-muted-foreground">Received total</dt>
              <dd class="mt-1 tabular-nums">
                {formatBytes(item.received_total)}
              </dd>
            </div>
            <div>
              <dt class="text-muted-foreground">Sent total</dt>
              <dd class="mt-1 tabular-nums">{formatBytes(item.sent_total)}</dd>
            </div>
          </dl></Card.Content
        ></Card.Root
      >{:else}<div
        class="rounded-lg border border-dashed border-border p-5 text-sm text-muted-foreground"
      >
        <Network class="mb-2 size-5" />
        <p>
          {interfaceSearch.trim()
            ? "No interfaces match this search."
            : details.interfaces.length
              ? "No interfaces with recorded traffic."
              : "No network interfaces were reported."}
        </p>
        {#if interfaceSearch}<Button
            variant="link"
            size="sm"
            class="mt-2 px-0"
            onclick={() => (interfaceSearch = "")}>Clear search</Button
          >{/if}
        {#if !interfaceSearch.trim() && details.interfaces.length && !includeUnused}<Button
            variant="link"
            size="sm"
            class="mt-2 px-0"
            onclick={() => (includeUnused = true)}>Show all interfaces</Button
          >{/if}
      </div>{/each}
  </section>
  {@render note(
    "Rates use the shared sampling interval. Totals come from the operating system and can reset when an interface restarts. Virtual interfaces and loopback can count the same traffic more than once; this is not an internet speed test.",
  )}
{/if}
