<script lang="ts">
  export let label: string;
  export let value: string;
</script>

<div class="flex min-w-0 items-start justify-between gap-2 text-xs">
  <span class="shrink-0 text-muted-foreground">{label}</span><span
    class="break-words text-right tabular-nums">{value}</span
  >
</div>
