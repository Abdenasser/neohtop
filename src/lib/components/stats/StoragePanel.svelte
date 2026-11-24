<script lang="ts">
  import { t } from "$lib/i18n";
  import ResourceDetails from "./ResourceDetails.svelte";
  import type { SystemStats } from "$lib/types";
  export let stats: SystemStats;
  export let capturedAt = 0;
  export let isFrozen = false;
  export let unavailable = false;
  import * as Card from "$lib/components/ui/card/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { HardDrive } from "@lucide/svelte";
  import { Progress } from "$lib/components/ui/progress/index.js";
  import StatItem from "./StatItem.svelte";
  import { formatBytes } from "$lib/utils";
  import { formatUsagePercentage } from "$lib/utils/metrics.js";
  export let diskTotalBytes: number;
  export let diskUsedBytes: number;
  export let diskFreeBytes: number;
  $: ready = Number.isFinite(diskTotalBytes) && diskTotalBytes > 0;
  $: percentage = ready
    ? Math.max(0, Math.min(100, (diskUsedBytes / diskTotalBytes) * 100))
    : null;
</script>

<Card.Root size="sm" class="h-60 gap-3">
  <Card.Header class="grid-cols-[1fr_auto] items-center gap-2">
    <Card.Title
      class="flex items-center gap-2 text-sm font-medium text-muted-foreground"
      ><Badge
        variant="secondary"
        class="size-5 rounded-md border-0 p-0 bg-primary/10 text-primary [&>svg]:size-3!"
        style="--primary:var(--chart-4)"
        aria-hidden="true"><HardDrive /></Badge
      >{$t("storagePanel.storage")}</Card.Title
    ><ResourceDetails
      kind="storage"
      {stats}
      {capturedAt}
      {isFrozen}
      {unavailable}
    /></Card.Header
  >
  <Card.Content class="min-h-0 flex-1 gap-2">
    <div>
      <div class="text-2xl leading-8 font-semibold tracking-tight tabular-nums">
        {ready ? formatBytes(diskFreeBytes) : "—"}
      </div>
      <p class="text-xs leading-4 text-muted-foreground">
        {$t("storagePanel.freeOf")}
        {ready
          ? formatBytes(diskTotalBytes)
          : $t("storagePanel.unknownCapacity")}
      </p>
    </div>
    <StatItem
      label={$t("storagePanel.used")}
      value={ready ? formatBytes(diskUsedBytes) : $t("common.unavailable")}
    />
    <div class="mt-auto space-y-2 pb-0.5">
      <Progress
        value={percentage}
        class="h-2"
        style="--primary:var(--chart-4)"
        aria-label={$t("storagePanel.storageCapacityUsed")}
        aria-valuetext={percentage === null
          ? $t("common.unavailable")
          : `${percentage.toFixed(1)}% used; ${formatBytes(diskFreeBytes)} free`}
      />
      <div
        class="flex justify-between gap-2 text-[10px] text-muted-foreground tabular-nums"
      >
        <span class="flex items-center gap-1.5"
          ><span
            class="size-1.5 rounded-full bg-[var(--chart-4)]"
            aria-hidden="true"
          ></span>{percentage === null
            ? $t("storagePanel.unavailable")
            : $t("storagePanel.valueUsed", {
                value: formatUsagePercentage(diskUsedBytes, diskTotalBytes),
              })}</span
        ><span
          >{percentage === null
            ? ""
            : $t("storagePanel.valueFree", {
                value: (100 - percentage).toFixed(1),
              })}</span
        >
      </div>
    </div></Card.Content
  ></Card.Root
>
