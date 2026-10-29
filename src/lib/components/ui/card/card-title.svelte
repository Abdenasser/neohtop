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
  data-slot="card-title"
  class={cn(
    "cn-font-heading text-base leading-normal font-medium group-data-[size=sm]/card:text-sm",
    className,
  )}
  {...restProps}
>
  {@render children?.()}
</div>
