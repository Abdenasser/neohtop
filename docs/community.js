// Public distribution counters. Keep sources separate: periods differ and
// Homebrew can download the same GitHub asset counted by GitHub.
function count(value) {
  if (typeof value === "string" && /^\d+(?:,\d{3})*$/.test(value))
    value = Number(value.replaceAll(",", ""));
  if (!Number.isSafeInteger(value) || value < 0)
    throw new Error("Invalid public download count");
  return value;
}

export function countReleaseDownloads(releases, seen = new Set()) {
  if (!Array.isArray(releases)) throw new Error("Invalid GitHub releases");
  let total = 0;
  for (const release of releases) {
    if (release.draft) continue;
    if (!Array.isArray(release.assets))
      throw new Error("Missing release assets");
    for (const asset of release.assets) {
      if (
        !/^(?:(?:intel|silicon)-)?neohtop.*\.(?:dmg|exe|msi|deb|rpm|appimage)$/i.test(
          asset.name,
        )
      )
        continue;
      const id = asset.id ?? `${release.tag_name}/${asset.name}`;
      if (seen.has(id)) continue;
      seen.add(id);
      total += count(asset.download_count);
      if (!Number.isSafeInteger(total))
        throw new Error("Download count overflow");
    }
  }
  return total;
}

export function parseHomebrewInstalls(data) {
  if (data?.token !== "neohtop") throw new Error("Unexpected Homebrew cask");
  return count(data.analytics?.install?.["365d"]?.neohtop);
}

export function parseNpmDownloads(data) {
  if (
    data?.package !== "neohtop-cli" ||
    !/^\d{4}-\d{2}-\d{2}$/.test(data.start) ||
    !/^\d{4}-\d{2}-\d{2}$/.test(data.end) ||
    data.start > data.end
  )
    throw new Error("Invalid CLI download period");
  return { value: count(data.downloads), start: data.start, end: data.end };
}

async function requestJson(url, fetcher) {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), 8000);
  try {
    const response = await fetcher(url, { signal: controller.signal });
    if (!response.ok)
      throw new Error(`Statistics unavailable (${response.status})`);
    return await response.json();
  } finally {
    clearTimeout(timeout);
  }
}

export async function fetchGithubDownloads(fetcher = fetch) {
  let total = 0;
  const seen = new Set();
  // Follow every page, never publish a truncated total if this bound is reached.
  for (let page = 1; page <= 20; page++) {
    const releases = await requestJson(
      `https://api.github.com/repos/Abdenasser/neohtop/releases?per_page=100&page=${page}`,
      fetcher,
    );
    total += countReleaseDownloads(releases, seen);
    if (releases.length < 100) return total;
  }
  throw new Error("GitHub release pagination exceeded");
}

export async function refreshCommunityStats({
  fetcher = fetch,
  now = new Date(),
} = {}) {
  const updatedAt = now.toISOString();
  const sources = {
    github: async () => ({
      value: await fetchGithubDownloads(fetcher),
      updatedAt,
    }),
    homebrew: async () => ({
      value: parseHomebrewInstalls(
        await requestJson(
          "https://formulae.brew.sh/api/cask/neohtop.json",
          fetcher,
        ),
      ),
      updatedAt,
    }),
    npm: async () => ({
      ...parseNpmDownloads(
        await requestJson(
          "https://api.npmjs.org/downloads/point/last-year/neohtop-cli",
          fetcher,
        ),
      ),
      updatedAt,
    }),
  };
  const results = await Promise.allSettled(
    Object.values(sources).map((run) => run()),
  );
  const stats = {},
    unavailable = [];
  Object.keys(sources).forEach((key, i) => {
    if (results[i].status === "fulfilled") stats[key] = results[i].value;
    else unavailable.push(key);
  });
  return { stats, unavailable };
}
