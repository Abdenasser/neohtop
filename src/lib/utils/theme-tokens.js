/** Parse the palette's RGB/RGBA hex colors, retaining alpha until compositing.
 * @param {string} hex
 */
function rgba(hex) {
  const value = hex.replace(/^#/, "");
  if (!/^(?:[\da-f]{3,4}|[\da-f]{6}|[\da-f]{8})$/i.test(value))
    throw new TypeError(`Invalid palette color: ${hex}`);
  const full =
    value.length <= 4 ? [...value].map((c) => c + c).join("") : value;
  return {
    rgb: [0, 2, 4].map((i) => Number.parseInt(full.slice(i, i + 2), 16)),
    alpha: full.length === 8 ? Number.parseInt(full.slice(6, 8), 16) / 255 : 1,
  };
}
/** @param {number[]} rgb */
function hex(rgb) {
  return (
    "#" + rgb.map((c) => Math.round(c).toString(16).padStart(2, "0")).join("")
  );
}
/** Composite a color/opacity over a known surface, using CSS sRGB alpha behavior.
 * @param {string} foreground @param {string} background @param {number} [opacity]
 */
export function composite(foreground, background, opacity = 1) {
  const front = rgba(foreground),
    back = rgba(background);
  const alpha = front.alpha * opacity;
  const canvas = back.rgb.map((c) => c * back.alpha + 255 * (1 - back.alpha));
  return hex(front.rgb.map((c, i) => c * alpha + canvas[i] * (1 - alpha)));
}
/** @param {string} color */
export function luminance(color) {
  const rgb = rgba(composite(color, "#ffffff")).rgb.map((c) => {
    const s = c / 255;
    return s <= 0.04045 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
  });
  return 0.2126 * rgb[0] + 0.7152 * rgb[1] + 0.0722 * rgb[2];
}
/** Alpha in a foreground is resolved over its background; a translucent background
 * needs a canvas, defaulting to white. Theme surfaces below are always opaque.
 * @param {string} foreground @param {string} background @param {string} [canvas]
 */
export function contrast(foreground, background, canvas = "#ffffff") {
  const back = composite(background, canvas),
    a = luminance(composite(foreground, back)),
    b = luminance(back);
  return (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05);
}
/** @param {import('../types').Theme['colors']} colors */
export function themeIsDark(colors) {
  return luminance(colors.base) < 0.45;
}
/** Find the smallest sRGB adjustment toward black or white that satisfies all
 * actual component surfaces/states. This keeps the original hue where possible.
 * @param {string} color @param {(candidate:string)=>boolean} accepts
 */
function fit(color, accepts) {
  const source = composite(color, "#ffffff");
  for (let step = 0; step <= 100; step++)
    for (const target of ["#ffffff", "#000000"]) {
      const candidate = composite(target, source, step / 100);
      if (accepts(candidate)) return candidate;
    }
  throw new Error(`Palette role cannot be resolved from ${color}`);
}
/** @param {string} color @param {string[]} surfaces @param {number} minimum @param {number[]} [washes] */
function readable(color, surfaces, minimum = 4.5, washes = [0]) {
  return fit(color, (candidate) =>
    surfaces.every((surface) =>
      washes.every(
        (alpha) =>
          contrast(candidate, composite(candidate, surface, alpha)) >=
          minimum + 0.05,
      ),
    ),
  );
}
/** Resolve label ink for the stock opaque and /80-hover primary button states.
 * @param {string} fill @param {string[]} surfaces
 */
function actionInk(fill, surfaces) {
  return ["#11111b", "#ffffff", "#000000"].find((ink) =>
    [fill, ...surfaces.map((s) => composite(fill, s, 0.8))].every(
      (s) => contrast(ink, s) >= 4.55,
    ),
  );
}
/** @param {string} seed @param {string[]} surfaces */
function action(seed, surfaces) {
  const fill = fit(
    seed,
    (c) =>
      surfaces.every(
        (s) =>
          contrast(c, s) >= 4.55 && contrast(c, composite(c, s, 0.1)) >= 4.55,
      ) && !!actionInk(c, surfaces),
  );
  return { fill, ink: actionInk(fill, surfaces) ?? "#ffffff" };
}
/** Full semantic shadcn roles, derived from the original community palette.
 * No palette definitions or saved theme identifiers are rewritten.
 * @param {import('../types').Theme['colors']} colors
 * @param {{name?:string}} [options]
 * @returns {Record<string,string>}
 */
export function themeTokens(colors, { name = "" } = {}) {
  const high = name === "highContrast",
    glassy = name === "glassy";
  const dark = themeIsDark(colors),
    background = composite(colors.base, "#ffffff");
  // Legacy RGBA (notably Ayu Light's mantle) becomes a stable, opaque surface.
  const card = composite(colors.mantle, background);
  const popover = dark
    ? composite(colors.text, card, 0.04)
    : composite("#ffffff", background, 0.6);
  const anchors = [background, card, popover];
  const secondary = fit(composite(colors.surface0, card), (c) =>
    anchors.every((s) => contrast(c, s) >= 1.12),
  );
  const muted = composite(secondary, card, 0.7);
  const accent = composite(colors.blue, secondary, 0.08);
  const planes = [...anchors, secondary, muted, accent];
  // The glass backdrop remains visible through a tinted application layer. Include
  // both extreme backdrop colors when resolving text, strokes and focus roles.
  if (glassy)
    planes.push(
      composite(background, "#ffffff", 0.85),
      composite(background, "#000000", 0.85),
    );
  const fieldCanvases = glassy
    ? [
        ...anchors,
        ...["#ffffff", "#000000"].map((c) =>
          composite(card, composite(background, c, 0.85), 0.78),
        ),
      ]
    : anchors;
  const input = readable(colors.surface1, fieldCanvases, high ? 4.5 : 3);
  const fields = dark
    ? fieldCanvases.flatMap((s) => [
        composite(input, s, 0.3),
        composite(input, s, 0.5),
      ])
    : [];
  const primary = action(colors.blue, planes);
  const painted = [
    ...planes,
    ...anchors.map((s) => composite(primary.fill, s, 0.06)),
  ];
  const foreground = fit(
    colors.text,
    (c) =>
      [...painted, ...fields].every((s) => contrast(c, s) >= 4.55) &&
      contrast(c, composite(c, secondary, 0.06)) >= 4.55 &&
      (!high || planes.every((s) => contrast(c, s) >= 7)),
  );
  const mutedForeground = fit(
    colors.subtext0,
    (c) =>
      [
        ...painted,
        ...(dark ? fieldCanvases.map((s) => composite(input, s, 0.3)) : []),
      ].every((s) => contrast(c, s) >= 4.55) &&
      planes.every(
        (s) => contrast(c, composite(foreground, s, 0.05)) >= 4.55,
      ) &&
      (!high || planes.every((s) => contrast(c, s) >= 7)),
  );
  const destructive = readable(colors.red, painted, 4.5, [0, 0.1, 0.2]);
  const ring = fit(colors.blue, (c) =>
    planes.every((s) => contrast(composite(c, s, 0.5), s) >= 3.05),
  );
  const tokens = {
    background,
    foreground,
    card,
    "card-foreground": foreground,
    popover,
    "popover-foreground": foreground,
    primary: primary.fill,
    "primary-foreground": primary.ink,
    secondary,
    "secondary-foreground": foreground,
    muted,
    "muted-foreground": mutedForeground,
    accent,
    "accent-foreground": foreground,
    destructive,
    "destructive-foreground":
      actionInk(destructive, planes) ?? readable(colors.text, [destructive]),
    border: readable(colors.surface0, planes, high ? 3 : 1.3),
    input,
    ring,
    "chart-1": readable(colors.blue, planes, 3, [0, 0.1]),
    "chart-2": readable(colors.green, planes, 3, [0, 0.1]),
    "chart-3": readable(colors.peach, planes, 3, [0, 0.1]),
    "chart-4": readable(colors.lavender, planes, 3, [0, 0.1]),
    "chart-5": readable(colors.teal, planes, 3, [0, 0.1]),
    "status-running": readable(colors.green, planes, 3),
    "status-stopped": readable(colors.red, planes, 3),
    "status-idle": readable(colors.subtext0, planes, 3),
    "scrollbar-thumb": readable(colors.surface2, planes, 3),
    "scrollbar-thumb-hover": readable(colors.surface2, planes, 4.5),
    radius: ["terminal", "amber", "ibmPC"].includes(name)
      ? "0.25rem"
      : high
        ? "0.375rem"
        : "0.625rem",
  };
  return tokens;
}
