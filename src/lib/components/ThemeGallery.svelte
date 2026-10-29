<script lang="ts">
  import { onMount } from "svelte";
  import { platform } from "@tauri-apps/plugin-os";
  import { themeStore } from "$lib/stores";
  import { themes } from "$lib/definitions";
  import { matchesTheme } from "$lib/utils/theme-search.js";
  import { THEME_GROUPS } from "$lib/constants";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Check, Search } from "@lucide/svelte";
  let glassySupported = false;
  export let onselect: () => void = () => {};
  export let embedded = false;
  let query = "";
  onMount(() => {
    try {
      const os = platform();
      glassySupported = os === "windows" || os === "macos";
    } catch {
      /* Native-only effects are absent in browser builds. */
    }
  });
  $: groups = [
    ...THEME_GROUPS,
    ...(glassySupported ? [{ label: "Glassy", themes: ["glassy"] }] : []),
  ];
  $: filtered = groups
    .map((group) => ({
      ...group,
      themes: group.themes.filter((name) =>
        matchesTheme(themes[name].label, query),
      ),
    }))
    .filter((group) => group.themes.length);
  function select(name: string) {
    themeStore.setTheme(name);
    onselect();
    query = "";
  }
</script>

<div class="relative">
  <Search
    class="pointer-events-none absolute left-3 top-2.5 size-4 text-muted-foreground"
  /><Input
    aria-label="Find a theme"
    placeholder="Find a theme…"
    bind:value={query}
    class="pl-9"
  />
</div>
<div
  class={`mt-3 space-y-4 pr-1 ${embedded ? "" : "max-h-[min(55vh,440px)] overflow-y-auto"}`}
>
  {#each filtered as group}<div>
      <h3 class="mb-1 px-2 text-xs font-medium text-muted-foreground">
        {group.label}
      </h3>
      <div class="grid grid-cols-2 gap-1">
        {#each group.themes as name}{@const theme = themes[name]}<Button
            variant={$themeStore.name === name ? "secondary" : "ghost"}
            size="sm"
            class="h-auto min-h-9 justify-start gap-2 py-2 text-xs"
            aria-pressed={$themeStore.name === name}
            onclick={() => select(name)}
            ><span class="flex shrink-0 gap-0.5" aria-hidden="true"
              >{#each [theme.colors.blue, theme.colors.red, theme.colors.green] as color}<span
                  class="size-2 rounded-full"
                  style:background={color}
                ></span>{/each}</span
            >{theme.label}{#if $themeStore.name === name}<Check
                class="ml-auto size-3"
              />{/if}</Button
          >{/each}
      </div>
    </div>{:else}<p class="p-3 text-center text-sm text-muted-foreground">
      No matching themes.
    </p>{/each}
</div>
