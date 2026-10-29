<script lang="ts">
  import { themeStore } from "$lib/stores/theme";
  import ThemeGallery from "./ThemeGallery.svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { Palette } from "@lucide/svelte";
  let open = false;
</script>

<Popover.Root bind:open>
  <Popover.Trigger
    >{#snippet child({ props })}<Button
        {...props}
        variant="ghost"
        size="icon-sm"
        aria-label={`Theme: ${$themeStore.label}`}
        title={`Theme: ${$themeStore.label}`}><Palette /></Button
      >{/snippet}</Popover.Trigger
  >
  <Popover.Content align="end" class="w-96 max-w-[calc(100vw-2rem)] p-4">
    <h2 class="mb-3 text-sm font-medium">Choose a theme</h2>
    <ThemeGallery onselect={() => (open = false)} />
  </Popover.Content>
</Popover.Root>
