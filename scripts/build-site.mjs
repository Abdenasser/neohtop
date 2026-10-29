import { build } from "vite";
import { faq } from "../site/content.js";
import publishedRelease from "../docs/release.json" with { type: "json" };
import { format, resolveConfig } from "prettier";
import {
  readFile,
  writeFile,
  readdir,
  copyFile,
  mkdir,
} from "node:fs/promises";
import { fileURLToPath, pathToFileURL } from "node:url";
import path from "node:path";
const root = fileURLToPath(new URL("../", import.meta.url));
await build({ configFile: path.join(root, "site/vite.config.js") });
const { renderSite } = await import(
  pathToFileURL(path.join(root, ".site-build/entry-server.mjs"))
);
const assets = await readdir(path.join(root, ".site-build/assets"));
const css = assets.find((name) => name.endsWith(".css"));
if (!css) throw new Error("Website CSS was not emitted");
await mkdir(path.join(root, "docs/assets"), { recursive: true });
// Keep earlier hashed bundles available for browsers with cached HTML.
for (const name of assets)
  await copyFile(
    path.join(root, ".site-build/assets", name),
    path.join(root, "docs/assets", name),
  );
const runtimeBuild = await build({
  configFile: false,
  publicDir: false,
  build: {
    write: false,
    rollupOptions: {
      input: path.join(root, "docs/main.js"),
      output: {
        entryFileNames: "site-runtime-[hash].js",
        chunkFileNames: "site-runtime-[hash].js",
      },
    },
  },
});
const runtimeOutput = (
  Array.isArray(runtimeBuild) ? runtimeBuild[0] : runtimeBuild
).output;
const runtime = runtimeOutput.find(
  (file) => file.type === "chunk" && file.isEntry,
);
if (!runtime) throw new Error("Website runtime was not emitted");
for (const file of runtimeOutput)
  await writeFile(
    path.join(root, "docs/assets", file.fileName),
    file.type === "chunk" ? file.code : file.source,
  );
const pages = [
  {
    page: "home",
    file: "index.html",
    markdown: "index.md",
    canonical: "",
    title: "NeoHtop — Your system. In plain sight.",
    description:
      "A free, open-source system monitor for macOS, Linux and Windows. Explore processes, GPU stats, resource charts, 24 themes and a menu-bar companion.",
  },
  {
    page: "install",
    file: "install.html",
    markdown: "install.md",
    canonical: "install.html",
    title: "Install NeoHtop — macOS, Windows and Linux",
    description:
      "Download and install NeoHtop Desktop on Apple Silicon or Intel Macs, Windows x64 and Linux x86_64 or ARM64. Official packages, checksums and troubleshooting.",
  },
  {
    page: "comparison",
    file: "desktop-vs-cli.html",
    markdown: "desktop-vs-cli.md",
    canonical: "desktop-vs-cli.html",
    title: "NeoHtop Desktop vs CLI — Choose your workspace",
    description:
      "Compare NeoHtop Desktop and NeoHtop CLI: graphical system monitoring with GPU diagnostics and a menu-bar overview, or keyboard-driven monitoring in your terminal.",
  },
];
pages.push({
  page: "privacy",
  file: "privacy.html",
  markdown: "privacy.md",
  canonical: "privacy.html",
  title: "NeoHtop Privacy — Local monitoring and analytics choices",
  description:
    "What NeoHtop keeps local, how the basic launch counter works, and how to choose optional desktop feature sharing in Settings.",
});
const base = "https://abdenasser.github.io/neohtop/";
const escape = (value) =>
  value
    .replaceAll("&", "&amp;")
    .replaceAll('"', "&quot;")
    .replaceAll("<", "&lt;");
for (const page of pages) {
  const rendered = renderSite(page.page);
  const schema =
    page.page === "home"
      ? {
          "@context": "https://schema.org",
          "@type": "SoftwareApplication",
          name: "NeoHtop",
          softwareVersion: publishedRelease.tag_name.slice(1),
          applicationCategory: "UtilitiesApplication",
          operatingSystem: ["macOS", "Linux", "Windows"],
          description: page.description,
          url: base,
          license: "https://github.com/Abdenasser/neohtop/blob/main/LICENSE",
          isAccessibleForFree: true,
          offers: { "@type": "Offer", price: "0", priceCurrency: "USD" },
          image: base + "assets/neohtop-desktop.png",
        }
      : {
          "@context": "https://schema.org",
          "@type": "WebPage",
          name: page.title,
          url: base + page.canonical,
          description: page.description,
        };
  const structured =
    page.page === "home"
      ? {
          "@context": "https://schema.org",
          "@graph": [
            { "@type": "WebSite", name: "NeoHtop", url: base },
            schema,
            {
              "@type": "FAQPage",
              mainEntity: faq.map(([name, text]) => ({
                "@type": "Question",
                name,
                acceptedAnswer: { "@type": "Answer", text },
              })),
            },
          ],
        }
      : schema;
  const html = `<!doctype html>
<html lang="en" class="dark" data-theme="catppuccin"><head><meta charset="UTF-8" /><meta name="viewport" content="width=device-width,initial-scale=1" /><meta name="color-scheme" content="dark" /><title>${escape(page.title)}</title><meta name="description" content="${escape(page.description)}" /><link rel="canonical" href="${base + page.canonical}" /><link rel="alternate" type="text/markdown" href="${page.markdown}" title="Markdown version" /><link rel="describedby" href="llms.txt" /><link rel="icon" href="favicon.ico" /><link rel="apple-touch-icon" href="assets/neohtop-icon.png" /><meta property="og:type" content="website" /><meta property="og:site_name" content="NeoHtop" /><meta property="og:url" content="${base + page.canonical}" /><meta property="og:title" content="${escape(page.title)}" /><meta property="og:description" content="${escape(page.description)}" /><meta property="og:image" content="${base}assets/neohtop-desktop.png" /><meta property="og:image:width" content="1280" /><meta property="og:image:height" content="900" /><meta property="og:image:alt" content="NeoHtop in Catppuccin with resource charts and a searchable process table" /><meta name="twitter:card" content="summary_large_image" /><meta name="twitter:creator" content="@__abdenasser" /><meta name="twitter:title" content="${escape(page.title)}" /><meta name="twitter:description" content="${escape(page.description)}" /><meta name="twitter:image" content="${base}assets/neohtop-desktop.png" /><link rel="stylesheet" href="assets/${css}" /><script type="application/ld+json">${JSON.stringify(structured)}</script>${rendered.head}</head><body>${rendered.body}<script type="module" src="assets/${runtime.fileName}"></script></body></html>
`;
  await writeFile(
    path.join(root, "docs", page.file),
    await format(html, {
      ...(await resolveConfig(path.join(root, "docs", page.file))),
      filepath: path.join(root, "docs", page.file),
    }),
  );
}
console.log("Rendered 4 static pages using shared shadcn-svelte components.");
