<script lang="ts">
  import * as Chart from "$lib/components/ui/chart/index.js";
  import { AreaChart, Area, LinearGradient, Points } from "layerchart";
  import type { ResourceSample } from "$lib/types";
  import { formatBytes } from "$lib/utils";
  import {
    RESOURCE_WINDOW_MS,
    networkCeiling,
    nearestResourceSample,
  } from "$lib/utils/resource-history.js";
  type Metric = "cpu" | "memory" | "network" | "gpu";
  type Key = "cpu" | "memory" | "received" | "sent" | "gpu";
  let {
    history: rawHistory = [],
    gpuId = "",
    unsupported = false,
    expanded = false,
    metric,
    label,
    isFrozen = false,
    unavailable = false,
  }: {
    history: ResourceSample[];
    gpuId?: string;
    unsupported?: boolean;
    expanded?: boolean;
    metric: Metric;
    label: string;
    isFrozen?: boolean;
    unavailable?: boolean;
  } = $props();
  const history = $derived(
    rawHistory.map((p) => ({ ...p, gpu: p.gpus?.[gpuId] ?? null })),
  );
  let selectedStamp = $state<number | null>(null);
  let keyboard = $state(false);
  const series = $derived(
    metric === "network"
      ? [
          {
            key: "received" as Key,
            label: "Received",
            color: "var(--chart-1)",
          },
          { key: "sent" as Key, label: "Sent", color: "var(--chart-4)" },
        ]
      : [
          {
            key: metric as Key,
            label:
              metric === "cpu" ? "CPU" : metric === "gpu" ? "GPU" : "RAM used",
            color:
              metric === "cpu"
                ? "var(--chart-1)"
                : metric === "gpu"
                  ? "var(--chart-2)"
                  : "var(--chart-3)",
          },
        ],
  );
  const config = $derived(
    Object.fromEntries(
      series.map((s) => [s.key, { label: s.label, color: s.color }]),
    ) satisfies Chart.ChartConfig,
  );
  const end = $derived(history.at(-1)?.timestamp ?? 0);
  const ceiling = $derived(
    metric === "network" ? networkCeiling(history) : 100,
  );
  const selected = $derived(
    selectedStamp === null
      ? history.at(-1)
      : nearestResourceSample(history, selectedStamp),
  );
  const selectedIndex = $derived(
    selected ? history.findIndex((p) => p.timestamp === selected.timestamp) : 0,
  );
  const validCount = $derived(
    history.filter((p) => series.some((s) => p[s.key] != null)).length,
  );
  const time = (stamp: number) =>
    new Date(stamp).toLocaleTimeString(undefined, {
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    });
  const value = (n: number | null) =>
    n === null
      ? "Unavailable"
      : metric === "network"
        ? `${formatBytes(n)}/s`
        : `${n.toFixed(1)}%`;
  const describe = (p: ResourceSample | undefined) =>
    p
      ? `${time(p.timestamp)} · ${series.map((s) => `${s.label} ${value(p[s.key] ?? null)}`).join(" · ")}`
      : "Collecting history";
  const caption = $derived(
    selectedStamp !== null
      ? metric === "network" && selected
        ? `↓ ${value(selected.received)} · ↑ ${value(selected.sent)}`
        : describe(selected)
      : isFrozen
        ? "Paused"
        : unavailable
          ? "Updates unavailable"
          : unsupported
            ? "Driver counter unavailable"
            : validCount < 2
              ? "Collecting history…"
              : metric === "network"
                ? `2m · 0–${formatBytes(ceiling)}/s`
                : "2m · 0–100%",
  );
  function point(event: PointerEvent) {
    if (!history.length) return;
    keyboard = false;
    const rect =
      event.currentTarget instanceof HTMLElement
        ? event.currentTarget.getBoundingClientRect()
        : null;
    if (!rect) return;
    const ratio = Math.max(
      0,
      Math.min(1, (event.clientX - rect.left) / rect.width),
    );
    selectedStamp =
      nearestResourceSample(
        history,
        end - RESOURCE_WINDOW_MS + ratio * RESOURCE_WINDOW_MS,
      )?.timestamp ?? null;
  }
  function key(event: KeyboardEvent) {
    if (
      !["ArrowLeft", "ArrowRight", "Home", "End", "Escape"].includes(event.key)
    )
      return;
    event.preventDefault();
    if (event.key === "Escape") {
      selectedStamp = null;
      keyboard = false;
      return;
    }
    keyboard = true;
    const next =
      event.key === "Home"
        ? 0
        : event.key === "End"
          ? history.length - 1
          : Math.max(
              0,
              Math.min(
                history.length - 1,
                selectedIndex + (event.key === "ArrowLeft" ? -1 : 1),
              ),
            );
    selectedStamp = history[next]?.timestamp ?? null;
  }
</script>

<div class="mt-auto min-w-0 space-y-1">
  <Chart.Container
    {config}
    class={`${expanded ? "h-28" : "h-12"} w-full rounded-sm aspect-auto focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring`}
    role="slider"
    tabindex={history.length ? 0 : -1}
    aria-label={`${label} history, last two minutes. Use Left and Right arrows to inspect samples.`}
    aria-valuemin={0}
    aria-valuemax={Math.max(0, history.length - 1)}
    aria-valuenow={selectedIndex}
    aria-valuetext={describe(selected)}
    title={describe(selected)}
    aria-disabled={!history.length}
    onkeydown={key}
    onpointermove={point}
    onpointerleave={() => {
      if (!keyboard) selectedStamp = null;
    }}
    onfocus={() => {
      keyboard = true;
      selectedStamp = history.at(-1)?.timestamp ?? null;
    }}
    onblur={() => {
      keyboard = false;
      selectedStamp = null;
    }}
  >
    {#if history.length}
      <AreaChart
        data={history}
        x="timestamp"
        y={series[0].key}
        {series}
        seriesLayout="overlap"
        xDomain={[end - RESOURCE_WINDOW_MS, end]}
        yDomain={[0, ceiling]}
        yNice={false}
        axis={false}
        grid={false}
        rule={false}
        legend={false}
        highlight={false}
        tooltipContext={false}
        motion={{ type: "none" }}
        padding={{ left: 2, right: 3, top: 3, bottom: 2 }}
      >
        {#snippet marks()}
          {#each series as s}
            <LinearGradient vertical>
              {#snippet stopsContent()}<stop
                  offset="0%"
                  stop-color={s.color}
                  stop-opacity=".32"
                /><stop
                  offset="100%"
                  stop-color={s.color}
                  stop-opacity=".01"
                />{/snippet}
              {#snippet children({ gradient })}<Area
                  seriesKey={s.key}
                  fill={gradient}
                  fillOpacity={1}
                  defined={(d: ResourceSample) => d[s.key] != null}
                  line={{
                    stroke: s.color,
                    "stroke-width": 1.5,
                    "stroke-dasharray": s.key === "sent" ? "3 2" : undefined,
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                  }}
                  motion={{ type: "none" }}
                />{/snippet}
            </LinearGradient>
            {#if selected && selected[s.key] != null}<Points
                data={[selected]}
                x="timestamp"
                y={s.key}
                r={2}
                fill={s.color}
                stroke="var(--card)"
                strokeWidth={1}
              />{/if}
          {/each}
        {/snippet}
      </AreaChart>
    {:else}<div class="flex h-full w-full items-end">
        <div class="w-full border-b border-border/60"></div>
      </div>{/if}
  </Chart.Container>
  <div
    class="min-h-3 truncate text-[10px] leading-3 text-muted-foreground tabular-nums"
    title={`${label}: ${caption}. History stays in memory. Pauses and missed reads stay disconnected.`}
  >
    {caption}
  </div>
</div>
