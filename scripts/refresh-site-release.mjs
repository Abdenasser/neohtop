import { readFile, writeFile } from "node:fs/promises";
import { selectPublishedRelease } from "../docs/downloads.js";
const file = new URL("../docs/release.json", import.meta.url);
const current = JSON.parse(await readFile(file, "utf8"));
const response = await fetch(
  "https://api.github.com/repos/Abdenasser/neohtop/releases/latest",
  {
    headers: { Accept: "application/vnd.github+json" },
    cache: "no-store",
    signal: AbortSignal.timeout(15000),
  },
);
if (!response.ok) throw new Error(`Release lookup failed: ${response.status}`);
const candidate = await response.json();
const selected = selectPublishedRelease(candidate, current);
if (selected === current) {
  console.log(`Keeping verified ${current.tag_name} downloads.`);
} else {
  const { tag_name, html_url, draft, prerelease } = selected;
  const snapshot = {
    tag_name,
    html_url,
    draft,
    prerelease,
    assets: selected.assets.map(({ name, browser_download_url }) => ({
      name,
      browser_download_url,
    })),
  };
  await writeFile(file, JSON.stringify(snapshot, null, 2) + "\n");
  console.log(
    `Updated verified website downloads to ${tag_name}. Run npm run build:site and commit the generated pages.`,
  );
}
