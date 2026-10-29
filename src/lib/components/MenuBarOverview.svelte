<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import * as Card from "$lib/components/ui/card/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { Progress } from "$lib/components/ui/progress/index.js";
  import {
    Cpu,
    MemoryStick,
    Gpu,
    Pause,
    Play,
    ArrowUpRight,
    Power,
    HardDrive,
    ArrowDown,
    ArrowUp,
  } from "@lucide/svelte";
  import ProcessIcon from "./process/ProcessIcon.svelte";
  import { formatBytes } from "$lib/utils";
  import { themeStore } from "$lib/stores/theme";
  import type { OverviewSnapshot } from "$lib/types";
  let data: OverviewSnapshot | null = null;
  let error = "";
  let visible = false;
  let destroyed = false;
  let clock = Date.now();
  let gpuId = "";
  let inFlight: Promise<void> | null = null;
  let poll: ReturnType<typeof setTimeout>;
  let ticker: ReturnType<typeof setInterval>;
  const listeners: UnlistenFn[] = [];
  $: cpu =
    data?.history.at(-1)?.cpu ??
    (data?.stats.cpu_usage.length
      ? data.stats.cpu_usage.reduce((a, b) => a + b, 0) /
        data.stats.cpu_usage.length
      : null);
  $: memory =
    data && data.stats.memory_total > 0
      ? (data.stats.memory_used / data.stats.memory_total) * 100
      : null;
  $: gpu = data?.gpus.find((g) => g.id === gpuId) ?? data?.gpus[0];
  $: age = data
    ? Math.max(0, Math.floor((clock - data.captured_at) / 1000))
    : 0;
  $: stale =
    !!data && clock - data.captured_at > Math.max(10000, data.interval * 3);
  const pct = (value: number | null | undefined) =>
    value == null ? "—" : `${value.toFixed(1)}%`;
  const rate = (value: number | null | undefined) =>
    value == null ? "—" : `${formatBytes(value)}/s`;
  async function fetchOverview() {
    try {
      const next = await invoke<OverviewSnapshot>("get_overview");
      if (!destroyed) {
        if (!next.gpus.some((g) => g.id === gpuId))
          gpuId =
            next.gpus.find((g) => g.utilization !== null)?.id ??
            next.gpus[0]?.id ??
            "";
        data = next;
        error = next.error ?? "";
        clock = Date.now();
      }
    } catch (e) {
      if (!destroyed) error = String(e);
    }
  }
  function read() {
    if (inFlight) return inFlight;
    inFlight = fetchOverview().finally(() => {
      inFlight = null;
    });
    return inFlight;
  }
  function schedule() {
    clearTimeout(poll);
    if (!visible || destroyed) return;
    poll = setTimeout(
      async () => {
        await read();
        schedule();
      },
      Math.min(data?.interval ?? 3000, 3000),
    );
  }
  async function visibility(value: boolean) {
    visible = value;
    clearTimeout(poll);
    clearInterval(ticker);
    if (visible && !destroyed) {
      ticker = setInterval(() => {
        clock = Date.now();
      }, 1000);
      await read();
      schedule();
    }
  }
  async function pause() {
    try {
      await invoke("set_monitor_paused", { paused: !data?.paused });
      await read();
    } catch (e) {
      error = String(e);
    }
  }
  async function open(process?: { pid: number; start_time: number }) {
    try {
      await invoke("open_main_window", { process: process ?? null });
    } catch (e) {
      error = String(e);
    }
  }
  onMount(() => {
    document.documentElement.dataset.companion = "true";
    themeStore.init();
    void (async () => {
      for (const [event, callback] of [
        [
          "companion-visibility",
          ({ payload }: { payload: boolean }) => {
            void visibility(payload);
          },
        ],
        [
          "monitor-control",
          () => {
            if (visible) void read();
          },
        ],
      ] as const) {
        const stop = await listen(event, callback);
        if (destroyed) stop();
        else listeners.push(stop);
      }
      // Listen before reading visibility, so a popup opened during startup is not lost.
      const isVisible = await getCurrentWindow().isVisible();
      if (!destroyed) void visibility(isVisible);
    })();
  });
  onDestroy(() => {
    destroyed = true;
    clearTimeout(poll);
    clearInterval(ticker);
    listeners.forEach((stop) => stop());
  });
</script>

<svelte:window
  on:keydown={(event) => {
    if (event.key === "Escape") void invoke("hide_companion");
  }}
/>

<div class="h-screen p-2">
  <Card.Root
    size="sm"
    class="h-full gap-0 overflow-hidden rounded-2xl shadow-lg ring-1 ring-border"
  >
    <Card.Header
      class="grid-cols-[1fr_auto] items-center gap-2 border-b border-border pb-4"
    >
      <div class="flex items-center gap-2">
        <img
          src="/128x128.png"
          alt=""
          width="28"
          height="28"
          class="size-7 shrink-0"
        />
        <div>
          <Card.Title>NeoHtop</Card.Title>
          <p class="mt-0.5 text-xs text-muted-foreground">
            {data?.paused
              ? "Updates paused"
              : stale
                ? `Last reading ${age}s ago`
                : data
                  ? "Live system overview"
                  : "Reading this machine…"}
          </p>
        </div>
      </div>
      <Button
        variant="ghost"
        size="icon-sm"
        onclick={() => open()}
        aria-label="Open NeoHtop"><ArrowUpRight /></Button
      >
    </Card.Header>
    <Card.Content class="min-h-0 flex-1 overflow-y-auto pt-3 pb-3 gap-3">
      {#if error}<div
          role="alert"
          class="rounded-lg border border-destructive/40 bg-destructive/5 p-2.5 text-xs text-destructive"
        >
          {error}<Button variant="link" size="sm" onclick={read}>Retry</Button>
        </div>{/if}
      <div class="grid grid-cols-3 divide-x divide-border">
        {#each [{ name: "CPU", value: cpu, icon: Cpu, color: "var(--chart-1)", detail: `${data?.stats.cpu_usage.length ?? 0} cores` }, { name: "Memory", value: memory, icon: MemoryStick, color: "var(--chart-3)", detail: data ? `${formatBytes(data.stats.memory_used)} used` : "Reading…" }, { name: "GPU", value: gpu?.utilization, icon: Gpu, color: "var(--chart-2)", detail: gpu?.name ?? "Unavailable" }] as metric}
          <div class="min-w-0 px-3 first:pl-0 last:pr-0 space-y-2">
            <div
              class="flex items-center gap-1.5 text-xs text-muted-foreground"
            >
              <Badge
                variant="secondary"
                class="size-5 rounded-md p-0 border-0 bg-primary/10 text-primary [&>svg]:size-3!"
                style={`--primary:${metric.color}`}
                aria-hidden="true"><metric.icon /></Badge
              >{metric.name}
            </div>
            <p class="text-xl font-semibold tabular-nums">
              {pct(metric.value)}
            </p>
            <Progress
              value={metric.value ?? 0}
              class="h-1.5"
              style={`--primary:${metric.color}`}
              aria-label={`${metric.name} utilization`}
              aria-valuetext={metric.value == null
                ? "Unavailable"
                : pct(metric.value)}
            />
            <p
              class="truncate text-[10px] text-muted-foreground"
              title={metric.detail}
            >
              {metric.detail}
            </p>
          </div>
        {/each}
      </div>
      <div
        class="grid grid-cols-2 gap-3 rounded-lg border border-border bg-muted/30 p-3 text-xs"
      >
        <div class="space-y-1.5">
          <p class="text-muted-foreground">Network</p>
          <p class="flex items-center gap-1.5 font-medium tabular-nums">
            <ArrowDown class="size-3 text-[var(--chart-1)]" />{rate(
              data?.stats.network_rx_bytes,
            )}
          </p>
          <p
            class="flex items-center gap-1.5 tabular-nums text-muted-foreground"
          >
            <ArrowUp class="size-3" />{rate(data?.stats.network_tx_bytes)}
          </p>
        </div>
        <div class="space-y-1.5">
          <p class="flex items-center gap-1.5 text-muted-foreground">
            <HardDrive class="size-3" />Storage free
          </p>
          <p class="font-medium tabular-nums">
            {data ? formatBytes(data.stats.disk_free_bytes) : "—"}
          </p>
          <p class="text-muted-foreground">
            {data
              ? `of ${formatBytes(data.stats.disk_total_bytes)}`
              : "Reading…"}
          </p>
        </div>
      </div>
      <section aria-label="Top CPU processes">
        <div class="mb-2 flex justify-between text-xs text-muted-foreground">
          <h2 class="font-medium text-foreground">Top CPU processes</h2>
          <span>CPU · RAM</span>
        </div>
        <div
          class="overflow-hidden rounded-lg border border-border divide-y divide-border"
        >
          {#each data?.processes ?? [] as process (process.pid)}<div
              class="flex"
            >
              <Button
                variant="ghost"
                class="h-auto w-full justify-start gap-2.5 rounded-none border-0 px-3 py-2 text-left font-normal focus-visible:ring-inset"
                onclick={() =>
                  open({ pid: process.pid, start_time: process.start_time })}
                title={`${process.name} · PID ${process.pid}`}
                aria-label={`Inspect ${process.name}, PID ${process.pid}`}
                ><ProcessIcon
                  pid={process.pid}
                  startTime={process.start_time}
                  size={20}
                /><span class="min-w-0 flex-1 truncate text-xs font-medium"
                  >{process.name}</span
                ><span class="shrink-0 text-right text-xs tabular-nums"
                  >{pct(process.cpu_usage)}<span
                    class="ml-2 inline-block w-16 text-muted-foreground"
                    >{formatBytes(process.memory_usage)}</span
                  ></span
                ></Button
              >
            </div>{:else}<p class="p-4 text-xs text-muted-foreground">
              {data ? "No processes reported" : "Collecting processes…"}
            </p>{/each}
        </div>
        <p class="mt-2 text-[10px] leading-4 text-muted-foreground">
          Select to inspect. Process CPU can exceed 100%.
        </p>
      </section>
    </Card.Content>
    <Card.Footer class="justify-between border-t border-border pt-3 gap-2"
      ><Button variant="outline" size="sm" onclick={pause} disabled={!data}
        >{#if data?.paused}<Play />Resume{:else}<Pause />Pause{/if}</Button
      ><Button variant="ghost" size="sm" onclick={() => open()}
        >Open NeoHtop<ArrowUpRight /></Button
      ><Button
        variant="ghost"
        size="icon-sm"
        aria-label="Quit NeoHtop"
        onclick={() => invoke("quit_app")}><Power /></Button
      ></Card.Footer
    >
  </Card.Root>
</div>
