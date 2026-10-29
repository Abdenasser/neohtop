<script lang="ts">
  import { Progress } from "$lib/components/ui/progress/index.js";
  export let label: string;
  export let value: number;
  export let labelWidth = "2.5rem";
  export let valueWidth = "2.5rem";
  $: color =
    value > 90 ? "red" : value > 75 ? "peach" : value > 50 ? "yellow" : "blue";
</script>

<div class="flex min-w-0 items-center gap-2 text-xs">
  <span class="shrink-0 text-muted-foreground" style:width={labelWidth}
    >{label}</span
  ><Progress
    class="h-1.5 min-w-3"
    value={Math.max(0, Math.min(100, Number.isFinite(value) ? value : 0))}
    style={`--primary:var(--${color})`}
    aria-label={`${label} usage`}
  /><span class="shrink-0 text-right tabular-nums" style:width={valueWidth}
    >{Number.isFinite(value) ? Math.round(value) : 0}%</span
  >
</div>
