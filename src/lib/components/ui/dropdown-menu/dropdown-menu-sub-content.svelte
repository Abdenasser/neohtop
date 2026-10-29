<script lang="ts">
  import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui";
  import { cn, type WithoutChildrenOrChild } from "$lib/shadcn-utils.js";
  import DropdownMenuPortal from "./dropdown-menu-portal.svelte";
  import type { ComponentProps } from "svelte";

  let {
    ref = $bindable(null),
    class: className,
    align = "start",
    alignOffset = -3,
    portalProps,
    ...restProps
  }: DropdownMenuPrimitive.SubContentProps & {
    portalProps?: WithoutChildrenOrChild<
      ComponentProps<typeof DropdownMenuPortal>
    >;
  } = $props();
</script>

<DropdownMenuPortal {...portalProps}>
  <DropdownMenuPrimitive.SubContent
    bind:ref
    data-slot="dropdown-menu-sub-content"
    {align}
    {alignOffset}
    class={cn(
      "data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-border bg-popover text-popover-foreground min-w-[96px] rounded-md p-1 shadow-lg ring-1 duration-100 cn-menu-target cn-menu-translucent z-50 origin-(--bits-dropdown-menu-content-transform-origin) overflow-hidden",
      className,
    )}
    {...restProps}
  />
</DropdownMenuPortal>
