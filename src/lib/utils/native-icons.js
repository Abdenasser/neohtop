/** Batches visible icons, shares pending lookups, and bounds positive/negative caching.
 * @param {(requests:{pid:number,start_time:number}[])=>Promise<{pid:number,start_time:number,icon:string|null}[]>} fetchIcons
 * @param {()=>number} [now]
 * @param {number} [limit]
 */
export function createNativeIconLoader(
  fetchIcons,
  now = Date.now,
  limit = 256,
) {
  /** @type {Map<string,{icon:string|null,expires:number}>} */
  const cache = new Map();
  /** @type {Map<string,{request:{pid:number,start_time:number},resolve:(icon:string|null)=>void,promise:Promise<string|null>}>} */
  const pending = new Map();
  /** @type {string[]} */
  const queue = [];
  let scheduled = false,
    running = false;
  const key = (/** @type {number} */ pid, /** @type {number} */ start) =>
    `${pid}:${start}`;
  function schedule() {
    if (!scheduled && !running) {
      scheduled = true;
      queueMicrotask(flush);
    }
  }
  async function flush() {
    scheduled = false;
    if (running || !queue.length) return;
    running = true;
    const keys = queue.splice(0, 32);
    const entries = keys.map((k) => pending.get(k));
    let failed = false;
    /** @type {Map<string,string|null>} */
    let results = new Map();
    try {
      const rows = await fetchIcons(
        entries.flatMap((e) => (e ? [e.request] : [])),
      );
      results = new Map(
        rows.map((row) => [
          key(row.pid, row.start_time),
          validIcon(row.icon) ? row.icon : null,
        ]),
      );
    } catch {
      failed = true;
    }
    for (const k of keys) {
      const entry = pending.get(k);
      if (!entry) continue;
      const icon = results.get(k) ?? null;
      cache.delete(k);
      cache.set(k, {
        icon,
        expires: now() + (failed ? 5000 : icon ? 300000 : 60000),
      });
      while (cache.size > limit) {
        const first = cache.keys().next().value;
        if (first === undefined) break;
        cache.delete(first);
      }
      pending.delete(k);
      entry.resolve(icon);
    }
    running = false;
    if (queue.length) schedule();
  }
  return (/** @type {number} */ pid, /** @type {number} */ startTime) => {
    if (
      !Number.isSafeInteger(pid) ||
      pid <= 0 ||
      !Number.isSafeInteger(startTime) ||
      startTime < 0
    )
      return Promise.resolve(null);
    const k = key(pid, startTime),
      hit = cache.get(k);
    if (hit && hit.expires > now()) {
      cache.delete(k);
      cache.set(k, hit);
      return Promise.resolve(hit.icon);
    }
    cache.delete(k);
    const waiting = pending.get(k);
    if (waiting) return waiting.promise;
    // Pages may change while I/O is pending. Bound the backlog as well as the cache.
    if (pending.size >= 256) return Promise.resolve(null);
    /** @type {(icon:string|null)=>void} */
    let resolve = () => {};
    const promise = new Promise(
      /** @param {(icon:string|null)=>void} done */ (done) => {
        resolve = done;
      },
    );
    pending.set(k, {
      request: { pid, start_time: startTime },
      resolve,
      promise,
    });
    queue.push(k);
    schedule();
    return promise;
  };
}
/** Native IPC returns images only, never a file URL or arbitrary network URL.
 * @param {unknown} value
 * @returns {value is string}
 */
function validIcon(value) {
  return (
    typeof value === "string" &&
    value.length <= 180000 &&
    /^data:image\/(png|svg\+xml);base64,[A-Za-z0-9+/=]+$/.test(value)
  );
}
