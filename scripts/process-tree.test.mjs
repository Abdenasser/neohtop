import assert from "node:assert/strict";
import test from "node:test";
import {
  buildProcessTree,
  paginateProcessTree,
  processIdentity,
} from "../src/lib/utils/process-tree.js";

function process(
  pid,
  ppid = 0,
  start_time = pid,
  cpu_usage = 0,
  memory_usage = 0,
) {
  return {
    pid,
    ppid,
    start_time,
    name: `process-${pid}`,
    command: `process-${pid}`,
    cpu_usage,
    memory_usage,
  };
}
const ids = (tree) => tree.rows.map((row) => row.process.pid);

test("families start collapsed and expanding preserves sorted sibling order", () => {
  const processes = [process(3, 1), process(1), process(2, 1), process(4)];
  assert.deepEqual(ids(buildProcessTree(processes, new Set())), [1, 4]);
  const expanded = new Set([processIdentity(processes[1])]);
  const tree = buildProcessTree(processes, expanded);
  assert.deepEqual(ids(tree), [1, 3, 2, 4]);
  assert.deepEqual(
    tree.rows.map((row) => row.depth),
    [0, 1, 1, 0],
  );
});

test("filtering a child retains and expands its ancestor chain", () => {
  const tree = buildProcessTree(
    [process(1), process(2, 1), process(3, 2), process(4, 1)],
    new Set(),
    new Set([3]),
  );
  assert.deepEqual(ids(tree), [1, 2, 3]);
  assert.equal(tree.rows[0].expanded, true);
  assert.equal(tree.rows[1].expanded, true);
  assert.deepEqual(
    ids(buildProcessTree([process(1)], new Set(), new Set())),
    [],
  );
});

test("orphaned processes and reused parent PIDs remain visible as roots", () => {
  const tree = buildProcessTree(
    [process(1, 99), process(2, 3, 10), process(3, 0, 20)],
    new Set(),
  );
  assert.deepEqual(ids(tree), [1, 2, 3]);
  // A new process with the same PID does not inherit the old expansion state.
  const expanded = new Set([processIdentity(process(3, 0, 1))]);
  assert.deepEqual(
    ids(buildProcessTree([process(3, 0, 20), process(4, 3, 30)], expanded)),
    [3],
  );
});

test("cyclic and self-parented snapshots never lose processes or loop forever", () => {
  const processes = [
    process(1, 2, 0),
    process(2, 3, 0),
    process(3, 1, 0),
    process(4, 4, 0),
  ];
  const expanded = new Set(processes.map(processIdentity));
  const tree = buildProcessTree(processes, expanded);
  assert.deepEqual([...ids(tree)].sort(), [1, 2, 3, 4]);
  assert.equal(new Set(ids(tree)).size, 4);
});

test("root pagination never splits expanded families and clamps disappearing pages", () => {
  const processes = [process(1), process(2, 1), process(3, 1), process(4)];
  const tree = buildProcessTree(
    processes,
    new Set([processIdentity(processes[0])]),
  );
  assert.deepEqual(ids(paginateProcessTree(tree, 1, 1)), [1, 2, 3]);
  const last = paginateProcessTree(tree, 99, 1);
  assert.deepEqual(ids(last), [4]);
  assert.equal(last.currentPage, 2);
  assert.equal(
    paginateProcessTree({ rows: [], rootCount: 0 }, 3, 15).totalPages,
    1,
  );
});

test("deep process chains are traversed without recursion", () => {
  const processes = Array.from({ length: 12000 }, (_, i) => process(i + 1, i));
  const tree = buildProcessTree(
    processes,
    new Set(processes.map(processIdentity)),
  );
  assert.equal(tree.rows.length, processes.length);
  assert.equal(tree.rows.at(-1).depth, 11999);
});

test("family totals include hidden descendants and preserve multi-core CPU values", () => {
  const processes = [
    process(1, 0, 1, 1, 100),
    process(2, 1, 2, 130, 200),
    process(3, 2, 3, 10, 50),
    process(4, 1, 4, 20, 25),
  ];
  const tree = buildProcessTree(processes, new Set(), new Set([3]));
  assert.deepEqual(ids(tree), [1, 2, 3]);
  assert.equal(tree.rows[0].familyCpuUsage, 161);
  assert.equal(tree.rows[0].familyMemoryUsage, 375);
  assert.equal(tree.rows[0].familyProcessCount, 4);
  assert.equal(tree.rows[1].familyCpuUsage, 140);
  assert.equal(tree.rows[1].familyProcessCount, 2);
});

test("sorting and pinning prioritize whole families without mutating snapshots", () => {
  const processes = [
    process(4, 0, 4, 50, 1000),
    process(1, 0, 1, 0, 100),
    process(2, 1, 2, 90, 200),
  ];
  const before = JSON.stringify(processes);
  const descending = { sortConfig: { field: "cpu_usage", direction: "desc" } };
  assert.deepEqual(
    ids(buildProcessTree(processes, new Set(), null, descending)),
    [1, 4],
  );
  const ascending = { sortConfig: { field: "memory_usage", direction: "asc" } };
  assert.deepEqual(
    ids(buildProcessTree(processes, new Set(), null, ascending)),
    [1, 4],
  );
  const pinned = {
    sortConfig: { field: "cpu_usage", direction: "asc" },
    pinnedProcesses: new Set(["process-2"]),
  };
  assert.deepEqual(
    ids(buildProcessTree(processes, new Set(), null, pinned)),
    [1, 4],
  );
  assert.equal(JSON.stringify(processes), before);
});
