<script lang="ts">
  import { onMount } from "svelte";
  import { invoke, isTauri } from "@tauri-apps/api/core";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import { X } from "@lucide/svelte";
  let visible = false;
  const key = "neohtop-analytics-notice-v1";
  onMount(() => {
    if (!isTauri()) return;
    void invoke<boolean>("analytics_configured")
      .then((configured) => {
        try {
          visible = configured && localStorage.getItem(key) !== "seen";
        } catch {
          visible = configured;
        }
      })
      .catch(() => {});
  });
  function dismiss() {
    visible = false;
    try {
      localStorage.setItem(key, "seen");
    } catch {
      /* Available again next launch. */
    }
  }
</script>

{#if visible}
  <Card.Root size="sm" role="status" class="mb-3"
    ><Card.Content class="flex-row items-start gap-3">
      <div class="min-w-0 flex-1">
        <p class="text-sm font-medium">A small change to privacy in v1.3</p>
        <p class="mt-1 text-xs leading-relaxed text-muted-foreground">
          NeoHtop sends a basic app-open count and version each launch, without
          a persistent identifier. Feature-usage sharing is off by default. Your
          process and system readings stay here.
        </p>
        <Button
          variant="link"
          size="sm"
          class="h-auto p-0 pt-2"
          onclick={() => {
            void invoke("open_settings", { pane: "privacy" });
          }}>Review privacy settings</Button
        >
      </div>
      <Button
        variant="ghost"
        size="icon-sm"
        aria-label="Dismiss privacy notice"
        onclick={dismiss}><X /></Button
      >
    </Card.Content></Card.Root
  >
{/if}
