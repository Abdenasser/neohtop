<script lang="ts">
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
  >Status<NativeSelect.Root
    bind:value={statusFilter}
    onchange={save}
    aria-label="Process status"
    >{#each STATUS_OPTIONS as option}<option value={option.value}
        >{option.label}</option
      >{/each}</NativeSelect.Root
  ></Label
>
