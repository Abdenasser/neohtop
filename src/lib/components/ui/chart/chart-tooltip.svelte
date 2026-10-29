<script lang="ts">
  import { getChartContext, Tooltip as TooltipPrimitive } from "layerchart";
  import {
    cn,
    type WithElementRef,
    type WithoutChildren,
  } from "$lib/shadcn-utils.js";
  import {
    getPayloadConfigFromPayload,
    useChart,
    type TooltipPayload,
  } from "./chart-utils.js";
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  function defaultFormatter(value: any, _payload: TooltipPayload[]) {
    return `${value}`;
  }

  let {
    ref = $bindable(null),
    class: className,
    hideLabel = false,
    indicator = "dot",
    hideIndicator = false,
    labelKey,
    label,
    labelFormatter = defaultFormatter,
    labelClassName,
    formatter,
    nameKey,
    color,
    ...restProps
  }: WithoutChildren<WithElementRef<HTMLAttributes<HTMLDivElement>>> & {
    hideLabel?: boolean;
    label?: string;
    indicator?: "line" | "dot" | "dashed";
    nameKey?: string;
    labelKey?: string;
    hideIndicator?: boolean;
    labelClassName?: string;
    labelFormatter?: // eslint-disable-next-line @typescript-eslint/no-explicit-any
    | ((value: any, payload: TooltipPayload[]) => string | number | Snippet)
      | null;
    formatter?: Snippet<
      [
        {
          value: unknown;
          name: string;
          item: TooltipPayload;
          index: number;
          payload: TooltipPayload[];
        },
      ]
    >;
  } = $props();

  const chart = useChart();
  const chartCtx = getChartContext();

  // Filter to series with defined values (important for item-based charts like Pie/Arc
  // where only the hovered item has a value)
  const visibleSeries = $derived(
    chartCtx.tooltip.series.filter(
      (s: TooltipPayload) => s.value !== undefined,
    ),
  );

  const formattedLabel = $derived.by(() => {
    if (hideLabel || !visibleSeries?.length) return null;

    const [item] = visibleSeries;
    const tooltipData = chartCtx.tooltip.data;

    // Get the x-axis label value from the raw tooltip data (e.g. a Date or month string)
    const dataLabel = tooltipData != null ? chartCtx.x(tooltipData) : undefined;

    const key = labelKey ?? item?.label ?? item?.key ?? "value";
    const itemConfig = getPayloadConfigFromPayload(
      chart.config,
      item,
      key,
      tooltipData as Record<string, unknown> | null,
    );

    let value: unknown;
    if (!labelKey && typeof label === "string") {
      value = chart.config[label as keyof typeof chart.config]?.label ?? label;
    } else if (labelKey) {
      value = itemConfig?.label ?? dataLabel;
    } else {
      value = dataLabel;
    }

    if (value === undefined) return null;
    if (!labelFormatter) return value;
    return labelFormatter(value, visibleSeries);
  });

  const nestLabel = $derived(visibleSeries.length === 1 && indicator !== "dot");
</script>

{#snippet TooltipLabel()}
  {#if formattedLabel}
    <div class={cn("font-medium", labelClassName)}>
      {#if typeof formattedLabel === "function"}
        {@render formattedLabel()}
      {:else}
        {formattedLabel}
      {/if}
    </div>
  {/if}
{/snippet}

<TooltipPrimitive.Root variant="none">
  <div
    bind:this={ref}
    class={cn(
      "grid min-w-32 items-start gap-1.5 rounded-lg border border-border/50 bg-background px-2.5 py-1.5 text-xs shadow-xl",
      className,
    )}
    {...restProps}
  >
    {#if !nestLabel}
      {@render TooltipLabel()}
    {/if}
    <div class="grid gap-1.5">
      {#each visibleSeries as item, i (item.key + i)}
        {@const key = `${nameKey || item.key || item.label || "value"}`}
        {@const itemConfig = getPayloadConfigFromPayload(
          chart.config,
          item,
          key,
          chartCtx.tooltip.data,
        )}
        {@const indicatorColor = color || item.config?.color || item.color}
        <div
          class={cn(
            "flex w-full flex-wrap items-stretch gap-2 [&>svg]:h-2.5 [&>svg]:w-2.5 [&>svg]:text-muted-foreground",
            indicator === "dot" && "items-center",
          )}
        >
          {#if formatter && item.value !== undefined && item.label}
            {@render formatter({
              value: item.value,
              name: item.label,
              item,
              index: i,
              payload: visibleSeries,
            })}
          {:else}
            {#if itemConfig?.icon}
              <itemConfig.icon />
            {:else if !hideIndicator}
              <div
                style="--color-bg: {indicatorColor}; --color-border: {indicatorColor};"
                class={cn(
                  "shrink-0 rounded-[2px] border-(--color-border) bg-(--color-bg)",
                  {
                    "h-2.5 w-2.5": indicator === "dot",
                    "h-full w-1": indicator === "line",
                    "w-0 border-[1.5px] border-dashed bg-transparent":
                      indicator === "dashed",
                    "my-0.5": nestLabel && indicator === "dashed",
                  },
                )}
              ></div>
            {/if}
            <div
              class={cn(
                "flex flex-1 shrink-0 justify-between leading-none",
                nestLabel ? "items-end" : "items-center",
              )}
            >
              <div class="grid gap-1.5">
                {#if nestLabel}
                  {@render TooltipLabel()}
                {/if}
                <span class="text-muted-foreground">
                  {itemConfig?.label || item.label}
                </span>
              </div>
              {#if item.value !== undefined}
                <span
                  class="font-mono font-medium text-foreground tabular-nums"
                >
                  {item.value.toLocaleString()}
                </span>
              {/if}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </div>
</TooltipPrimitive.Root>
