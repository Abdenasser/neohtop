<script lang="ts">
  import { t } from "$lib/i18n";
  import { trackUsage } from "$lib/utils/telemetry";
  import type { Snippet } from "svelte";
  import * as Sheet from "$lib/components/ui/sheet/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { ArrowUpRight, Pause, Play } from "@lucide/svelte";
  import { processStore } from "$lib/stores/processes";
  import type { SystemStats, ResourceSample } from "$lib/types";
  import ResourceDetailsContent from "./ResourceDetailsContent.svelte";
  const panels = $derived({
    cpu: {
      title: $t("common.cpu"),
      label: $t("details.cpu"),
      description: $t("details.cpuDescription"),
    },
    memory: {
      title: $t("common.memory"),
      label: $t("details.memory"),
      description: $t("details.memoryDescription"),
    },
    storage: {
      title: $t("common.storage"),
      label: $t("details.storage"),
      description: $t("details.storageDescription"),
    },
    system: {
      title: $t("common.system"),
      label: $t("details.system"),
      description: $t("details.systemDescription"),
    },
    network: {
      title: $t("common.network"),
      label: $t("details.network"),
      description: $t("details.networkDescription"),
    },
    gpu: {
      title: $t("details.graphics"),
      label: $t("details.gpu"),
      description: $t("details.gpuDescription"),
    },
  });
  let {
    kind,
    stats,
    history = [],
    isFrozen = false,
    unavailable = false,
    capturedAt = 0,
    children,
  }: {
    kind: "cpu" | "memory" | "storage" | "system" | "network" | "gpu";
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

<Sheet.Root
  onOpenChange={(open) => {
    if (open) trackUsage("resource_view", kind);
  }}
>
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
        {#if unavailable}<Badge variant="destructive"
            >{$t("resourceDetails.updatesUnavailable")}</Badge
          >{:else if isFrozen}<Badge variant="secondary"
            >{$t("resourceDetails.paused")}</Badge
          >{:else}<span
            class="size-1.5 rounded-full bg-[var(--status-running)]"
            aria-hidden="true"
          ></span>{/if}
        <span
          >{capturedAt
            ? $t("resourceDetails.readingFromValue", {
                value: new Date(capturedAt).toLocaleTimeString(),
              })
            : $t("resourceDetails.waitingForAReading")}</span
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
        aria-label={isFrozen
          ? $t("resourceDetails.resumeMonitoring")
          : $t("resourceDetails.pauseMonitoring")}
        onclick={() => processStore.setIsFrozen(!isFrozen)}
        >{#if isFrozen}<Play />{$t("resourceDetails.resume")}{:else}<Pause
          />{$t("resourceDetails.pause")}{/if}</Button
      >
      <Sheet.Close
        >{#snippet child({ props })}<Button
            {...props}
            variant="secondary"
            size="sm">{$t("resourceDetails.done")}</Button
          >{/snippet}</Sheet.Close
      >
    </Sheet.Footer>
  </Sheet.Content>
</Sheet.Root>
