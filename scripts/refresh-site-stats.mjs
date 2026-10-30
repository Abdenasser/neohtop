import { readFile, writeFile } from "node:fs/promises";
import { refreshCommunityStats } from "../docs/community.js";
const file = new URL("../docs/community-stats.json", import.meta.url);
const previous = JSON.parse(await readFile(file, "utf8"));
const { stats, unavailable } = await refreshCommunityStats();
if (!Object.keys(stats).length)
  throw new Error(
    "No statistics sources available; existing snapshot preserved",
  );
await writeFile(
  file,
  JSON.stringify({ ...previous, ...stats }, null, 2) + "\n",
);
console.log(`Updated public statistics: ${Object.keys(stats).join(", ")}.`);
if (unavailable.length)
  console.warn(`Preserved previous counts for: ${unavailable.join(", ")}.`);
console.log("Run npm run build:site to publish the refreshed snapshot.");
