export const RESOURCE_WINDOW_MS = 120_000;
/** One shared zero-based network scale; receive/send are not stacked.
 * @param {import('../types').ResourceSample[]} history
 */
export function networkCeiling(history) {
  const peak = Math.max(
    1024,
    ...history.flatMap((p) => [p.received ?? 0, p.sent ?? 0]),
  );
  const magnitude = 10 ** Math.floor(Math.log10(peak));
  return Math.ceil(peak / magnitude) * magnitude;
}
/** Select the nearest actually collected observation, including explicit gaps.
 * @param {import('../types').ResourceSample[]} history
 * @param {number} timestamp
 */
export function nearestResourceSample(history, timestamp) {
  return history.reduce(
    (best, p) =>
      !best ||
      Math.abs(p.timestamp - timestamp) < Math.abs(best.timestamp - timestamp)
        ? p
        : best,
    /** @type {import('../types').ResourceSample|undefined} */ (undefined),
  );
}
