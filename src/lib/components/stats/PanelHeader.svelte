<script lang="ts">
  import { Badge } from "$lib/components/ui/badge/index.js";
  import Fa from "svelte-fa";
  import type { IconDefinition } from "@fortawesome/free-solid-svg-icons";
  export let icon: IconDefinition;
  export let title: string;
  export let usageValue: string | null = null;
</script>

<div class="flex items-center gap-2 text-sm font-medium">
  <Fa {icon} color="var(--primary)" />
  <h3>{title}</h3>
  {#if usageValue}<Badge
      variant="secondary"
      class="ml-auto font-normal tabular-nums">{usageValue}</Badge
    >{/if}
</div>
