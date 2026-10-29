/**
 * Format a usage ratio for display, marking unavailable totals as N/A.
 *
 * @param {number} used
 * @param {number} total
 * @returns {string}
 */
export function formatUsagePercentage(used, total) {
  if (total === 0) return "N/A";
  return `${((used / total) * 100).toFixed(1)}%`;
}
