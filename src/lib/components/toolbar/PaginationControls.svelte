<script lang="ts">
  import { t, type MessageKey } from "$lib/i18n";
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
  export let groupFamilies = false;
  let countKey: MessageKey;
  $: countKey = groupFamilies
    ? totalResults === 1
      ? "process.familyOne"
      : "process.familyMany"
    : totalResults === 1
      ? "process.countOne"
      : "process.count";
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
  aria-label={$t("paginationControls.processPagination")}
>
  <span class="text-xs text-muted-foreground"
    >{$t(countKey, { count: totalResults.toLocaleString() })}</span
  >
  <div class="flex items-center gap-3">
    <Label class="gap-2 text-xs font-normal text-muted-foreground"
      >{$t("paginationControls.rows")}<NativeSelect.Root
        bind:value={itemsPerPage}
        onchange={save}
        class="w-20"
        size="sm"
        aria-label={$t("paginationControls.rowsPerPage")}
        >{#each options as value}<option {value}>{value}</option
          >{/each}</NativeSelect.Root
      ></Label
    ><span class="text-xs text-muted-foreground"
      >{$t("process.page", {
        current: currentPage,
        total: Math.max(1, totalPages),
      })}</span
    >
    <div class="flex gap-1">
      <Button
        variant="outline"
        size="icon-sm"
        aria-label={$t("paginationControls.firstPage")}
        disabled={currentPage <= 1}
        onclick={() => (currentPage = 1)}><ChevronsLeft /></Button
      ><Button
        variant="outline"
        size="icon-sm"
        aria-label={$t("paginationControls.previousPage")}
        disabled={currentPage <= 1}
        onclick={() => currentPage--}><ChevronLeft /></Button
      ><Button
        variant="outline"
        size="icon-sm"
        aria-label={$t("paginationControls.nextPage")}
        disabled={currentPage >= totalPages}
        onclick={() => currentPage++}><ChevronRight /></Button
      ><Button
        variant="outline"
        size="icon-sm"
        aria-label={$t("paginationControls.lastPage")}
        disabled={currentPage >= totalPages}
        onclick={() => (currentPage = totalPages)}><ChevronsRight /></Button
      >
    </div>
  </div>
</footer>
