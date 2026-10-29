import assert from "node:assert/strict";
import test from "node:test";
import {
  networkCeiling,
  nearestResourceSample,
} from "../src/lib/utils/resource-history.js";
test("network scale covers both independent rates and leaves unknowns unknown", () => {
  const history = [
    { timestamp: 10000, received: 100, sent: 20 },
    { timestamp: 13000, received: null, sent: 19000 },
  ];
  assert.equal(networkCeiling(history), 20000);
  assert.equal(networkCeiling([]), 2000);
  assert.equal(nearestResourceSample(history, 12500), history[1]);
  assert.equal(nearestResourceSample(history, 10000), history[0]);
  assert.equal(nearestResourceSample([], 0), undefined);
});
