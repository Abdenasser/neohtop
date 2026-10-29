/** Compare the numeric release triplet, including prefixed platform tags.
 * @param {string} latest @param {string} current
 */
export function isNewerVersion(latest, current) {
  const a = latest
    .match(/\d+\.\d+\.\d+/)?.[0]
    .split(".")
    .map(Number);
  const b = current
    .match(/\d+\.\d+\.\d+/)?.[0]
    .split(".")
    .map(Number);
  if (!a || !b) return false;
  for (let i = 0; i < 3; i++) {
    if (a[i] !== b[i]) return a[i] > b[i];
  }
  return false;
}
