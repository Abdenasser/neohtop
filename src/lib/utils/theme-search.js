/** Theme labels should match ordinary unaccented keyboard searches.
 * @param {string} label @param {string} query
 */
export function matchesTheme(label, query) {
  const fold = (/** @type {string} */ value) =>
    value
      .normalize("NFKD")
      .replace(/\p{Diacritic}/gu, "")
      .toLowerCase()
      .trim()
      .replace(/\s+/g, " ");
  return fold(label).includes(fold(query));
}
