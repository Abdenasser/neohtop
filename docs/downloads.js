// Select only assets that actually exist on the latest published release.
// Prefer the new Windows installer, while supporting older portable releases.
export function resolveDownloadAssets(release) {
  const version = release.tag_name?.match(
    /^v(\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?)$/,
  )?.[1];
  if (!version || !Array.isArray(release.assets)) return {};
  const candidates = {
    "macos-intel": [`intel-NeoHtop_${version}_x64.dmg`],
    "macos-silicon": [`silicon-NeoHtop_${version}_aarch64.dmg`],
    windows: [`NeoHtop_${version}_x64-setup.exe`, `NeoHtop_${version}_x64.exe`],
    "linux-deb-x64": [`NeoHtop_${version}_x86_64.deb`],
    "linux-appimage-x64": [`NeoHtop_${version}_x86_64.AppImage`],
    "linux-rpm-x64": [`NeoHtop_${version}_x86_64.rpm`],
    "linux-deb-arm64": [`NeoHtop_${version}_aarch64.deb`],
    "linux-appimage-arm64": [`NeoHtop_${version}_aarch64.AppImage`],
    "linux-rpm-arm64": [`NeoHtop_${version}_aarch64.rpm`],
  };
  const assets = new Map(release.assets.map((asset) => [asset.name, asset]));
  return Object.fromEntries(
    Object.entries(candidates).flatMap(([platform, names]) => {
      const asset = names.map((name) => assets.get(name)).find(Boolean);
      return asset?.browser_download_url
        ? [[platform, asset.browser_download_url]]
        : [];
    }),
  );
}
