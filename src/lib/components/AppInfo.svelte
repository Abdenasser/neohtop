<script lang="ts">
  import { getVersion } from "@tauri-apps/api/app";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { themeStore } from "$lib/stores/theme";
  import { ASCII_ART, APP_INFO } from "$lib/constants";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { ExternalLink, RefreshCw } from "@lucide/svelte";
  import { isNewerVersion } from "$lib/utils/app-version.js";
  let version = "";
  let latestVersion = "";
  let checking = false;
  let failed = false;
  let controller: AbortController | undefined;
  $: hasUpdate = isNewerVersion(latestVersion, version);
  async function checkLatestVersion() {
    if (checking) return;
    checking = true;
    failed = false;
    controller = new AbortController();
    const timeout = setTimeout(() => controller?.abort(), 10000);
    try {
      const response = await fetch(
        "https://api.github.com/repos/abdenasser/neohtop/releases/latest",
        { signal: controller.signal },
      );
      if (!response.ok) throw new Error("Unavailable");
      const data = await response.json();
      latestVersion =
        typeof data.tag_name === "string"
          ? data.tag_name.match(/\d+\.\d+\.\d+/)?.[0] || ""
          : "";
      failed = !latestVersion;
    } catch {
      failed = true;
    } finally {
      clearTimeout(timeout);
      checking = false;
    }
  }
  onMount(() => {
    let active = true;
    let stop: (() => void) | undefined;
    themeStore.init();
    void listen("check-updates", () => {
      void checkLatestVersion();
    }).then((unlisten) => {
      if (active) stop = unlisten;
      else unlisten();
    });
    void (async () => {
      try {
        const next = await getVersion();
        if (!active) return;
        version = next;
        await checkLatestVersion();
      } catch {
        /* Desktop version metadata is not available in a browser. */
      }
    })();
    return () => {
      active = false;
      stop?.();
      controller?.abort();
    };
  });
</script>

<div class="h-screen space-y-5 overflow-y-auto bg-background p-6">
  <div class="flex items-center gap-3">
    <img src="/32x32.png" alt="" class="size-10" />
    <div>
      <h2 class="font-semibold">NeoHtop {version ? `v${version}` : ""}</h2>
      <p class="mt-1 text-xs text-muted-foreground">
        {APP_INFO.name} · by {APP_INFO.developer}
      </p>
    </div>
    <Badge variant="secondary" class="ml-auto font-normal">Open source</Badge>
  </div>
  <pre
    class="overflow-x-auto rounded-md border border-border bg-background p-3 font-mono text-[8px] leading-tight text-primary">{ASCII_ART}</pre>
  <div class="space-y-2 text-xs">
    <div class="flex justify-between">
      <span class="text-muted-foreground">Stack</span><span
        >{APP_INFO.stack.join(" · ")}</span
      >
    </div>
    <Button
      variant="link"
      size="sm"
      class="h-auto p-0 text-xs"
      href={APP_INFO.github}
      target="_blank"
      rel="noopener noreferrer"
      >Source on GitHub<ExternalLink class="size-3" /></Button
    >
  </div>
  <div
    class="flex items-center justify-between gap-3 border-t border-border pt-3"
  >
    <span class="text-xs text-muted-foreground"
      >{checking
        ? "Checking latest release…"
        : failed
          ? "Release check unavailable"
          : hasUpdate
            ? `Version ${latestVersion} available`
            : latestVersion
              ? "You have the latest stable release"
              : "Desktop version information"}</span
    >{#if hasUpdate}<Button
        variant="outline"
        size="sm"
        href="https://github.com/abdenasser/neohtop/releases/latest"
        target="_blank"
        rel="noopener noreferrer">View release<ExternalLink /></Button
      >{:else if failed}<Button
        variant="ghost"
        size="icon-sm"
        aria-label="Retry release check"
        onclick={checkLatestVersion}><RefreshCw /></Button
      >{/if}
  </div>
</div>
