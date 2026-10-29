import assert from "node:assert/strict";
import test from "node:test";
import { resolveDownloadAssets } from "../docs/downloads.js";

function release(names, tag = "v1.3.0") {
  return {
    tag_name: tag,
    assets: names.map((name) => ({
      name,
      browser_download_url: `https://github.com/Abdenasser/neohtop/releases/download/${tag}/${name}`,
    })),
  };
}

test("website prefers the new Windows installer while supporting portable releases", () => {
  const old = "NeoHtop_1.3.0_x64.exe";
  const installer = "NeoHtop_1.3.0_x64-setup.exe";
  assert.ok(
    resolveDownloadAssets(release([old, installer])).windows.endsWith(
      installer,
    ),
  );
  assert.ok(resolveDownloadAssets(release([old])).windows.endsWith(old));
});

test("website does not invent ARM downloads missing from a release", () => {
  const links = resolveDownloadAssets(release(["NeoHtop_1.3.0_x86_64.deb"]));
  assert.deepEqual(Object.keys(links), ["linux-deb-x64"]);
  assert.equal(links["linux-deb-arm64"], undefined);
  assert.deepEqual(
    resolveDownloadAssets({ message: "API rate limit exceeded" }),
    {},
  );
});

test("download links preserve the actual tag and asset URL", () => {
  const tag = "v1.3.0-beta.1";
  const name = "silicon-NeoHtop_1.3.0-beta.1_aarch64.dmg";
  assert.equal(
    resolveDownloadAssets(release([name], tag))["macos-silicon"],
    `https://github.com/Abdenasser/neohtop/releases/download/${tag}/${name}`,
  );
});

test("cached older, draft and preview responses cannot downgrade current downloads", async () => {
  const { selectPublishedRelease } = await import("../docs/downloads.js");
  const current = release(["silicon-NeoHtop_1.3.0_aarch64.dmg"]);
  const old = release(["silicon-NeoHtop_1.2.0_aarch64.dmg"], "v1.2.0");
  const next = release(["silicon-NeoHtop_1.4.0_aarch64.dmg"], "v1.4.0");
  assert.equal(selectPublishedRelease(old, current), current);
  assert.equal(
    selectPublishedRelease({ ...next, draft: true }, current),
    current,
  );
  assert.equal(
    selectPublishedRelease({ ...next, prerelease: true }, current),
    current,
  );
  assert.equal(
    selectPublishedRelease({ message: "rate limited" }, current),
    current,
  );
  assert.equal(selectPublishedRelease(release([], "v1.4.0"), current), current);
});
test("a newer published release advances downloads using numeric version comparison", async () => {
  const { selectPublishedRelease } = await import("../docs/downloads.js");
  const current = release(["NeoHtop_1.9.0_x64-setup.exe"], "v1.9.0");
  const newer = release(["NeoHtop_1.10.0_x64-setup.exe"], "v1.10.0");
  assert.equal(selectPublishedRelease(newer, current), newer);
  assert.equal(selectPublishedRelease(current, newer), newer);
});
test("same-version partial responses keep all verified architecture downloads", async () => {
  const { selectPublishedRelease } = await import("../docs/downloads.js");
  const current = release([
    "NeoHtop_1.3.0_aarch64.deb",
    "NeoHtop_1.3.0_x86_64.deb",
  ]);
  const partial = release(["NeoHtop_1.3.0_x86_64.deb"]);
  assert.equal(selectPublishedRelease(partial, current), current);
});
