import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";
import {
  copyFile,
  lstat,
  mkdir,
  mkdtemp,
  readFile,
  readdir,
  rm,
  writeFile,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import { basename, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";

export const projectRoot = fileURLToPath(new URL("../", import.meta.url));

// Keep existing macOS/Linux download names stable for package managers.
const platforms = {
  "linux-x86_64": [
    ["appimage", ".AppImage", "NeoHtop_VERSION_x86_64.AppImage"],
    ["deb", ".deb", "NeoHtop_VERSION_x86_64.deb"],
    ["rpm", ".rpm", "NeoHtop_VERSION_x86_64.rpm"],
  ],
  "linux-aarch64": [
    ["appimage", ".AppImage", "NeoHtop_VERSION_aarch64.AppImage"],
    ["deb", ".deb", "NeoHtop_VERSION_aarch64.deb"],
    ["rpm", ".rpm", "NeoHtop_VERSION_aarch64.rpm"],
  ],
  "macos-x86_64": [["dmg", ".dmg", "intel-NeoHtop_VERSION_x64.dmg"]],
  "macos-aarch64": [["dmg", ".dmg", "silicon-NeoHtop_VERSION_aarch64.dmg"]],
  // The old Windows .exe was a portable binary. Label the new installer.
  "windows-x86_64": [["nsis", ".exe", "NeoHtop_VERSION_x64-setup.exe"]],
};

export async function checkVersion(root = projectRoot, tag = "") {
  const pkg = JSON.parse(await readFile(join(root, "package.json"), "utf8"));
  const npmLock = JSON.parse(
    await readFile(join(root, "package-lock.json"), "utf8"),
  );
  const config = JSON.parse(
    await readFile(join(root, "src-tauri/tauri.conf.json"), "utf8"),
  );
  const cargo = await readFile(join(root, "src-tauri/Cargo.toml"), "utf8");
  const lock = await readFile(join(root, "src-tauri/Cargo.lock"), "utf8");
  const packageBlock = cargo
    .split(/(?=^\[)/m)
    .find((block) => block.startsWith("[package]"));
  const cargoVersion = packageBlock?.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
  const lockedVersion = lock.match(
    /\[\[package\]\]\s*\nname = "neohtop"\s*\nversion = "([^"]+)"/,
  )?.[1];
  const versions = [
    pkg.version,
    npmLock.version,
    npmLock.packages?.[""]?.version,
    config.version,
    cargoVersion,
    lockedVersion,
  ];
  if (
    !versions.every(
      (version) => typeof version === "string" && version === pkg.version,
    )
  ) {
    throw new Error(
      `Versions differ (package.json, npm lock/root package, Tauri, Cargo.toml, Cargo.lock): ${versions.join(", ")}`,
    );
  }
  if (
    !/^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-[0-9A-Za-z.-]+)?$/.test(
      pkg.version,
    )
  ) {
    throw new Error(`Invalid release version: ${pkg.version}`);
  }
  if (tag && tag !== `v${pkg.version}`) {
    throw new Error(`Tag ${tag} does not match v${pkg.version}`);
  }
  return pkg.version;
}

export function expectedAssets(version) {
  return Object.values(platforms)
    .flatMap((bundles) =>
      bundles.map(([, , name]) => name.replace("VERSION", version)),
    )
    .sort();
}

async function requirePackage(path) {
  const info = await lstat(path);
  if (!info.isFile() || info.size === 0)
    throw new Error(`Missing, empty, or non-file package: ${path}`);
  return info.size;
}

export async function stagePackages(
  platform,
  source,
  destination,
  root = projectRoot,
) {
  const bundles = platforms[platform];
  if (!bundles) throw new Error(`Unknown platform: ${platform}`);
  const version = await checkVersion(root);
  const packages = [];
  for (const [folder, extension, name] of bundles) {
    const directory = join(source, folder);
    const candidates = (await readdir(directory)).filter((file) =>
      file.endsWith(extension),
    );
    if (candidates.length !== 1)
      throw new Error(
        `Expected one ${extension} package in ${directory}, found ${candidates.length}`,
      );
    const path = join(directory, candidates[0]);
    await requirePackage(path);
    packages.push([path, name.replace("VERSION", version)]);
  }
  await mkdir(destination, { recursive: true });
  for (const [path, name] of packages)
    await copyFile(path, join(destination, name));
  return packages.map(([, name]) => name);
}

async function sha256(path) {
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(path)) hash.update(chunk);
  return hash.digest("hex");
}

export async function createManifest(
  directory,
  tag,
  root = projectRoot,
  commit = process.env.GITHUB_SHA || null,
) {
  const version = await checkVersion(root, tag);
  const expected = expectedAssets(version);
  const actual = (await readdir(directory))
    .filter((file) => !["SHA256SUMS", "release-manifest.json"].includes(file))
    .sort();
  if (JSON.stringify(actual) !== JSON.stringify(expected)) {
    throw new Error(
      `Release packages are incomplete or unexpected.\nMissing: ${expected.filter((file) => !actual.includes(file)).join(", ")}\nUnexpected: ${actual.filter((file) => !expected.includes(file)).join(", ")}`,
    );
  }
  const assets = [];
  for (const name of expected) {
    const path = join(directory, name);
    const size = await requirePackage(path);
    assets.push({ name, size, sha256: await sha256(path) });
  }
  const manifest = { schemaVersion: 1, version, commit, assets };
  await writeFile(
    join(directory, "SHA256SUMS"),
    assets.map((asset) => `${asset.sha256}  ${asset.name}\n`).join(""),
  );
  await writeFile(
    join(directory, "release-manifest.json"),
    `${JSON.stringify(manifest, null, 2)}\n`,
  );
  return manifest;
}

export async function forbiddenDisplayLibraries(directory) {
  const found = [];
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (/^libwayland-client\.so(?:\.|$)/.test(entry.name)) found.push(path);
    // Do not follow symlinks into the build host's library directories.
    if (entry.isDirectory())
      found.push(...(await forbiddenDisplayLibraries(path)));
  }
  return found;
}

async function checkAppImage(directory) {
  const images = (await readdir(directory)).filter((name) =>
    name.endsWith(".AppImage"),
  );
  if (images.length !== 1)
    throw new Error(`Expected one AppImage, found ${images.length}`);
  const temporary = await mkdtemp(join(tmpdir(), "neohtop-appimage-"));
  try {
    const image = resolve(directory, images[0]);
    const extracted = spawnSync(image, ["--appimage-extract"], {
      cwd: temporary,
      stdio: ["ignore", "ignore", "pipe"],
      encoding: "utf8",
    });
    if (extracted.error || extracted.status !== 0)
      throw new Error(
        `Cannot inspect AppImage: ${extracted.error || extracted.stderr}`,
      );
    const forbidden = await forbiddenDisplayLibraries(
      join(temporary, "squashfs-root"),
    );
    if (forbidden.length)
      throw new Error(
        `AppImage bundles host Wayland libraries (see #203): ${forbidden.map((path) => basename(path)).join(", ")}`,
      );
  } finally {
    await rm(temporary, { recursive: true, force: true });
  }
}

if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
) {
  const [command, ...args] = process.argv.slice(2);
  try {
    switch (command) {
      case "check-version":
        console.log(
          `Release version: ${await checkVersion(projectRoot, args[0])}`,
        );
        break;
      case "stage":
        console.log((await stagePackages(...args)).join("\n"));
        break;
      case "manifest":
        console.log(JSON.stringify(await createManifest(...args), null, 2));
        break;
      case "check-appimage":
        await checkAppImage(args[0]);
        console.log("AppImage uses the host Wayland client library.");
        break;
      default:
        throw new Error(
          "Usage: release.mjs check-version [tag] | stage <platform> <bundle-dir> <output-dir> | manifest <artifact-dir> <tag> | check-appimage <bundle-dir>",
        );
    }
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
