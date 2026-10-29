<script lang="ts">
  import { settingsStore } from "$lib/stores";
  import { ITEMS_PER_PAGE_OPTIONS } from "$lib/constants";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import * as NativeSelect from "$lib/components/ui/native-select/index.js";
  import {
    ChevronLeft,
    ChevronRight,
    ChevronsLeft,
    ChevronsRight,
  } from "@lucide/svelte";
  export let itemsPerPage: number;
  export let currentPage: number;
  export let totalPages: number;
  export let totalResults: number;
  export let resultLabel = "processes";
  $: options = ITEMS_PER_PAGE_OPTIONS.includes(itemsPerPage)
    ? ITEMS_PER_PAGE_OPTIONS
    : [itemsPerPage, ...ITEMS_PER_PAGE_OPTIONS];
  function save() {
    currentPage = 1;
    settingsStore.updateConfig({
      behavior: { ...$settingsStore.behavior, itemsPerPage },
    });
  }
</script>

<footer
  class="flex shrink-0 flex-wrap items-center justify-between gap-3 border-t border-border bg-card px-4 py-3"
  aria-label="Process pagination"
>
  <span class="text-xs text-muted-foreground"
    >{totalResults.toLocaleString()} {resultLabel}</span
  >
  <div class="flex items-center gap-3">
    <Label class="gap-2 text-xs font-normal text-muted-foreground"
      >Rows<NativeSelect.Root
        bind:value={itemsPerPage}
        onchange={save}
        class="w-20"
        size="sm"
        aria-label="Rows per page"
        >{#each options as value}<option {value}>{value}</option
          >{/each}</NativeSelect.Root
      ></Label
    ><span class="text-xs text-muted-foreground"
      >Page {currentPage} of {Math.max(1, totalPages)}</span
    >
    <div class="flex gap-1">
      <Button
        variant="outline"
        size="icon-sm"
        aria-label="First page"
        disabled={currentPage <= 1}
        onclick={() => (currentPage = 1)}><ChevronsLeft /></Button
      ><Button
        variant="outline"
        size="icon-sm"
        aria-label="Previous page"
        disabled={currentPage <= 1}
        onclick={() => currentPage--}><ChevronLeft /></Button
      ><Button
        variant="outline"
        size="icon-sm"
        aria-label="Next page"
        disabled={currentPage >= totalPages}
        onclick={() => currentPage++}><ChevronRight /></Button
      ><Button
        variant="outline"
        size="icon-sm"
        aria-label="Last page"
        disabled={currentPage >= totalPages}
        onclick={() => (currentPage = totalPages)}><ChevronsRight /></Button
      >
    </div>
  </div>
</footer>
