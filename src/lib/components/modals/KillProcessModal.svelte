<script lang="ts">
  import * as AlertDialog from "$lib/components/ui/alert-dialog/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { LoaderCircle } from "@lucide/svelte";
  export let show = false;
  export let process: { pid: number; name: string } | null = null;
  export let onClose: () => void;
  export let onConfirm: () => Promise<void>;
  export let isKilling = false;
</script>

<AlertDialog.Root
  open={show}
  onOpenChange={(open) => {
    if (!open && !isKilling) onClose();
  }}
  ><AlertDialog.Content
    onEscapeKeydown={(event) => {
      if (isKilling) event.preventDefault();
    }}
    ><AlertDialog.Header
      ><AlertDialog.Title
        >End {process?.name || "this process"}?</AlertDialog.Title
      ><AlertDialog.Description
        >This stops the selected process. Unsaved work may be lost.</AlertDialog.Description
      ></AlertDialog.Header
    >{#if process}<div
        class="rounded-md border border-border bg-muted/50 px-4 py-3"
      >
        <p class="break-all text-sm font-medium">{process.name}</p>
        <p class="mt-1 font-mono text-xs text-muted-foreground">
          PID {process.pid}
        </p>
      </div>{/if}<AlertDialog.Footer
      ><AlertDialog.Cancel disabled={isKilling}>Cancel</AlertDialog.Cancel
      ><Button
        variant="destructive"
        disabled={isKilling || !process}
        onclick={onConfirm}
        >{#if isKilling}<LoaderCircle class="animate-spin" />Ending…{:else}End
          process{/if}</Button
      ></AlertDialog.Footer
    ></AlertDialog.Content
  ></AlertDialog.Root
>
