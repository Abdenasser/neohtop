import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { checkVersion, createManifest, projectRoot } from "./release.mjs";

export async function uploadDraft({
  github,
  context,
  directory,
  root = projectRoot,
}) {
  if (!context.ref.startsWith("refs/tags/"))
    throw new Error("Draft releases require a version tag.");
  const tag = context.ref.slice("refs/tags/".length);
  const version = await checkVersion(root, tag);
  // Validate everything before making any GitHub writes.
  const manifest = await createManifest(directory, tag, root, context.sha);
  const { data: commit } = await github.rest.repos.getCommit({
    ...context.repo,
    ref: tag,
  });
  if (commit.sha !== context.sha)
    throw new Error(
      "The version tag no longer points to the commit built by this workflow.",
    );

  let release;
  try {
    ({ data: release } = await github.rest.repos.getReleaseByTag({
      ...context.repo,
      tag,
    }));
  } catch (error) {
    if (error.status !== 404) throw error;
  }
  if (release && !release.draft)
    throw new Error("Refusing to modify an already published release.");

  const names = [
    ...manifest.assets.map((asset) => asset.name),
    "SHA256SUMS",
    "release-manifest.json",
  ];
  const existing = release
    ? await github.paginate(github.rest.repos.listReleaseAssets, {
        ...context.repo,
        release_id: release.id,
        per_page: 100,
      })
    : [];
  if (existing.some((asset) => !names.includes(asset.name))) {
    throw new Error(
      "The existing draft has unexpected assets. Review them before retrying this release.",
    );
  }
  if (!release) {
    ({ data: release } = await github.rest.repos.createRelease({
      ...context.repo,
      tag_name: tag,
      target_commitish: context.sha,
      name: `NeoHtop v${version}`,
      body: `Built from ${context.sha}.\n\nAll five platform builds passed. Checksums and a build manifest are attached.\n\nReview the packages and release notes before publishing.`,
      draft: true,
      prerelease: version.includes("-"),
    }));
  }
  for (const name of names) {
    const previous = existing.find((asset) => asset.name === name);
    if (previous)
      await github.rest.repos.deleteReleaseAsset({
        ...context.repo,
        asset_id: previous.id,
      });
    const data = await readFile(join(directory, name));
    await github.rest.repos.uploadReleaseAsset({
      ...context.repo,
      release_id: release.id,
      name,
      data,
      headers: {
        "content-type": "application/octet-stream",
        "content-length": data.length,
      },
    });
  }
  return release.html_url;
}
