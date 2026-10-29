<script lang="ts">
  import { Dialog as SheetPrimitive } from "bits-ui";
  import { cn } from "$lib/shadcn-utils.js";

  let {
    ref = $bindable(null),
    class: className,
    ...restProps
  }: SheetPrimitive.TitleProps = $props();
</script>

<SheetPrimitive.Title
  bind:ref
  data-slot="sheet-title"
  class={cn("cn-font-heading text-foreground font-medium", className)}
  {...restProps}
/>
