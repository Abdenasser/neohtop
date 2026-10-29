<script lang="ts">
  import { onMount } from "svelte";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { Search, X, CircleHelp } from "@lucide/svelte";
  export let searchTerm: string;
  let input: HTMLInputElement | null = null;
  let open = false;
  const examples = [
    ["systemd, dbus", "Multiple names, separated by commas"],
    ["d$", "Names ending in d"],
    ["^kernel", "Names starting with kernel"],
    ["ssh.*server", "Regular expressions"],
    ["1234", "Find a process ID"],
    [":3000", "Processes listening on a local port"],
    ["port:443", "Explicit local TCP listener or UDP port"],
    ["python, node, nginx", "Several runtimes"],
    ["docker, containerd", "Container processes"],
    ["gnome, plasma", "Desktop environments"],
  ];
  function useExample(query: string) {
    searchTerm = query;
    open = false;
    requestAnimationFrame(() => input?.focus());
  }
  onMount(() => {
    const key = (event: KeyboardEvent) => {
      if (
        (event.metaKey || event.ctrlKey) &&
        event.key.toLowerCase() === "f" &&
        !document.querySelector('[role="dialog"],[role="alertdialog"]')
      ) {
        event.preventDefault();
        input?.focus();
      }
    };
    document.addEventListener("keydown", key);
    return () => document.removeEventListener("keydown", key);
  });
</script>

<div class="flex min-w-56 max-w-md flex-1 items-center gap-1">
  <div class="relative min-w-0 flex-1">
    <Search
      class="pointer-events-none absolute left-3 top-2.5 size-4 text-muted-foreground"
    /><Input
      bind:ref={input}
      bind:value={searchTerm}
      aria-label="Search processes"
      placeholder="Search processes…"
      class="h-9 pl-9 pr-9"
      onkeydown={(event) => {
        if (event.key === "Escape") {
          searchTerm = "";
          event.preventDefault();
        }
      }}
    />{#if searchTerm}<Button
        variant="ghost"
        size="icon-xs"
        class="absolute right-1.5 top-1.5 text-muted-foreground"
        aria-label="Clear search"
        onclick={() => {
          searchTerm = "";
          input?.focus();
        }}><X /></Button
      >{/if}
  </div>
  <Popover.Root bind:open
    ><Popover.Trigger
      >{#snippet child({ props })}<Button
          {...props}
          variant="ghost"
          size="icon-sm"
          aria-label="Search syntax and examples"
          ><CircleHelp class="size-4 text-muted-foreground" /></Button
        >{/snippet}</Popover.Trigger
    ><Popover.Content
      align="start"
      class="max-h-[70vh] w-80 overflow-y-auto space-y-4"
      ><div>
        <h2 class="text-sm font-medium">Find a process</h2>
        <p class="mt-1 text-xs text-muted-foreground">
          Search names, commands, PIDs or local ports (:3000). Comma-separated
          terms match any term. Name searches also support regular expressions.
        </p>
      </div>
      <div class="space-y-1">
        {#each examples as [query, label]}<Button
            variant="ghost"
            class="h-auto w-full items-start justify-start gap-3 whitespace-normal py-2 text-left"
            onclick={() => useExample(query)}
            ><code class="shrink-0 text-xs text-primary">{query}</code><span
              class="text-xs font-normal text-muted-foreground">{label}</span
            ></Button
          >{/each}
      </div>
      <p class="border-t border-border pt-3 text-xs text-muted-foreground">
        ⌘ / Ctrl F focuses search. Escape clears it. Invalid expressions still
        match literal text.
      </p></Popover.Content
    ></Popover.Root
  >
</div>
