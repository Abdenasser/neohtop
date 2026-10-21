import assert from "node:assert/strict";
import { formatUsagePercentage } from "../src/lib/utils/metrics.js";

// Synthetic monitor values only; this test does not read host metrics.
assert.equal(formatUsagePercentage(0, 0), "N/A");
assert.equal(formatUsagePercentage(3, 4), "75.0%");
assert.equal(formatUsagePercentage(1, 3), "33.3%");
assert.equal(formatUsagePercentage(0, 4), "0.0%");

console.log(
  "PASS: zero-capacity usage is unavailable; positive totals format normally",
);
