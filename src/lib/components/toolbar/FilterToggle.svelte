<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Checkbox } from "$lib/components/ui/checkbox/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import * as NativeSelect from "$lib/components/ui/native-select/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { SlidersHorizontal, RotateCcw } from "@lucide/svelte";
  export let filters: {
    cpu: { operator: string; value: number; enabled: boolean };
    gpu: { operator: string; value: number; enabled: boolean };
    ram: { operator: string; value: number; enabled: boolean };
    runtime: { operator: string; value: number; enabled: boolean };
    status: { values: string[]; enabled: boolean };
  };
  $: count = Object.values(filters).filter((f) => f.enabled).length;
  const numeric = [
    { id: "cpu" as const, label: "CPU usage", unit: "%" },
    { id: "gpu" as const, label: "GPU engine activity", unit: "%" },
    { id: "ram" as const, label: "Memory", unit: "MB" },
    { id: "runtime" as const, label: "Run time", unit: "min" },
  ];
  const statuses = [
    "Running",
    "Sleeping",
    "Stopped",
    "Zombie",
    "Idle",
    "Unknown",
  ];
  function setStatus(value: string, enabled: boolean) {
    filters = {
      ...filters,
      status: {
        values: enabled
          ? [...filters.status.values, value]
          : filters.status.values.filter((v) => v !== value),
        enabled: false,
      },
    };
    filters.status.enabled = filters.status.values.length > 0;
  }
  function reset() {
    filters = {
      ...filters,
      cpu: { ...filters.cpu, enabled: false },
      gpu: { ...filters.gpu, enabled: false },
      ram: { ...filters.ram, enabled: false },
      runtime: { ...filters.runtime, enabled: false },
      status: { values: [], enabled: false },
    };
  }
</script>

<Popover.Root
  ><Popover.Trigger
    >{#snippet child({ props })}<Button
        {...props}
        variant="outline"
        size="sm"
        class="h-9"
        aria-label={count ? `Filters, ${count} active` : "Filters"}
        ><SlidersHorizontal />Filters{#if count}<Badge
            class="ml-1 size-4 justify-center rounded-sm p-0 text-[10px]"
            >{count}</Badge
          >{/if}</Button
      >{/snippet}</Popover.Trigger
  ><Popover.Content
    align="start"
    class="w-96 max-w-[calc(100vw-2rem)] space-y-5"
    ><div>
      <h2 class="text-sm font-medium">Filter processes</h2>
      <p class="mt-1 text-xs text-muted-foreground">
        Combine conditions with name, command or PID search.
      </p>
    </div>
    <div class="space-y-3">
      {#each numeric as rule}<div
          class="grid grid-cols-[1fr_64px_80px_28px] items-center gap-2"
        >
          <Label class="gap-2 text-xs"
            ><Checkbox
              bind:checked={filters[rule.id].enabled}
            />{rule.label}</Label
          ><NativeSelect.Root
            bind:value={filters[rule.id].operator}
            aria-label={`${rule.label} comparison`}
            disabled={!filters[rule.id].enabled}
            size="sm"
            ><option value=">">&gt;</option><option value="<">&lt;</option
            ><option value="=">=</option><option value=">=">≥</option><option
              value="<=">≤</option
            ></NativeSelect.Root
          ><Input
            type="number"
            min="0"
            step="any"
            bind:value={filters[rule.id].value}
            disabled={!filters[rule.id].enabled}
            aria-label={`${rule.label} threshold`}
            class="h-8 text-xs"
          /><span class="text-xs text-muted-foreground">{rule.unit}</span>
        </div>{/each}
    </div>
    <div class="space-y-3 border-t border-border pt-4">
      <h3 class="text-xs font-medium">Status</h3>
      <div class="grid grid-cols-2 gap-3">
        {#each statuses as status}<Label class="gap-2 text-xs font-normal"
            ><Checkbox
              checked={filters.status.values.includes(status)}
              onCheckedChange={(checked) => setStatus(status, checked)}
            />{status}</Label
          >{/each}
      </div>
    </div>
    <div class="flex items-center justify-between border-t border-border pt-3">
      <span class="text-xs text-muted-foreground"
        >{count ? `${count} active conditions` : "No filters applied"}</span
      ><Button variant="ghost" size="sm" disabled={!count} onclick={reset}
        ><RotateCcw />Reset filters</Button
      >
    </div></Popover.Content
  ></Popover.Root
>
