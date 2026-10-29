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
  data-slot="card-footer"
  class={cn(
    "rounded-b-xl px-(--card-spacing) [.border-t]:pt-(--card-spacing) flex items-center",
    className,
  )}
  {...restProps}
>
  {@render children?.()}
</div>
