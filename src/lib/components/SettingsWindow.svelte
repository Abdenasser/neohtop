<script lang="ts">
  import { t } from "$lib/i18n";
  import type { AppConfig } from "$lib/types";
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
  import { Settings2, Palette, ShieldCheck } from "@lucide/svelte";
  let pane = "general";
  let runtimeError = "";
  let analyticsConfigured = false;
  $: rate = $settingsStore.behavior.refreshRate;
  $: options = REFRESH_RATE_OPTIONS.some((o) => o.value === rate)
    ? REFRESH_RATE_OPTIONS
    : [{ value: rate, label: `${rate / 1000}s` }, ...REFRESH_RATE_OPTIONS];
  function select(value: string) {
    if (!["general", "appearance", "privacy"].includes(value)) return;
    pane = value;
    try {
      localStorage.setItem("settings-pane", value);
    } catch {
      /* Session selection remains available. */
    }
  }
  function save<K extends keyof AppConfig["behavior"]>(
    key: K,
    value: AppConfig["behavior"][K],
  ) {
    const behavior = { ...$settingsStore.behavior, [key]: value };
    settingsStore.updateConfig({ behavior });
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
      analyticsConfigured = await invoke<boolean>("analytics_configured");
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
      aria-label={$t("settingsWindow.settingsSection")}
    >
      <ToggleGroup.Item value="general" class="px-5"
        ><Settings2 />{$t("settingsWindow.general")}</ToggleGroup.Item
      >
      <ToggleGroup.Item value="appearance" class="px-5"
        ><Palette />{$t("settingsWindow.appearance")}</ToggleGroup.Item
      >
      <ToggleGroup.Item value="privacy" class="px-5"
        ><ShieldCheck />{$t("settingsWindow.privacy")}</ToggleGroup.Item
      >
    </ToggleGroup.Root>
  </header>
  <div class="min-h-0 flex-1 overflow-y-auto p-6">
    {#if pane === "general"}
      <h1 class="text-lg font-semibold tracking-tight">
        {$t("settingsWindow.general")}
      </h1>
      <p class="mt-1 mb-5 text-sm text-muted-foreground">
        {$t("settingsWindow.chooseHowNeohtopMonitorsYourSystem")}
      </p>
      <fieldset disabled={!$settingsStatus.ready} class="space-y-5">
        <Card.Root size="sm">
          <Card.Content class="flex-row items-center justify-between gap-6">
            <Label for="display-language">{$t("settingsWindow.language")}</Label
            >
            <NativeSelect.Root
              id="display-language"
              class="w-28"
              value={$settingsStore.behavior.language}
              onchange={(event) =>
                save(
                  "language",
                  event.currentTarget.value === "en" ? "en" : "zh-CN",
                )}
            >
              <option value="zh-CN">简体中文</option>
              <option value="en">English</option>
            </NativeSelect.Root>
          </Card.Content>
        </Card.Root>
        <Card.Root size="sm"
          ><Card.Header
            ><Card.Title>{$t("settingsWindow.monitoring")}</Card.Title
            ></Card.Header
          ><Card.Content>
            <div class="flex items-center justify-between gap-6">
              <div>
                <Label for="refresh-interval"
                  >{$t("settingsWindow.refreshInterval")}</Label
                >
                <p class="mt-1 text-xs leading-relaxed text-muted-foreground">
                  {$t("settingsWindow.appliesToTheAppAndMenuBar")}
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
                    >{$t("common.second", {
                      value: option.value / 1000,
                    })}</option
                  >{/each}
              </NativeSelect.Root>
            </div>
            <p class="mt-1 text-xs text-muted-foreground">
              {$t(
                "settingsWindow.longerIntervalsReduceBackgroundActivityPauseAnd",
              )}
            </p>
          </Card.Content></Card.Root
        >
        <Card.Root size="sm"
          ><Card.Header
            ><Card.Title>{$t("settingsWindow.menuBarCompanion")}</Card.Title
            ></Card.Header
          ><Card.Content class="gap-5">
            <div class="flex items-start gap-3">
              <Checkbox
                id="show-menu-icon"
                class="mt-0.5"
                checked={$settingsStore.behavior.menuBarEnabled}
                onCheckedChange={(value) => save("menuBarEnabled", value)}
              />
              <div>
                <Label for="show-menu-icon"
                  >{$t("settingsWindow.showMenuBarIcon")}</Label
                >
                <p class="mt-1 text-xs leading-relaxed text-muted-foreground">
                  {$t("settingsWindow.keepCpuMemoryGraphicsAndNetworkReadings")}
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
                  >{$t("settingsWindow.keepRunningWhenTheWindowCloses")}</Label
                >
                <p class="mt-1 text-xs leading-relaxed text-muted-foreground">
                  {$t("settingsWindow.continueFromTheMenuBarChooseQuit")}
                </p>
              </div>
            </div>
          </Card.Content></Card.Root
        >
      </fieldset>
    {:else if pane === "privacy"}
      <h1 class="text-lg font-semibold tracking-tight">
        {$t("settingsWindow.privacy")}
      </h1>
      <p class="mt-1 mb-5 text-sm text-muted-foreground">
        {$t("settingsWindow.yourSystemReadingsStayOnYourMachine")}
      </p>
      <div class="space-y-5">
        <Card.Root size="sm"
          ><Card.Header
            ><Card.Title>{$t("settingsWindow.basicLaunchCounter")}</Card.Title
            ></Card.Header
          ><Card.Content class="gap-3">
            <p class="text-sm leading-relaxed">
              {$t("settingsWindow.officialBuildsSendOneAppOpenEvent")}
            </p>
            <p class="text-xs leading-relaxed text-muted-foreground">
              {$t("settingsWindow.noPersistentIdentifierOrSystemReadingsAre")}
            </p>
            {#if !analyticsConfigured}<p class="text-xs text-muted-foreground">
                {$t("settingsWindow.analyticsIsNotConnectedInThisBuild")}
              </p>{/if}
          </Card.Content></Card.Root
        >
        <Card.Root size="sm"
          ><Card.Header
            ><Card.Title>{$t("settingsWindow.helpImproveNeohtop")}</Card.Title
            ></Card.Header
          ><Card.Content class="gap-4">
            <div class="flex items-start gap-3">
              <Checkbox
                id="usage-analytics"
                class="mt-0.5"
                checked={$settingsStore.behavior.usageAnalytics}
                disabled={!$settingsStatus.ready || !analyticsConfigured}
                onCheckedChange={(value) => save("usageAnalytics", value)}
              />
              <div>
                <Label for="usage-analytics"
                  >{$t("settingsWindow.shareFeatureUsage")}</Label
                >
                <p class="mt-1 text-xs leading-relaxed text-muted-foreground">
                  {$t("settingsWindow.offByDefaultShareWhichResourceViews")}
                </p>
              </div>
            </div>
            <p class="text-xs leading-relaxed text-muted-foreground">
              {$t("settingsWindow.neverSendsProcessNamesCommandsSearchText")}
            </p>
            <Button
              href="https://abdenasser.github.io/neohtop/privacy.html"
              target="_blank"
              rel="noreferrer"
              variant="outline"
              size="sm"
              class="self-start"
              >{$t("settingsWindow.readThePrivacyNotice")}</Button
            >
          </Card.Content></Card.Root
        >
      </div>
    {:else}
      <h1 class="text-lg font-semibold tracking-tight">
        {$t("settingsWindow.appearance")}
      </h1>
      <p class="mt-1 mb-5 text-sm text-muted-foreground">
        {$t("settingsWindow.allYourNeohtopThemesAcrossEveryWindow")}
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
          }}>{$t("settingsWindow.retry")}</Button
        >
      </div>
    {:else}<p role="status" class="text-xs text-muted-foreground">
        {!$settingsStatus.ready
          ? $t("settingsWindow.loadingSettings")
          : $settingsStatus.saving
            ? $t("settingsWindow.savingChanges")
            : $t("settingsWindow.changesAreSavedAutomatically")}
      </p>{/if}
  </footer>
</div>
