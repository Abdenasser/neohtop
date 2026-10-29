<script lang="ts">
  import ResourceDetails from "./ResourceDetails.svelte";
  import type { SystemStats } from "$lib/types";
  export let stats: SystemStats;
  export let capturedAt = 0;
  import * as Card from "$lib/components/ui/card/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import type { ResourceSample } from "$lib/types";
  import ResourceTrend from "./ResourceTrend.svelte";
  export let history: ResourceSample[] = [];
  export let isFrozen = false;
  export let unavailable = false;
  import { MemoryStick } from "@lucide/svelte";
  import StatItem from "./StatItem.svelte";
  import { formatMemorySize } from "$lib/utils";
  export let memoryTotal: number;
  export let memoryUsed: number;
  export let memoryFree: number;
  $: ready =
    Number.isFinite(memoryTotal) &&
    memoryTotal > 0 &&
    Number.isFinite(memoryUsed);
  $: percentage = ready
    ? Math.max(0, Math.min(100, (memoryUsed / memoryTotal) * 100))
    : null;
</script>

<Card.Root size="sm" class="h-60 gap-3">
  <Card.Header class="grid-cols-[1fr_auto] items-center gap-2">
    <Card.Title
      class="flex items-center gap-2 text-sm font-medium text-muted-foreground"
      ><Badge
        variant="secondary"
        class="size-5 rounded-md border-0 p-0 bg-primary/10 text-primary [&>svg]:size-3!"
        style="--primary:var(--chart-3)"
        aria-hidden="true"><MemoryStick /></Badge
      >Memory</Card.Title
    ><ResourceDetails
      kind="memory"
      {stats}
      {capturedAt}
      {isFrozen}
      {unavailable}
      {history}
    /></Card.Header
  >
  <Card.Content class="min-h-0 flex-1 gap-2">
    <div>
      <div class="text-2xl leading-8 font-semibold tracking-tight tabular-nums">
        {ready ? formatMemorySize(memoryUsed) : "—"}
      </div>
      <p class="text-xs leading-4 text-muted-foreground">
        used of {ready ? formatMemorySize(memoryTotal) : "unknown total"}
      </p>
    </div>
    <StatItem
      label="Not in use"
      value={ready ? formatMemorySize(memoryFree) : "Unavailable"}
    />
    <StatItem
      label="Utilization"
      value={percentage === null ? "Unavailable" : `${percentage.toFixed(1)}%`}
    />
    <ResourceTrend
      {history}
      metric="memory"
      label="Memory utilization"
      {isFrozen}
      {unavailable}
    />
  </Card.Content></Card.Root
>
