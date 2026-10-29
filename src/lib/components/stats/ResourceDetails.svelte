<script lang="ts">
  import type { Snippet } from "svelte";
  import * as Sheet from "$lib/components/ui/sheet/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { ArrowUpRight, Pause, Play } from "@lucide/svelte";
  import { processStore } from "$lib/stores/processes";
  import type { SystemStats, ResourceSample } from "$lib/types";
  import ResourceDetailsContent from "./ResourceDetailsContent.svelte";
  const panels = {
    cpu: {
      title: "CPU",
      label: "CPU details",
      description: "Overall utilization and activity on each core.",
    },
    memory: {
      title: "Memory",
      label: "Memory details",
      description: "Physical RAM, allocation, and swap.",
    },
    storage: {
      title: "Storage",
      label: "Storage details",
      description: "Mounted volumes and their available capacity.",
    },
    system: {
      title: "System",
      label: "System details",
      description: "Operating system, uptime, and system load.",
    },
    network: {
      title: "Network",
      label: "Network details",
      description: "Traffic across this machine’s network interfaces.",
    },
    gpu: {
      title: "Graphics",
      label: "GPU details",
      description: "Driver-reported readings from this machine.",
    },
  };
  let {
    kind,
    stats,
    history = [],
    isFrozen = false,
    unavailable = false,
    capturedAt = 0,
    children,
  }: {
    kind: keyof typeof panels;
    stats?: SystemStats;
    history?: ResourceSample[];
    isFrozen?: boolean;
    unavailable?: boolean;
    capturedAt?: number;
    children?: Snippet;
  } = $props();
  const panel = $derived(panels[kind]);
  const headingId = $derived(`resource-details-${kind}`);
  const triggerId = $derived(`resource-trigger-${kind}`);
</script>

<Sheet.Root>
  <Sheet.Trigger id={triggerId}
    >{#snippet child({ props })}<Button
        {...props}
        variant="ghost"
        size="icon-xs"
        aria-label={panel.label}
        title={panel.label}><ArrowUpRight class="size-3.5" /></Button
      >{/snippet}</Sheet.Trigger
  >
  <Sheet.Content
    class="data-[side=right]:sm:max-w-md"
    onCloseAutoFocus={(event) => {
      event.preventDefault();
      document.getElementById(triggerId)?.focus();
    }}
    onOpenAutoFocus={(event) => {
      event.preventDefault();
      document.getElementById(headingId)?.focus();
    }}
  >
    <Sheet.Header class="border-b border-border pb-4 pr-12">
      <Sheet.Title id={headingId} tabindex={-1} class="outline-none"
        >{panel.title}</Sheet.Title
      >
      <Sheet.Description>{panel.description}</Sheet.Description>
      <div class="mt-1 flex items-center gap-2 text-xs text-muted-foreground">
        {#if unavailable}<Badge variant="destructive">Updates unavailable</Badge
          >{:else if isFrozen}<Badge variant="secondary">Paused</Badge
          >{:else}<span
            class="size-1.5 rounded-full bg-[var(--status-running)]"
            aria-hidden="true"
          ></span>{/if}
        <span
          >{capturedAt
            ? `Reading from ${new Date(capturedAt).toLocaleTimeString()}`
            : "Waiting for a reading"}</span
        >
      </div>
    </Sheet.Header>
    <div class="min-h-0 flex-1 overflow-y-auto px-4 pb-5 space-y-5">
      {#if children}{@render children()}{:else if stats && kind !== "gpu"}<ResourceDetailsContent
          {kind}
          {stats}
          {history}
          {isFrozen}
          {unavailable}
        />{/if}
    </div>
    <Sheet.Footer
      class="shrink-0 flex-row items-center justify-between border-t border-border px-4 py-3"
    >
      <Button
        variant="outline"
        size="sm"
        aria-label={isFrozen ? "Resume monitoring" : "Pause monitoring"}
        onclick={() => processStore.setIsFrozen(!isFrozen)}
        >{#if isFrozen}<Play />Resume{:else}<Pause />Pause{/if}</Button
      >
      <Sheet.Close
        >{#snippet child({ props })}<Button
            {...props}
            variant="secondary"
            size="sm">Done</Button
          >{/snippet}</Sheet.Close
      >
    </Sheet.Footer>
  </Sheet.Content>
</Sheet.Root>
