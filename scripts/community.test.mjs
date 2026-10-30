import assert from "node:assert/strict";
import test from "node:test";
import {
  countReleaseDownloads,
  fetchGithubDownloads,
  parseHomebrewInstalls,
  parseNpmDownloads,
  refreshCommunityStats,
} from "../docs/community.js";

const asset = (name, download_count, id) => ({ name, download_count, id });
const release = (assets, extra = {}) => ({
  tag_name: "v1.3.0",
  draft: false,
  assets,
  ...extra,
});
const response = (data) => ({ ok: true, json: async () => data });

test("GitHub counts installers, excluding draft releases, signatures and metadata", () => {
  assert.equal(
    countReleaseDownloads([
      release([
        asset("silicon-NeoHtop_1.3.0_aarch64.dmg", 20, 1),
        asset("NeoHtop_1.3.0_x86_64.AppImage", 5, 2),
        asset("NeoHtop_1.3.0_x64-setup.exe", 4, 3),
        asset("SHA256SUMS", 100, 4),
        asset("release-manifest.json", 100, 5),
        asset("NeoHtop_1.3.0_x64-setup.exe.sig", 100, 6),
      ]),
      release([asset("NeoHtop-silicon.dmg", 99, 7)], { draft: true }),
      release([asset("NeoHtop-intel.dmg", 3, 8)], { tag_name: "v1.0.0" }),
    ]),
    32,
  );
  assert.equal(
    countReleaseDownloads([release([asset("NeoHtop-intel.dmg", 0)])]),
    0,
  );
});

test("GitHub follows pagination and avoids counting an asset repeated across pages", async () => {
  const urls = [];
  const firstPage = Array.from({ length: 100 }, (_, i) =>
    release([asset("NeoHtop-intel.dmg", 1, i)]),
  );
  const result = await fetchGithubDownloads(async (url) => {
    urls.push(url);
    return response(
      url.endsWith("page=1")
        ? firstPage
        : [
            release([
              asset("NeoHtop-intel.dmg", 1, 99),
              asset("NeoHtop-silicon.dmg", 7, 101),
            ]),
          ],
    );
  });
  assert.equal(result, 107);
  assert.equal(urls.length, 2);
  assert.ok(urls[1].endsWith("page=2"));
});

test("GitHub never returns a partial total after a pagination failure or bound", async () => {
  const fullPage = Array.from({ length: 100 }, () => release([]));
  await assert.rejects(
    fetchGithubDownloads(async (url) => {
      if (url.endsWith("page=2")) throw new Error("Rate limited");
      return response(fullPage);
    }),
    /Rate limited/,
  );
  await assert.rejects(
    fetchGithubDownloads(async () => response(fullPage)),
    /pagination exceeded/,
  );
});

test("Homebrew uses the neohtop cask analytics, with zero distinct from unavailable", () => {
  assert.equal(
    parseHomebrewInstalls({
      token: "neohtop",
      analytics: { install: { "365d": { neohtop: "3,051" } } },
    }),
    3051,
  );
  assert.equal(
    parseHomebrewInstalls({
      token: "neohtop",
      analytics: { install: { "365d": { neohtop: 0 } } },
    }),
    0,
  );
  assert.throws(() =>
    parseHomebrewInstalls({ token: "neohtop", formulae: {} }),
  );
  assert.throws(() =>
    parseHomebrewInstalls({
      token: "different",
      analytics: { install: { "365d": { neohtop: 9 } } },
    }),
  );
});

test("CLI statistics validate the package, count and reported period", () => {
  const data = {
    package: "neohtop-cli",
    downloads: 2536,
    start: "2025-10-30",
    end: "2026-10-29",
  };
  assert.deepEqual(parseNpmDownloads(data), {
    value: 2536,
    start: data.start,
    end: data.end,
  });
  for (const patch of [
    { package: "neohtop" },
    { downloads: -1 },
    { downloads: null },
    { end: "2024-01-01" },
    { start: "unknown" },
  ])
    assert.throws(() => parseNpmDownloads({ ...data, ...patch }));
});

test("a failed source does not hide successful counters or invent a zero", async () => {
  const now = new Date("2026-10-30T12:00:00Z");
  const result = await refreshCommunityStats({
    now,
    fetcher: async (url) => {
      if (url.includes("github.com")) return { ok: false, status: 403 };
      if (url.includes("brew.sh"))
        return response({
          token: "neohtop",
          analytics: { install: { "365d": { neohtop: 12 } } },
        });
      return response({
        package: "neohtop-cli",
        downloads: 0,
        start: "2025-10-30",
        end: "2026-10-29",
      });
    },
  });
  assert.deepEqual(result.unavailable, ["github"]);
  assert.equal(result.stats.github, undefined);
  assert.deepEqual(result.stats.homebrew, {
    value: 12,
    updatedAt: now.toISOString(),
  });
  assert.equal(result.stats.npm.value, 0);
});

test("invalid responses and offline sources preserve the static fallback", async () => {
  for (const fetcher of [
    async () => {
      throw new Error("Offline");
    },
    async () => response({ error: "Unavailable" }),
  ]) {
    const result = await refreshCommunityStats({ fetcher });
    assert.deepEqual(result.stats, {});
    assert.deepEqual(result.unavailable, ["github", "homebrew", "npm"]);
  }
});
