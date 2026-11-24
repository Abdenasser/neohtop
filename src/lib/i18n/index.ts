import { derived } from "svelte/store";
import { settingsStore } from "$lib/stores/settings";
import en from "./en.json";
import zhCN from "./zh-CN.json";

export type MessageKey = keyof typeof en;
export type Translate = (
  key: MessageKey,
  values?: Record<string, string | number | boolean | null | undefined>,
) => string;

const chinese: Record<MessageKey, string> = zhCN;

export const uiLocale = derived(settingsStore, ($settings) =>
  $settings.behavior.language === "en" ? "en" : "zh-CN",
);

export const t = derived(uiLocale, ($locale): Translate => {
  const messages = $locale === "zh-CN" ? chinese : en;
  const pluralRules = new Intl.PluralRules($locale);
  return (key, values) => {
    const count = values?.count;
    const pluralKey =
      typeof count === "number"
        ? (`${key}.${pluralRules.select(count)}` as MessageKey)
        : key;
    const message =
      messages[pluralKey] ?? messages[key] ?? en[pluralKey] ?? en[key];
    return values
      ? message.replace(/\{(\w+)\}/g, (_, name) => String(values[name] ?? ""))
      : message;
  };
});

const statusKeys: Record<string, MessageKey> = {
  running: "status.running",
  sleeping: "status.sleeping",
  idle: "status.idle",
  stopped: "status.stopped",
  zombie: "status.zombie",
  dead: "status.dead",
  tracing: "status.tracing",
  waking: "status.waking",
  parked: "status.parked",
  lockblocked: "status.lockBlocked",
  uninterruptibledisksleep: "status.uninterruptibleDiskSleep",
  unknown: "status.unknown",
};

export function processStatus(translate: Translate, status: string) {
  const key = statusKeys[status.toLowerCase()];
  return key ? translate(key) : status;
}
