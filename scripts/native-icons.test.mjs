import test from "node:test";
import assert from "node:assert/strict";
import { createNativeIconLoader } from "../src/lib/utils/native-icons.js";
const image = "data:image/png;base64,aW1hZ2U=";
const reply = (requests) => requests.map((r) => ({ ...r, icon: image }));
test("visible icon requests share pending work and batch with a bounded backlog", async () => {
  let batches = [],
    active = 0,
    peak = 0;
  const load = createNativeIconLoader(async (requests) => {
    batches.push(requests);
    peak = Math.max(peak, ++active);
    await Promise.resolve();
    active--;
    return reply(requests);
  });
  const first = load(1, 100);
  assert.equal(first, load(1, 100));
  const result = await Promise.all([
    first,
    ...Array.from({ length: 299 }, (_, i) => load(i + 2, 100)),
  ]);
  assert.equal(peak, 1);
  assert.equal(batches.length, 8);
  assert.ok(batches.every((b) => b.length <= 32));
  assert.equal(result.filter(Boolean).length, 256);
});
test("known and missing icons are cached, then may be retried after expiry", async () => {
  let now = 0,
    calls = 0;
  const load = createNativeIconLoader(
    async (requests) => {
      calls++;
      return requests.map((r) => ({ ...r, icon: r.pid === 1 ? image : null }));
    },
    () => now,
  );
  assert.equal(await load(1, 10), image);
  assert.equal(await load(2, 10), null);
  await load(1, 10);
  await load(2, 10);
  assert.equal(calls, 2);
  now = 60001;
  await load(2, 10);
  await load(1, 10);
  assert.equal(calls, 3);
  now = 300001;
  await load(1, 10);
  assert.equal(calls, 4);
});
test("transient IPC errors settle callers and retry instead of poisoning the cache", async () => {
  let now = 0,
    calls = 0;
  const load = createNativeIconLoader(
    async (requests) => {
      if (++calls === 1) throw Error("unavailable");
      return reply(requests);
    },
    () => now,
  );
  assert.equal(await load(1, 1), null);
  assert.equal(await load(1, 1), null);
  assert.equal(calls, 1);
  now = 5001;
  assert.equal(await load(1, 1), image);
  assert.equal(calls, 2);
});
test("reused process IDs get independent identities and invalid IDs never reach IPC", async () => {
  let requests = [];
  const load = createNativeIconLoader(async (rows) => {
    requests.push(...rows);
    return reply(rows);
  });
  await load(1, 100);
  await load(1, 200);
  assert.equal(requests.length, 2);
  for (const args of [
    [0, 1],
    [-1, 1],
    [NaN, 1],
    [1, -1],
    [1, Infinity],
    [1.5, 2],
  ])
    assert.equal(await load(...args), null);
  assert.equal(requests.length, 2);
});
test("the cache evicts the least recently used process identity", async () => {
  let calls = 0;
  const load = createNativeIconLoader(
    async (requests) => {
      calls++;
      return reply(requests);
    },
    Date.now,
    2,
  );
  await load(1, 1);
  await load(2, 1);
  await load(1, 1);
  await load(3, 1);
  await load(1, 1);
  assert.equal(calls, 3);
  await load(2, 1);
  assert.equal(calls, 4);
});
test("file, network, malformed and oversized icon payloads fall back locally", async () => {
  const invalid = [
    "file:///private/file",
    "https://example.com/icon.png",
    "data:text/html;base64,abcd",
    "data:image/png;base64,%%%",
    "data:image/png;base64," + "A".repeat(180000),
  ];
  const load = createNativeIconLoader(async (requests) =>
    requests.map((r, i) => ({ ...r, icon: invalid[i] })),
  );
  assert.deepEqual(
    await Promise.all(invalid.map((_, i) => load(i + 1, 1))),
    invalid.map(() => null),
  );
});
