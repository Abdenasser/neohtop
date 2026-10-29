// Send only changed leaves: one window must not overwrite another window's preferences.
/** @param {any} previous @param {any} next @returns {Record<string, any>} */
export function changedLeaves(previous, next) {
  /** @type {Record<string, any>} */
  const patch = {};
  for (const [key, value] of Object.entries(next)) {
    if (value && typeof value === "object" && !Array.isArray(value)) {
      const nested = changedLeaves(previous?.[key] || {}, value);
      if (Object.keys(nested).length) patch[key] = nested;
    } else if (previous?.[key] !== value) patch[key] = value;
  }
  return patch;
}
/** @param {any} base @param {any} patch @returns {any} */
export function mergePreferences(base, patch) {
  const next = { ...base };
  for (const [key, value] of Object.entries(patch)) {
    next[key] =
      value && typeof value === "object" && !Array.isArray(value)
        ? mergePreferences(base[key] || {}, value)
        : value;
  }
  return next;
}
