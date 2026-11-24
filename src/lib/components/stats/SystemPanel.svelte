<script lang="ts">
  import { t, uiLocale } from "$lib/i18n";
  import ResourceDetails from "./ResourceDetails.svelte";
  import type { SystemStats } from "$lib/types";
  export let stats: SystemStats;
  export let capturedAt = 0;
  export let isFrozen = false;
  export let unavailable = false;
  import * as Card from "$lib/components/ui/card/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { onMount } from "svelte";
  import { platform } from "@tauri-apps/plugin-os";
  import { Server } from "@lucide/svelte";
  import { formatUptime } from "$lib/utils";
  export let uptime: number;
  export let loadAvg: [number, number, number];
  let loadSupported: boolean | null = null;
  onMount(() => {
    try {
      loadSupported = platform() !== "windows";
    } catch {
      loadSupported = null;
    }
  });
</script>

<Card.Root size="sm" class="h-60 gap-3">
  <Card.Header class="grid-cols-[1fr_auto] items-center gap-2">
    <Card.Title
      class="flex items-center gap-2 text-sm font-medium text-muted-foreground"
      ><Badge
        variant="secondary"
        class="size-5 rounded-md border-0 p-0 bg-muted text-muted-foreground [&>svg]:size-3!"
        aria-hidden="true"><Server /></Badge
      >{$t("systemPanel.system")}</Card.Title
    ><ResourceDetails
      kind="system"
      {stats}
      {capturedAt}
      {isFrozen}
      {unavailable}
    /></Card.Header
  >
  <Card.Content class="min-h-0 flex-1 gap-2">
    <div>
      <div
        class="whitespace-nowrap text-xl leading-8 font-semibold tracking-tight tabular-nums"
        title={formatUptime(uptime, $uiLocale)}
      >
        {Number.isFinite(uptime) ? formatUptime(uptime, $uiLocale) : "—"}
      </div>
      <p class="text-xs leading-4 text-muted-foreground">
        {$t("systemPanel.uptime")}
      </p>
    </div>
    <div
      class="mt-auto border-t border-border/60 pt-3"
      title={$t("systemPanel.systemLoadAveragedOver15And")}
    >
      <p class="mb-2 text-[11px] text-muted-foreground">
        {$t("systemPanel.loadAverage")}
      </p>
      {#if loadSupported}<dl class="grid grid-cols-3 gap-2">
          {#each loadAvg as value, index}<div>
              <dt class="text-[10px] text-muted-foreground">
                {[1, 5, 15][index]}
                {$t("systemPanel.min")}
              </dt>
              <dd class="mt-0.5 text-sm font-medium tabular-nums">
                {Number.isFinite(value) ? value.toFixed(2) : "—"}
              </dd>
            </div>{/each}
        </dl>{:else}<p class="text-xs text-muted-foreground">
          {loadSupported === false
            ? $t("systemPanel.notProvidedOnWindows")
            : $t("systemPanel.unavailableOnThisSystem")}
        </p>{/if}
    </div></Card.Content
  ></Card.Root
>
