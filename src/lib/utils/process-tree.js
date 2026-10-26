/** @typedef {import('../types').Process} Process */
/** @typedef {import('../types').ProcessTreeRow} ProcessTreeRow */

/** @param {Process} process */
export function processIdentity(process) {
  return `${process.pid}:${process.start_time}`;
}

/**
 * Keep sibling order from the existing sort, retain ancestors of search matches,
 * and tolerate disappearing parents, reused PIDs, and malformed cycles.
 * Traversal is iterative so deep process chains cannot overflow the stack.
 * @param {Process[]} sortedProcesses
 * @param {Set<string>} expanded
 * @param {Set<number> | null} matchingPids null means no active search/filter
 * @param {{sortConfig?: import('../types').SortConfig, pinnedProcesses?: Set<string>}} options
 * @returns {{rows: ProcessTreeRow[], rootCount: number}}
 */
export function buildProcessTree(
  sortedProcesses,
  expanded,
  matchingPids = null,
  options = {},
) {
  const processes = new Map(
    sortedProcesses.map((process) => [process.pid, process]),
  );
  /** @type {Map<number, number>} */
  const parents = new Map();
  for (const process of processes.values()) {
    const parent = processes.get(process.ppid);
    if (
      parent &&
      parent.pid !== process.pid &&
      !(
        parent.start_time > 0 &&
        process.start_time > 0 &&
        parent.start_time > process.start_time
      )
    )
      parents.set(process.pid, parent.pid);
  }

  // Resolve every parent chain once; cut an edge when a cycle is encountered.
  const resolved = new Set();
  for (const pid of processes.keys()) {
    const chain = new Set();
    let current = pid;
    while (!resolved.has(current)) {
      if (chain.has(current)) {
        parents.delete(current);
        break;
      }
      chain.add(current);
      const parent = parents.get(current);
      if (parent === undefined) break;
      current = parent;
    }
    for (const item of chain) resolved.add(item);
  }

  /** @type {Map<number, Process[]>} */
  const allChildren = new Map();
  /** @type {Process[]} */
  const allRoots = [];
  for (const process of processes.values()) {
    const parent = parents.get(process.pid);
    if (parent === undefined) allRoots.push(process);
    else {
      const siblings = allChildren.get(parent) || [];
      siblings.push(process);
      allChildren.set(parent, siblings);
    }
  }

  // Aggregate before filtering so searches do not change metric definitions.
  // Memory is a sum of reported RSS; shared pages can be counted more than once.
  const order = [];
  const stack = [...allRoots];
  while (stack.length) {
    const process = stack.pop();
    if (!process) break;
    order.push(process);
    for (const child of allChildren.get(process.pid) || []) stack.push(child);
  }
  /** @type {Map<number, {cpu: number, memory: number, count: number, pinned: boolean}>} */
  const totals = new Map();
  for (let index = order.length - 1; index >= 0; index--) {
    const process = order[index];
    const total = {
      cpu: Number.isFinite(process.cpu_usage)
        ? Math.max(0, process.cpu_usage)
        : 0,
      memory: Number.isFinite(process.memory_usage)
        ? Math.max(0, process.memory_usage)
        : 0,
      count: 1,
      pinned: options.pinnedProcesses?.has(process.command) || false,
    };
    for (const child of allChildren.get(process.pid) || []) {
      const childTotal = totals.get(child.pid);
      if (!childTotal) continue;
      total.cpu += childTotal.cpu;
      total.memory += childTotal.memory;
      total.count += childTotal.count;
      total.pinned ||= childTotal.pinned;
    }
    totals.set(process.pid, total);
  }

  const included = new Set();
  if (matchingPids === null) {
    for (const pid of processes.keys()) included.add(pid);
  } else {
    for (const pid of matchingPids) {
      let current = pid;
      while (processes.has(current) && !included.has(current)) {
        included.add(current);
        const parent = parents.get(current);
        if (parent === undefined) break;
        current = parent;
      }
    }
  }

  /** @type {Map<number, Process[]>} */
  const children = new Map();
  /** @type {Process[]} */
  const roots = [];
  for (const process of processes.values()) {
    if (!included.has(process.pid)) continue;
    const parent = parents.get(process.pid);
    if (parent === undefined || !included.has(parent)) {
      roots.push(process);
    } else {
      const siblings = children.get(parent) || [];
      siblings.push(process);
      children.set(parent, siblings);
    }
  }

  /** @param {Process} left @param {Process} right */
  function compareFamilies(left, right) {
    const a = totals.get(left.pid);
    const b = totals.get(right.pid);
    if (!a || !b) return 0;
    if (a.pinned !== b.pinned) return a.pinned ? -1 : 1;
    const direction = options.sortConfig?.direction === "asc" ? 1 : -1;
    if (options.sortConfig?.field === "cpu_usage")
      return direction * (a.cpu - b.cpu);
    if (options.sortConfig?.field === "memory_usage")
      return direction * (a.memory - b.memory);
    return 0;
  }
  roots.sort(compareFamilies);
  for (const siblings of children.values()) siblings.sort(compareFamilies);

  /** @type {ProcessTreeRow[]} */
  const rows = [];
  for (const [rootIndex, root] of roots.entries()) {
    const stack = [{ process: root, depth: 0 }];
    while (stack.length) {
      const entry = stack.pop();
      if (!entry) break;
      const { process, depth } = entry;
      const descendants = children.get(process.pid) || [];
      const key = processIdentity(process);
      const total = totals.get(process.pid);
      const isExpanded =
        descendants.length > 0 && (matchingPids !== null || expanded.has(key));
      rows.push({
        process,
        key,
        depth,
        childCount: descendants.length,
        expanded: isExpanded,
        rootIndex,
        familyCpuUsage: total?.cpu || 0,
        familyMemoryUsage: total?.memory || 0,
        familyProcessCount: total?.count || 1,
      });
      if (isExpanded) {
        for (let i = descendants.length - 1; i >= 0; i--) {
          stack.push({ process: descendants[i], depth: depth + 1 });
        }
      }
    }
  }
  return { rows, rootCount: roots.length };
}

/**
 * Paginate root families, keeping each expanded family together.
 * @param {{rows: ProcessTreeRow[], rootCount: number}} tree
 * @param {number} page
 * @param {number} perPage
 */
export function paginateProcessTree(tree, page, perPage) {
  const size =
    Number.isFinite(perPage) && perPage > 0
      ? Math.max(1, Math.floor(perPage))
      : 15;
  const totalPages = Math.max(1, Math.ceil(tree.rootCount / size));
  const currentPage = Math.max(
    1,
    Math.min(totalPages, Number.isFinite(page) ? Math.floor(page) : 1),
  );
  const start = (currentPage - 1) * size;
  return {
    rows: tree.rows.filter(
      (row) => row.rootIndex >= start && row.rootIndex < start + size,
    ),
    totalPages,
    currentPage,
  };
}
