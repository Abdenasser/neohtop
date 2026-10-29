<script lang="ts">
  import { cn, type WithElementRef } from "$lib/shadcn-utils.js";
  import type { HTMLOptgroupAttributes } from "svelte/elements";

  let {
    ref = $bindable(null),
    class: className,
    children,
    ...restProps
  }: WithElementRef<HTMLOptgroupAttributes> = $props();
</script>

<optgroup
  bind:this={ref}
  data-slot="native-select-optgroup"
  class={cn("bg-[Canvas] text-[CanvasText]", className)}
  {...restProps}
>
  {@render children?.()}
</optgroup>
