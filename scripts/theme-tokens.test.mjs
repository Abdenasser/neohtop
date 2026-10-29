import assert from "node:assert/strict";
import test from "node:test";
import { readFileSync } from "node:fs";
import vm from "node:vm";
import { matchesTheme } from "../src/lib/utils/theme-search.js";
import {
  themeTokens,
  themeIsDark,
  luminance,
  contrast,
  composite,
} from "../src/lib/utils/theme-tokens.js";
import { isNewerVersion } from "../src/lib/utils/app-version.js";
const source = readFileSync(
  new URL("../src/lib/definitions/themes.ts", import.meta.url),
  "utf8",
);
const themes = vm.runInNewContext(
  "(" +
    source
      .slice(source.indexOf("= {") + 2)
      .trim()
      .replace(/;$/, "") +
    ")",
);
const required = [
  "background",
  "foreground",
  "card",
  "card-foreground",
  "popover",
  "popover-foreground",
  "primary",
  "primary-foreground",
  "secondary",
  "secondary-foreground",
  "muted",
  "muted-foreground",
  "accent",
  "accent-foreground",
  "destructive",
  "destructive-foreground",
  "border",
  "input",
  "ring",
  "chart-1",
  "chart-2",
  "chart-3",
  "chart-4",
  "chart-5",
  "status-running",
  "status-stopped",
  "status-idle",
  "scrollbar-thumb",
  "scrollbar-thumb-hover",
  "radius",
];
function atLeast(fg, bg, min, label) {
  assert.ok(
    contrast(fg, bg) >= min,
    `${label}: ${contrast(fg, bg).toFixed(3)} < ${min}`,
  );
}
for (const [name, theme] of Object.entries(themes))
  test(`${theme.label}: surfaces, controls, actions, charts and focus`, () => {
    const before = JSON.stringify(theme);
    const t = themeTokens(theme.colors, { name });
    assert.equal(JSON.stringify(theme), before, "source palette is immutable");
    assert.deepEqual(
      Object.keys(t).sort(),
      required.toSorted ? required.toSorted() : [...required].sort(),
    );
    assert.equal(
      t.background,
      composite(theme.colors.base, "#ffffff"),
      "preserve canvas",
    );
    for (const [key, color] of Object.entries(t))
      if (key !== "radius")
        assert.match(color, /^#[\da-f]{6}$/i, `${key} must be opaque`);
    const anchors = [t.background, t.card, t.popover],
      planes = [...anchors, t.secondary, t.muted, t.accent];
    if (name === "glassy")
      planes.push(
        ...["#ffffff", "#000000"].map((c) => composite(t.background, c, 0.85)),
      );
    const painted = [
      ...planes,
      ...anchors.map((bg) => composite(t.primary, bg, 0.06)),
    ];
    for (const bg of painted) {
      for (const fg of [
        "foreground",
        "card-foreground",
        "popover-foreground",
        "secondary-foreground",
        "accent-foreground",
        "muted-foreground",
      ])
        atLeast(t[fg], bg, 4.5, `${fg} on ${bg}`);
      atLeast(t.destructive, bg, 4.5, "destructive text");
      for (const opacity of [0.1, 0.2])
        atLeast(
          t.destructive,
          composite(t.destructive, bg, opacity),
          4.5,
          "destructive tinted/hover action",
        );
    }
    for (const bg of planes) {
      atLeast(t.primary, bg, 4.5, "links");
      atLeast(t.primary, composite(t.primary, bg, 0.1), 4.5, "tinted action");
      atLeast(
        t["primary-foreground"],
        composite(t.primary, bg, 0.8),
        4.5,
        "primary hover label",
      );
      atLeast(
        composite(t.ring, bg, 0.5),
        bg,
        3,
        "actual half-opacity focus ring",
      );
      atLeast(t.border, bg, name === "highContrast" ? 3 : 1.3, "shared edges");
      for (const n of [1, 2, 3, 4, 5]) {
        atLeast(t[`chart-${n}`], bg, 3, "chart stroke");
        atLeast(
          t[`chart-${n}`],
          composite(t[`chart-${n}`], bg, 0.1),
          3,
          "resource icon tint",
        );
      }
      for (const key of [
        "status-running",
        "status-stopped",
        "status-idle",
        "scrollbar-thumb",
      ])
        atLeast(t[key], bg, 3, key);
      atLeast(
        t["muted-foreground"],
        composite(t.foreground, bg, 0.05),
        4.5,
        "secondary-button hover text",
      );
      if (name === "highContrast")
        atLeast(t.foreground, bg, 7, "high-contrast main text");
    }
    const fields =
      name === "glassy"
        ? [
            ...anchors,
            ...["#ffffff", "#000000"].map((c) =>
              composite(t.card, composite(t.background, c, 0.85), 0.78),
            ),
          ]
        : anchors;
    for (const bg of fields) {
      atLeast(t.input, bg, name === "highContrast" ? 4.5 : 3, "field outline");
      if (themeIsDark(theme.colors)) {
        atLeast(
          t.foreground,
          composite(t.input, bg, 0.3),
          4.5,
          "dark field value",
        );
        atLeast(
          t.foreground,
          composite(t.input, bg, 0.5),
          4.5,
          "dark select hover value",
        );
        atLeast(
          t["muted-foreground"],
          composite(t.input, bg, 0.3),
          4.5,
          "dark field placeholder",
        );
      }
    }
    atLeast(t["primary-foreground"], t.primary, 4.5, "primary label");
    atLeast(
      t["destructive-foreground"],
      t.destructive,
      4.5,
      "destructive fill label",
    );
  });
test("all 24 saved identifiers and intended modes remain", () => {
  assert.equal(Object.keys(themes).length, 24);
  for (const [name, t] of Object.entries(themes)) assert.equal(t.name, name);
  for (const name of [
    "githubLight",
    "solarizedLight",
    "oneLight",
    "ayuLight",
    "bubblegum",
    "cottonCandy",
    "candyfloss",
  ])
    assert.equal(themeIsDark(themes[name].colors), false, name);
  for (const name of [
    "catppuccin",
    "dracula",
    "terminal",
    "amber",
    "ibmPC",
    "highContrast",
    "glassy",
  ])
    assert.equal(themeIsDark(themes[name].colors), true, name);
});
test("contrast math accounts for alpha and known luminance vectors", () => {
  assert.equal(composite("#8A91991A", "#fcfcfc"), "#f0f1f2");
  assert.equal(composite("#00000080", "#ffffff"), "#7f7f7f");
  assert.equal(composite("#f008", "#fff"), "#ff7777");
  assert.equal(contrast("#000", "#fff"), 21);
  assert.ok(Math.abs(contrast("#777777", "#ffffff") - 4.478) < 0.002);
  assert.ok(Math.abs(contrast("#00000080", "#ffffff") - 4.004) < 0.002);
  assert.equal(luminance("#000"), 0);
  assert.equal(luminance("#fff"), 1);
  assert.throws(() => contrast("invalid", "#fff"), TypeError);
});
test("Ayu Light panels and every popup are opaque, stable surfaces", () => {
  const t = themeTokens(themes.ayuLight.colors, { name: "ayuLight" });
  assert.equal(t.card, "#f0f1f2");
  for (const theme of Object.values(themes)) {
    const p = themeTokens(theme.colors, { name: theme.name });
    assert.equal(
      composite(p.popover, "#000000"),
      composite(p.popover, "#ffffff"),
    );
  }
});
test("bootstrap CSS matches the default palette semantic roles", () => {
  const css = readFileSync(new URL("../src/app.css", import.meta.url), "utf8");
  const root = css.slice(
    css.indexOf(":root {"),
    css.indexOf("\n}", css.indexOf(":root {")),
  );
  const tokens = themeTokens(themes.catppuccin.colors, { name: "catppuccin" });
  for (const [name, value] of Object.entries(tokens))
    assert.ok(root.includes(`--${name}: ${value};`), name);
  for (const n of [1, 2, 3, 4, 5])
    assert.ok(css.includes(`--color-chart-${n}: var(--chart-${n});`));
});
test("retro corner tokens and high contrast remain distinct from ordinary themes", () => {
  for (const name of ["terminal", "amber", "ibmPC"])
    assert.equal(themeTokens(themes[name].colors, { name }).radius, "0.25rem");
  assert.equal(
    themeTokens(themes.highContrast.colors, { name: "highContrast" }).radius,
    "0.375rem",
  );
  assert.equal(
    themeTokens(themes.catppuccin.colors, { name: "catppuccin" }).radius,
    "0.625rem",
  );
});
test("release notice does not offer a downgrade as an update", () => {
  assert.equal(isNewerVersion("macos-nightly-1.1.0", "1.2.0"), false);
  assert.equal(isNewerVersion("1.2.0", "1.2.0"), false);
  assert.equal(isNewerVersion("1.10.0", "1.9.9"), true);
  assert.equal(isNewerVersion("2.0.0", "1.99.99"), true);
  assert.equal(isNewerVersion("invalid", "1.2.0"), false);
  assert.equal(isNewerVersion("1.3.0", ""), false);
});

test("theme search folds accents, case and keyboard whitespace", () => {
  assert.equal(matchesTheme("Rosé Pine", "rose"), true);
  assert.equal(matchesTheme("Rosé Pine", "  ROSE   pine "), true);
  assert.equal(matchesTheme("GitHub Light", "github light"), true);
  assert.equal(matchesTheme("Catppuccin", "   "), true);
  assert.equal(matchesTheme("Solarized Dark", "Light"), false);
});
