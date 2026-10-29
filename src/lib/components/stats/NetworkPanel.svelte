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
  import { Network } from "@lucide/svelte";
  import StatItem from "./StatItem.svelte";
  import { formatBytes } from "$lib/utils";
  export let networkRxBytes: number | null;
  export let networkTxBytes: number | null;
</script>

<Card.Root size="sm" class="h-60 gap-3">
  <Card.Header class="grid-cols-[1fr_auto] items-center gap-2">
    <Card.Title
      class="flex items-center gap-2 text-sm font-medium text-muted-foreground"
      ><Badge
        variant="secondary"
        class="size-5 rounded-md border-0 p-0 bg-primary/10 text-primary [&>svg]:size-3!"
        style="--primary:var(--chart-1)"
        aria-hidden="true"><Network /></Badge
      >Network</Card.Title
    ><ResourceDetails
      kind="network"
      {stats}
      {capturedAt}
      {isFrozen}
      {unavailable}
      {history}
    /></Card.Header
  >
  <Card.Content class="min-h-0 flex-1 gap-2">
    <div>
      <div
        class="whitespace-nowrap text-2xl leading-8 font-semibold tracking-tight tabular-nums"
      >
        {networkRxBytes !== null && Number.isFinite(networkRxBytes)
          ? formatBytes(networkRxBytes)
          : "—"}<span class="ml-0.5 text-base font-normal text-muted-foreground"
          >/s</span
        >
      </div>
      <p
        class="flex items-center gap-1.5 text-xs leading-4 text-muted-foreground"
      >
        <span
          class="w-2.5 border-t-2 border-[var(--chart-1)]"
          aria-hidden="true"
        ></span>↓ Received
      </p>
    </div>
    <div class="flex items-center gap-1.5">
      <span
        class="w-2.5 shrink-0 border-t-2 border-dashed border-[var(--chart-4)]"
        aria-hidden="true"
      ></span>
      <div class="min-w-0 flex-1">
        <StatItem
          label="↑ Sent"
          value={networkTxBytes !== null && Number.isFinite(networkTxBytes)
            ? `${formatBytes(networkTxBytes)}/s`
            : "Unavailable"}
        />
      </div>
    </div>
    <ResourceTrend
      {history}
      metric="network"
      label="Network receive and send"
      {isFrozen}
      {unavailable}
    />
  </Card.Content></Card.Root
>
