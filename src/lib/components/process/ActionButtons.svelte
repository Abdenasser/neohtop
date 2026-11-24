<script lang="ts">
  import { t } from "$lib/i18n";
  import type { Process } from "$lib/types";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Table from "$lib/components/ui/table/index.js";
  import { Pin, Info, X } from "@lucide/svelte";
  export let process: Process;
  export let isPinned: boolean;
  export let onTogglePin: (command: string) => void;
  export let onShowDetails: (process: Process) => void;
  export let onKillProcess: (process: Process) => void;
</script>

<Table.Cell class="actions-cell w-28 pr-3"
  ><div class="flex items-center justify-end gap-1">
    <Button
      variant="ghost"
      size="icon-xs"
      class={isPinned ? "text-primary" : "text-muted-foreground"}
      aria-label={$t("actionButtons.valueName", {
        value: isPinned ? $t("actionButtons.unpin") : $t("actionButtons.pin"),
        name: process.name,
      })}
      title={isPinned ? $t("actionButtons.unpin") : $t("actionButtons.pin")}
      onclick={() => onTogglePin(process.command)}
      ><Pin class={isPinned ? "fill-primary/20" : ""} /></Button
    ><Button
      variant="ghost"
      size="icon-xs"
      class="text-muted-foreground"
      aria-label={$t("actionButtons.inspectName", { name: process.name })}
      title={$t("actionButtons.showDetails")}
      onclick={() => onShowDetails(process)}><Info /></Button
    ><Button
      variant="ghost"
      size="icon-xs"
      class="text-muted-foreground hover:bg-destructive/10 hover:text-destructive"
      aria-label={$t("actionButtons.endName", { name: process.name })}
      title={$t("actionButtons.endProcess")}
      onclick={() => onKillProcess(process)}><X /></Button
    >
  </div></Table.Cell
>
