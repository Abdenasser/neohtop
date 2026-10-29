import test from "node:test";
import assert from "node:assert/strict";
import {
  changedLeaves,
  mergePreferences,
} from "../src/lib/utils/settings-patch.js";
test("settings patches carry only edited leaves from a full settings group", () => {
  const base = {
    behavior: { refreshRate: 3000, processView: "flat" },
    appearance: { columnVisibility: { pid: true, name: true } },
  };
  const patch = changedLeaves(base, {
    behavior: { ...base.behavior, processView: "tree" },
  });
  assert.deepEqual(patch, { behavior: { processView: "tree" } });
  const otherWindow = mergePreferences(base, {
    behavior: { refreshRate: 5000 },
  });
  assert.deepEqual(mergePreferences(otherWindow, patch).behavior, {
    refreshRate: 5000,
    processView: "tree",
  });
  const columns = changedLeaves(base, {
    appearance: { columnVisibility: { pid: false, name: true } },
  });
  assert.deepEqual(columns, {
    appearance: { columnVisibility: { pid: false } },
  });
  assert.deepEqual(changedLeaves(base, base), {});
});
