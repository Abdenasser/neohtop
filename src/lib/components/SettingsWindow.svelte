<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { emit, listen } from "@tauri-apps/api/event";
  import { settingsStore, settingsStatus } from "$lib/stores/settings";
  import { REFRESH_RATE_OPTIONS } from "$lib/constants";
  import ThemeGallery from "./ThemeGallery.svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Checkbox } from "$lib/components/ui/checkbox/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import * as NativeSelect from "$lib/components/ui/native-select/index.js";
  import * as ToggleGroup from "$lib/components/ui/toggle-group/index.js";
  import { Settings2, Palette } from "@lucide/svelte";
  let pane = "general";
  let runtimeError = "";
  $: rate = $settingsStore.behavior.refreshRate;
  $: options = REFRESH_RATE_OPTIONS.some((o) => o.value === rate)
    ? REFRESH_RATE_OPTIONS
    : [{ value: rate, label: `${rate / 1000}s` }, ...REFRESH_RATE_OPTIONS];
  function select(value: string) {
    if (value !== "general" && value !== "appearance") return;
    pane = value;
    try {
      localStorage.setItem("settings-pane", value);
    } catch {
      /* Session selection remains available. */
    }
  }
  function save(
    key: "refreshRate" | "menuBarEnabled" | "keepRunning",
    value: number | boolean,
  ) {
    settingsStore.updateConfig({
      behavior: { ...$settingsStore.behavior, [key]: value },
    });
  }
  onMount(() => {
    let disposed = false;
    const stops: (() => void)[] = [];
    void (async () => {
      const stopPane = await listen<string>("settings-pane", ({ payload }) =>
        select(payload),
      );
      const stopError = await listen<string>(
        "preferences-runtime-error",
        ({ payload }) => {
          runtimeError = payload;
        },
      );
      if (disposed) {
        stopPane();
        stopError();
        return;
      }
      stops.push(stopPane, stopError);
      select(
        (await invoke<string>("settings_pane")) ||
          localStorage.getItem("settings-pane") ||
          "general",
      );
      await settingsStore.init();
      await emit("preferences-runtime-status-request");
    })();
    return () => {
      disposed = true;
      stops.forEach((stop) => stop());
    };
  });
</script>

<div class="flex h-screen flex-col bg-background">
  <header
    class="flex shrink-0 justify-center border-b border-border bg-card px-6 py-4"
  >
    <ToggleGroup.Root
      type="single"
      value={pane}
      onValueChange={select}
      variant="outline"
      aria-label="Settings section"
    >
      <ToggleGroup.Item value="general" class="px-5"
        ><Settings2 />General</ToggleGroup.Item
      >
      <ToggleGroup.Item value="appearance" class="px-5"
        ><Palette />Appearance</ToggleGroup.Item
      >
    </ToggleGroup.Root>
  </header>
  <div class="min-h-0 flex-1 overflow-y-auto p-6">
    {#if pane === "general"}
      <h1 class="text-lg font-semibold tracking-tight">General</h1>
      <p class="mt-1 mb-5 text-sm text-muted-foreground">
        Choose how NeoHtop monitors your system.
      </p>
      <fieldset disabled={!$settingsStatus.ready} class="space-y-5">
        <Card.Root size="sm"
          ><Card.Header><Card.Title>Monitoring</Card.Title></Card.Header
          ><Card.Content>
            <div class="flex items-center justify-between gap-6">
              <div>
                <Label for="refresh-interval">Refresh interval</Label>
                <p class="mt-1 text-xs leading-relaxed text-muted-foreground">
                  Applies to the app and menu-bar overview.
                </p>
              </div>
              <NativeSelect.Root
                id="refresh-interval"
                class="w-28"
                value={rate}
                onchange={(event) =>
                  save("refreshRate", Number(event.currentTarget.value))}
              >
                {#each options as option}<option value={option.value}
                    >{option.label}</option
                  >{/each}
              </NativeSelect.Root>
            </div>
            <p class="mt-1 text-xs text-muted-foreground">
              Longer intervals reduce background activity. Pause and resume from
              the app bar.
            </p>
          </Card.Content></Card.Root
        >
        <Card.Root size="sm"
          ><Card.Header><Card.Title>Menu-bar companion</Card.Title></Card.Header
          ><Card.Content class="gap-5">
            <div class="flex items-start gap-3">
              <Checkbox
                id="show-menu-icon"
                class="mt-0.5"
                checked={$settingsStore.behavior.menuBarEnabled}
                onCheckedChange={(value) => save("menuBarEnabled", value)}
              />
              <div>
                <Label for="show-menu-icon">Show menu-bar icon</Label>
                <p class="mt-1 text-xs leading-relaxed text-muted-foreground">
                  Keep CPU, memory, graphics and network readings close at hand.
                </p>
              </div>
            </div>
            <div class="flex items-start gap-3">
              <Checkbox
                id="keep-running"
                class="mt-0.5"
                checked={$settingsStore.behavior.keepRunning}
                disabled={!$settingsStore.behavior.menuBarEnabled}
                onCheckedChange={(value) => save("keepRunning", value)}
              />
              <div
                class={!$settingsStore.behavior.menuBarEnabled
                  ? "opacity-50"
                  : ""}
              >
                <Label for="keep-running"
                  >Keep running when the window closes</Label
                >
                <p class="mt-1 text-xs leading-relaxed text-muted-foreground">
                  Continue from the menu bar. Choose Quit NeoHtop to exit
                  completely.
                </p>
              </div>
            </div>
          </Card.Content></Card.Root
        >
      </fieldset>
    {:else}
      <h1 class="text-lg font-semibold tracking-tight">Appearance</h1>
      <p class="mt-1 mb-5 text-sm text-muted-foreground">
        All your NeoHtop themes, across every window.
      </p>
      <ThemeGallery embedded />
    {/if}
  </div>
  <footer class="shrink-0 border-t border-border bg-card px-6 py-3">
    {#if $settingsStatus.error || runtimeError}
      <div
        role="alert"
        class="flex items-center justify-between gap-3 text-xs text-destructive"
      >
        <span>{$settingsStatus.error || runtimeError}</span><Button
          variant="outline"
          size="sm"
          onclick={() => {
            void settingsStore.retry();
            void emit("preferences-runtime-retry");
          }}>Retry</Button
        >
      </div>
    {:else}<p role="status" class="text-xs text-muted-foreground">
        {!$settingsStatus.ready
          ? "Loading settings…"
          : $settingsStatus.saving
            ? "Saving changes…"
            : "Changes are saved automatically."}
      </p>{/if}
  </footer>
</div>
