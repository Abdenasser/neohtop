<script lang="ts">
  import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui";
  import { CheckIcon, MinusIcon } from "@lucide/svelte";
  import { cn, type WithoutChildrenOrChild } from "$lib/shadcn-utils.js";
  import type { Snippet } from "svelte";

  let {
    ref = $bindable(null),
    checked = $bindable(false),
    indeterminate = $bindable(false),
    class: className,
    children: childrenProp,
    closeOnSelect = false,
    ...restProps
  }: WithoutChildrenOrChild<DropdownMenuPrimitive.CheckboxItemProps> & {
    children?: Snippet;
  } = $props();
</script>

<DropdownMenuPrimitive.CheckboxItem
  bind:ref
  {closeOnSelect}
  bind:checked
  bind:indeterminate
  data-slot="dropdown-menu-checkbox-item"
  class={cn(
    "focus:bg-accent focus:text-accent-foreground focus:**:text-accent-foreground gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm data-inset:pl-8 [&_svg:not([class*='size-'])]:size-4 relative flex cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
    className,
  )}
  {...restProps}
>
  {#snippet children({ checked, indeterminate })}
    <span
      class="absolute right-2 flex items-center justify-center pointer-events-none"
      data-slot="dropdown-menu-checkbox-item-indicator"
    >
      {#if indeterminate}
        <MinusIcon />
      {:else if checked}
        <CheckIcon />
      {/if}
    </span>
    {@render childrenProp?.()}
  {/snippet}
</DropdownMenuPrimitive.CheckboxItem>
