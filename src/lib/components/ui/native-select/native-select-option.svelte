<script lang="ts">
  import { cn, type WithElementRef } from "$lib/shadcn-utils.js";
  import type { HTMLOptionAttributes } from "svelte/elements";

  let {
    ref = $bindable(null),
    class: className,
    children,
    ...restProps
  }: WithElementRef<HTMLOptionAttributes> = $props();
</script>

<option
  bind:this={ref}
  data-slot="native-select-option"
  class={cn("bg-[Canvas] text-[CanvasText]", className)}
  {...restProps}
>
  {@render children?.()}
</option>
