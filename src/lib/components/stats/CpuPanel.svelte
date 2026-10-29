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
  import { Cpu } from "@lucide/svelte";
  import { Progress } from "$lib/components/ui/progress/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
  export let cpuUsage: number[];
  let activeCore = 0;
  $: average =
    cpuUsage.length && cpuUsage.every(Number.isFinite)
      ? cpuUsage.reduce((a, b) => a + b, 0) / cpuUsage.length
      : null;
  $: activeCore = Math.max(0, Math.min(activeCore, cpuUsage.length - 1));
  const reading = (value: number) =>
    Number.isFinite(value)
      ? `${Math.max(0, Math.min(100, value)).toFixed(1)}%`
      : "Unavailable";
  function moveCore(event: KeyboardEvent, index: number) {
    if (
      ![
        "ArrowLeft",
        "ArrowRight",
        "ArrowUp",
        "ArrowDown",
        "Home",
        "End",
      ].includes(event.key)
    )
      return;
    const group =
      event.currentTarget instanceof HTMLElement
        ? event.currentTarget.closest("[data-core-meters]")
        : null;
    if (!group) return;
    const meters = Array.from(
      group.querySelectorAll<HTMLButtonElement>("[data-core-index]"),
    );
    const next =
      event.key === "Home"
        ? 0
        : event.key === "End"
          ? meters.length - 1
          : Math.max(
              0,
              Math.min(
                meters.length - 1,
                index + (["ArrowLeft", "ArrowUp"].includes(event.key) ? -1 : 1),
              ),
            );
    event.preventDefault();
    activeCore = next;
    meters[next]?.focus();
  }
</script>

<Card.Root size="sm" class="h-60 gap-3">
  <Card.Header class="grid-cols-[1fr_auto] items-center gap-2">
    <Card.Title
      class="flex items-center gap-2 text-sm font-medium text-muted-foreground"
      ><Badge
        variant="secondary"
        class="size-5 rounded-md border-0 p-0 bg-primary/10 text-primary [&>svg]:size-3!"
        style="--primary:var(--chart-1)"
        aria-hidden="true"><Cpu /></Badge
      >CPU</Card.Title
    >
    <div class="flex items-center gap-1">
      <Badge variant="secondary" class="text-[10px] font-normal tabular-nums"
        >{cpuUsage.length} cores</Badge
      ><ResourceDetails
        kind="cpu"
        {stats}
        {capturedAt}
        {isFrozen}
        {unavailable}
        {history}
      />
    </div></Card.Header
  >
  <Card.Content class="min-h-0 flex-1 gap-2">
    <div>
      <div class="text-3xl leading-8 font-semibold tracking-tight tabular-nums">
        {average === null ? "—" : average.toFixed(1)}<span
          class="ml-0.5 text-lg font-normal text-muted-foreground"
          >{average === null ? "" : "%"}</span
        >
      </div>
      <p class="text-xs leading-4 text-muted-foreground">Average utilization</p>
    </div>
    {#if cpuUsage.length}<div
        class="grid max-h-12 shrink-0 gap-x-0.5 overflow-y-auto"
        style={`grid-template-columns:repeat(${cpuUsage.length <= 6 ? cpuUsage.length : cpuUsage.length <= 12 ? Math.ceil(cpuUsage.length / 2) : 8},minmax(0,1fr))`}
        role="group"
        aria-label="CPU cores. Use arrow keys to inspect each core."
        data-core-meters
      >
        {#each cpuUsage as value, index}<Tooltip.Root
            ><Tooltip.Trigger
              >{#snippet child({ props })}<Button
                  {...props}
                  variant="ghost"
                  class="h-6 min-w-0 flex-col gap-0.5 rounded-sm px-0.5 py-0.5"
                  tabindex={activeCore === index ? 0 : -1}
                  data-core-index={index}
                  aria-label={`Core ${index}, ${reading(value)}`}
                  onfocus={(event) => {
                    if (typeof props.onfocus === "function")
                      props.onfocus(event);
                    activeCore = index;
                  }}
                  onkeydown={(event) => {
                    if (typeof props.onkeydown === "function")
                      props.onkeydown(event);
                    moveCore(event, index);
                  }}
                >
                  <Progress
                    class="h-1 w-full"
                    value={Number.isFinite(value)
                      ? Math.max(0, Math.min(100, value))
                      : 0}
                    aria-hidden="true"
                  /><span
                    class="text-[9px] leading-3 font-normal text-muted-foreground"
                    aria-hidden="true">{index}</span
                  ></Button
                >{/snippet}</Tooltip.Trigger
            ><Tooltip.Content side="bottom" sideOffset={5}
              ><span class="tabular-nums">Core {index} · {reading(value)}</span
              ></Tooltip.Content
            ></Tooltip.Root
          >{/each}
      </div>{:else}<p class="text-xs text-muted-foreground">
        Core readings unavailable.
      </p>{/if}
    <ResourceTrend
      {history}
      metric="cpu"
      label="CPU utilization"
      {isFrozen}
      {unavailable}
    />
  </Card.Content></Card.Root
>
