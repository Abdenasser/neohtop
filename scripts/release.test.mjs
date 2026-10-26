import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import {
  copyFile,
  mkdir,
  mkdtemp,
  readFile,
  rm,
  symlink,
  writeFile,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import {
  checkVersion,
  createManifest,
  expectedAssets,
  forbiddenDisplayLibraries,
  projectRoot,
  stagePackages,
} from "./release.mjs";
import { uploadDraft } from "./upload-draft.mjs";

async function fixture(t) {
  const root = await mkdtemp(join(tmpdir(), "neohtop-release-test-"));
  t.after(() => rm(root, { recursive: true, force: true }));
  await mkdir(join(root, "src-tauri"));
  for (const file of [
    "package.json",
    "package-lock.json",
    "src-tauri/tauri.conf.json",
    "src-tauri/Cargo.toml",
    "src-tauri/Cargo.lock",
  ]) {
    await copyFile(join(projectRoot, file), join(root, file));
  }
  const version = await checkVersion(root);
  const artifacts = join(root, "artifacts");
  await mkdir(artifacts);
  for (const name of expectedAssets(version))
    await writeFile(join(artifacts, name), `test package: ${name}`);
  return { root, artifacts, version };
}

test("release versions must agree across both lockfiles and app manifests", async (t) => {
  const { root, version } = await fixture(t);
  assert.equal(await checkVersion(root, `v${version}`), version);
  await assert.rejects(checkVersion(root, "v99.0.0"), /does not match/);
  const path = join(root, "src-tauri/Cargo.lock");
  const lock = await readFile(path, "utf8");
  await writeFile(
    path,
    lock.replace(/(name = "neohtop"\s*\nversion = ")[^"]+/, "$199.0.0"),
  );
  await assert.rejects(checkVersion(root), /Versions differ/);
});

test("the manifest requires every platform and independently verifiable hashes", async (t) => {
  const { root, artifacts, version } = await fixture(t);
  const manifest = await createManifest(
    artifacts,
    `v${version}`,
    root,
    "abc123",
  );
  assert.equal(manifest.assets.length, 9);
  assert.equal(manifest.commit, "abc123");
  const sums = await readFile(join(artifacts, "SHA256SUMS"), "utf8");
  for (const asset of manifest.assets) {
    const data = await readFile(join(artifacts, asset.name));
    const digest = createHash("sha256").update(data).digest("hex");
    assert.equal(asset.size, data.length);
    assert.equal(asset.sha256, digest);
    assert.ok(sums.includes(`${digest}  ${asset.name}\n`));
  }
  // Rerunning a draft must accept the previously generated metadata.
  assert.deepEqual(
    await createManifest(artifacts, `v${version}`, root, "abc123"),
    manifest,
  );
});

test("a missing, empty, or stale platform package blocks the release", async (t) => {
  const { root, artifacts, version } = await fixture(t);
  const name = expectedAssets(version)[0];
  const path = join(artifacts, name);
  await rm(path);
  await assert.rejects(
    createManifest(artifacts, `v${version}`, root),
    /Missing:/,
  );
  await writeFile(path, "");
  await assert.rejects(createManifest(artifacts, `v${version}`, root), /empty/);
  await writeFile(path, "restored");
  await writeFile(join(artifacts, "NeoHtop_old_version.exe"), "stale");
  await assert.rejects(
    createManifest(artifacts, `v${version}`, root),
    /Unexpected: NeoHtop_old_version/,
  );
});

test("Windows staging selects an installer instead of a portable executable", async (t) => {
  const { root, version } = await fixture(t);
  const source = join(root, "bundle");
  await mkdir(join(source, "nsis"), { recursive: true });
  await writeFile(join(source, "nsis/NeoHtop_setup.exe"), "installer");
  await writeFile(join(source, "NeoHtop.exe"), "portable");
  const destination = join(root, "staged");
  const names = await stagePackages(
    "windows-x86_64",
    source,
    destination,
    root,
  );
  assert.deepEqual(names, [`NeoHtop_${version}_x64-setup.exe`]);
  assert.equal(
    await readFile(join(destination, names[0]), "utf8"),
    "installer",
  );
  await writeFile(join(source, "nsis/stale_setup.exe"), "stale");
  await assert.rejects(
    stagePackages("windows-x86_64", source, destination, root),
    /found 2/,
  );
});

test("AppImage inspection detects bundled Wayland libraries, including symlinks", async (t) => {
  const { root } = await fixture(t);
  const libraries = join(root, "usr/lib");
  await mkdir(libraries, { recursive: true });
  await writeFile(join(libraries, "libwebkit2gtk-4.1.so"), "allowed");
  assert.deepEqual(await forbiddenDisplayLibraries(libraries), []);
  await writeFile(
    join(libraries, "libwayland-client.so.0.24.0"),
    "host library",
  );
  // File symlinks are supported on Windows without Developer Mode only when
  // elevated, so the symlink case runs on Unix CI runners.
  if (process.platform !== "win32")
    await symlink(
      "libwayland-client.so.0.24.0",
      join(libraries, "libwayland-client.so.0"),
    );
  assert.equal(
    (await forbiddenDisplayLibraries(libraries)).length,
    process.platform === "win32" ? 1 : 2,
  );
});

function githubFixture(tag, state = {}) {
  const writes = [];
  const context = {
    ref: `refs/tags/${tag}`,
    sha: "abc123",
    repo: { owner: "test", repo: "neohtop" },
  };
  const github = {
    rest: {
      repos: {
        getCommit: async () => ({ data: { sha: state.sha || context.sha } }),
        getReleaseByTag: async () => {
          if (state.release) return { data: state.release };
          throw Object.assign(new Error("not found"), { status: 404 });
        },
        listReleaseAssets: () => {},
        createRelease: async (data) => {
          writes.push(["create", data]);
          return { data: { id: 1, draft: true } };
        },
        deleteReleaseAsset: async (data) => {
          writes.push(["delete", data]);
        },
        uploadReleaseAsset: async (data) => {
          writes.push(["upload", data]);
        },
      },
    },
    paginate: async () => state.assets || [],
  };
  return { github, context, writes };
}

test("draft upload refuses published releases and moved tags before any writes", async (t) => {
  const { root, artifacts, version } = await fixture(t);
  for (const state of [
    { release: { id: 1, draft: false } },
    { sha: "different-commit" },
  ]) {
    const { github, context, writes } = githubFixture(`v${version}`, state);
    await assert.rejects(
      uploadDraft({ github, context, directory: artifacts, root }),
      /published|no longer points/,
    );
    assert.deepEqual(writes, []);
  }
});

test("a new release stays draft and uploads all nine packages plus metadata", async (t) => {
  const { root, artifacts, version } = await fixture(t);
  const { github, context, writes } = githubFixture(`v${version}`);
  await uploadDraft({ github, context, directory: artifacts, root });
  const create = writes.find(([kind]) => kind === "create")[1];
  assert.equal(create.draft, true);
  assert.equal(create.target_commitish, context.sha);
  assert.equal(writes.filter(([kind]) => kind === "upload").length, 11);
  assert.equal(writes.filter(([kind]) => kind === "delete").length, 0);
});

test("an interrupted draft upload can be retried without creating a second release", async (t) => {
  const { root, artifacts, version } = await fixture(t);
  const { github, context, writes } = githubFixture(`v${version}`, {
    release: { id: 5, draft: true },
    assets: [{ id: 7, name: expectedAssets(version)[0] }],
  });
  await uploadDraft({ github, context, directory: artifacts, root });
  assert.equal(writes.filter(([kind]) => kind === "create").length, 0);
  assert.equal(writes.filter(([kind]) => kind === "delete").length, 1);
  assert.equal(writes.filter(([kind]) => kind === "upload").length, 11);
});

test("incomplete artifacts and non-tag runs cannot create a draft", async (t) => {
  const { root, artifacts, version } = await fixture(t);
  const { github, context, writes } = githubFixture(`v${version}`);
  await rm(join(artifacts, expectedAssets(version)[0]));
  await assert.rejects(
    uploadDraft({ github, context, directory: artifacts, root }),
    /incomplete/,
  );
  context.ref = "refs/heads/main";
  await assert.rejects(
    uploadDraft({ github, context, directory: artifacts, root }),
    /version tag/,
  );
  assert.deepEqual(writes, []);
});
