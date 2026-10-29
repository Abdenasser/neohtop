import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import ts from "typescript";
const source = await readFile(
  new URL("../src/lib/utils/index.ts", import.meta.url),
  "utf8",
);
const javascript = ts.transpileModule(source, {
  compilerOptions: {
    module: ts.ModuleKind.ES2022,
    target: ts.ScriptTarget.ES2022,
  },
}).outputText;
const { parsePortTerm, searchHasPortQuery, filterProcesses } = await import(
  `data:text/javascript;base64,${Buffer.from(javascript).toString("base64")}`
);
const filters = () => ({
  cpu: { enabled: false },
  gpu: { enabled: false },
  ram: { enabled: false },
  runtime: { enabled: false },
  status: { enabled: false, values: [] },
});
const processes = [
  {
    pid: 10,
    name: "node",
    command: "node server.js",
    ports: [3000],
    gpu_usage: 12,
  },
  { pid: 3000, name: "worker", command: "worker", ports: [], gpu_usage: null },
  { pid: 30, name: "idle", command: "idle", ports: [443], gpu_usage: 0 },
];
test("port syntax validates ranges and keeps bare numbers as process IDs", () => {
  for (const [term, expected] of [
    [":3000", 3000],
    ["port:443", 443],
    [" PORT:65535 ", 65535],
    ["3000", null],
    [":0", null],
    [":65536", null],
    ["port:no", null],
  ])
    assert.equal(parsePortTerm(term), expected);
  assert.equal(searchHasPortQuery("worker, port:3000"), true);
  assert.equal(searchHasPortQuery("3000, worker"), false);
  assert.deepEqual(
    filterProcesses(processes, ":3000", filters()).map((p) => p.pid),
    [10],
  );
  assert.deepEqual(
    filterProcesses(processes, "3000", filters()).map((p) => p.pid),
    [3000],
  );
  assert.deepEqual(
    filterProcesses(processes, "port:443, worker$", filters()).map(
      (p) => p.pid,
    ),
    [3000, 30],
  );
});
test("GPU filters distinguish unknown readings from actual zero activity", () => {
  const low = filters();
  low.gpu = { enabled: true, operator: "<", value: 1 };
  assert.deepEqual(
    filterProcesses(processes, "", low).map((p) => p.pid),
    [30],
  );
  const high = filters();
  high.gpu = { enabled: true, operator: ">", value: 1 };
  assert.deepEqual(
    filterProcesses(processes, "", high).map((p) => p.pid),
    [10],
  );
});
