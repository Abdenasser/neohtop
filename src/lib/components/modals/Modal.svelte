<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  export let show = false;
  export let maxWidth = "600px";
  export let title: string;
  export let onClose: () => void;
</script>

<Dialog.Root
  open={show}
  onOpenChange={(open) => {
    if (!open) onClose();
  }}
  ><Dialog.Content
    class="max-h-[85vh] overflow-y-auto"
    style={`max-width:min(calc(100vw - 2rem),${maxWidth})`}
    ><Dialog.Header
      ><Dialog.Title>{title}</Dialog.Title><Dialog.Description class="sr-only"
        >NeoHtop details</Dialog.Description
      ></Dialog.Header
    ><slot /></Dialog.Content
  ></Dialog.Root
>
