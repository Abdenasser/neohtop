// Check emitted CSS against the data-state attributes supplied by Bits UI.
// Type checks alone cannot catch a missing shared Tailwind stylesheet.
import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
const root = new URL("../build/_app/immutable/assets/", import.meta.url);
const css = readdirSync(root)
  .filter((name) => name.endsWith(".css"))
  .map((name) => readFileSync(new URL(name, root), "utf8"))
  .join("\n");
for (const [utility, state] of [
  ["data-checked\\:bg-primary", "checked"],
  ["data-open\\:animate-in", "open"],
  ["data-closed\\:animate-out", "closed"],
]) {
  const escaped = utility.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const rule = new RegExp(
    `\\.${escaped}[^{}]*\\[data-state=(?:"${state}"|${state})\\][^{}]*\\{`,
  );
  assert.match(
    css,
    rule,
    `Missing shadcn ${state} state selector. Import shadcn-svelte/tailwind.css.`,
  );
}
console.log("Shadcn checked/open/closed state styles verified.");
