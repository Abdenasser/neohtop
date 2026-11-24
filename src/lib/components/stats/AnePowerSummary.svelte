<script lang="ts">
  import * as Chart from "$lib/components/ui/chart/index.js";
  import { AreaChart, Area, Points } from "layerchart";
  import type { AnePowerSnapshot } from "$lib/types";

  let {
    power,
    end,
    unavailable,
  }: { power: AnePowerSnapshot; end: number; unavailable: boolean } = $props();
  const color = "var(--chart-4)";
  const label = "Neural Engine power";
  const config = { watts: { label, color } };
  const formatter = new Intl.NumberFormat(undefined, {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
  const latestPoint = $derived(
    power.points.findLast((point) => point.watts !== null),
  );
  const format = (value: number | null, unit: "W" | "J") =>
    value === null ? "Unavailable" : `${formatter.format(value)} ${unit}`;
</script>

<section
  aria-labelledby="ane-power-title"
  class="space-y-2 rounded-lg border border-border bg-muted/30 p-3 text-xs"
>
  <div class="flex items-baseline justify-between gap-3">
    <h2 id="ane-power-title" class="font-medium">{label}</h2>
    <p class="font-semibold tabular-nums">
      {format(unavailable ? null : power.watts, "W")}
    </p>
  </div>
  <div
    class="flex flex-wrap justify-between gap-x-3 gap-y-1 text-[10px] text-muted-foreground"
  >
    <span>Last {power.window_ms / 1000}s</span>
    {#if power.average_watts !== null && power.peak_watts !== null}
      <span class="tabular-nums">
        Avg {format(power.average_watts, "W")} · Peak {format(
          power.peak_watts,
          "W",
        )}
      </span>
    {/if}
  </div>
  {#if latestPoint}
    <Chart.Container
      {config}
      class="h-10 w-full aspect-auto"
      aria-hidden="true"
    >
      <AreaChart
        data={power.points}
        x="timestamp"
        y="watts"
        series={[{ key: "watts", label, color }]}
        xDomain={[end - power.window_ms, end]}
        yDomain={[0, Math.max(0.5, power.peak_watts ?? 0)]}
        yNice={false}
        axis={false}
        grid={false}
        rule={false}
        legend={false}
        highlight={false}
        tooltipContext={false}
        motion={{ type: "none" }}
        padding={{ left: 2, right: 3, top: 3, bottom: 3 }}
      >
        {#snippet marks()}
          <Area
            seriesKey="watts"
            defined={(point: AnePowerSnapshot["points"][number]) =>
              point.watts !== null}
            fill={color}
            fillOpacity={0.14}
            line={{
              stroke: color,
              "stroke-width": 1.5,
              "stroke-linecap": "round",
            }}
            motion={{ type: "none" }}
          />
          <Points
            data={[latestPoint]}
            x="timestamp"
            y="watts"
            r={2}
            fill={color}
          />
        {/snippet}
      </AreaChart>
    </Chart.Container>
  {/if}
  {#if power.joules !== null}
    <p class="text-[10px] text-muted-foreground tabular-nums">
      Energy {format(power.joules, "J")} · {Math.round(
        Math.min(power.window_ms, power.covered_ms) / 1000,
      )}s sampled
    </p>
  {/if}
</section>
