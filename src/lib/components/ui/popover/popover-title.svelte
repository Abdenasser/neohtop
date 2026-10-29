<script lang="ts">
  import { cn, type WithElementRef } from "$lib/shadcn-utils.js";
  import type { HTMLAttributes } from "svelte/elements";

  let {
    ref = $bindable(null),
    class: className,
    children,
    ...restProps
  }: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div
  bind:this={ref}
  data-slot="popover-title"
  class={cn("font-medium", className)}
  {...restProps}
>
  {@render children?.()}
</div>
