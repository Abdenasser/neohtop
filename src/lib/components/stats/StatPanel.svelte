<script lang="ts">
  import * as Card from "$lib/components/ui/card/index.js";
  export let title: string;
  export let flex = 1;
</script>

<Card.Root size="sm" class="gap-3" style={`flex:${flex}`}
  ><Card.Header><Card.Title class="text-sm">{title}</Card.Title></Card.Header
  ><Card.Content><slot /></Card.Content></Card.Root
>
