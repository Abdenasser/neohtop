<script lang="ts">
  import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui";
  import { CheckIcon } from "@lucide/svelte";
  import { cn, type WithoutChild } from "$lib/shadcn-utils.js";

  let {
    ref = $bindable(null),
    class: className,
    children: childrenProp,
    closeOnSelect = false,
    ...restProps
  }: WithoutChild<DropdownMenuPrimitive.RadioItemProps> = $props();
</script>

<DropdownMenuPrimitive.RadioItem
  bind:ref
  {closeOnSelect}
  data-slot="dropdown-menu-radio-item"
  class={cn(
    "focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm data-inset:pl-8 [&_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
    className,
  )}
  {...restProps}
>
  {#snippet children({ checked })}
    <span
      class="absolute right-2 flex items-center justify-center pointer-events-none"
      data-slot="dropdown-menu-radio-item-indicator"
    >
      {#if checked}
        <CheckIcon />
      {/if}
    </span>
    {@render childrenProp?.({ checked })}
  {/snippet}
</DropdownMenuPrimitive.RadioItem>
