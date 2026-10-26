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
