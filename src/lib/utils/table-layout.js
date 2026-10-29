/** Stable column widths let the standard desktop layout fit and keep
 * long optional fields readable through deliberate horizontal scrolling. */
const widths = {
  name: 250,
  pid: 75,
  ppid: 85,
  session_id: 85,
  status: 100,
  user: 65,
  cpu_usage: 95,
  memory_usage: 110,
  virtual_memory: 105,
  disk_usage: 145,
  run_time: 130,
  start_time: 180,
};
/** @param {string} id @param {boolean} tree */
export function columnWidth(id, tree = false) {
  return id === "name" && tree
    ? 340
    : (widths[/** @type {keyof typeof widths} */ (id)] ?? 220);
}
/** @param {{id:string,visible:boolean}[]} columns @param {boolean} tree */
export function tableWidth(columns, tree = false) {
  return (
    112 +
    columns
      .filter((c) => c.visible)
      .reduce((sum, c) => sum + columnWidth(c.id, tree), 0)
  );
}
