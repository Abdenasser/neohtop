<script lang="ts">
  import { t, processStatus } from "$lib/i18n";
  import { STATUS_OPTIONS } from "$lib/constants";
  import { settingsStore } from "$lib/stores";
  import { Label } from "$lib/components/ui/label/index.js";
  import * as NativeSelect from "$lib/components/ui/native-select/index.js";
  export let statusFilter = "all";
  function save() {
    settingsStore.updateConfig({
      behavior: {
        ...$settingsStore.behavior,
        defaultStatusFilter: statusFilter,
      },
    });
  }
</script>

<Label class="gap-2 text-xs"
  >{$t("statusFilter.status")}<NativeSelect.Root
    bind:value={statusFilter}
    onchange={save}
    aria-label={$t("statusFilter.processStatus")}
    >{#each STATUS_OPTIONS as option}<option value={option.value}
        >{option.value === "all"
          ? $t("status.all")
          : processStatus($t, option.label)}</option
      >{/each}</NativeSelect.Root
  ></Label
>
