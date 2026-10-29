import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../docs/", import.meta.url));
const base = "https://abdenasser.github.io/neohtop/";
const pages = [
  "index.html",
  "install.html",
  "desktop-vs-cli.html",
  "privacy.html",
];
const read = (file) => fs.readFileSync(path.join(root, file), "utf8");

// Protect navigation and crawler entry points across the static deployment.
test("website links, fragments, images, and Markdown references resolve in the Pages subtree", () => {
  for (const file of pages) {
    const html = read(file);
    for (const match of html.matchAll(/\b(?:href|src)="([^"]+)"/g)) {
      const value = match[1];
      if (/^(?:https?:|mailto:|data:)/.test(value)) continue;
      assert.ok(!value.startsWith("/"), `${file}: ${value} escapes /neohtop/`);
      const [resource, fragment] = value.split("#");
      let target = resource || file;
      if (target.endsWith("/")) target += "index.html";
      const fullPath = path.resolve(root, target);
      assert.ok(fullPath.startsWith(root), `${file}: ${value} escapes docs`);
      assert.ok(fs.existsSync(fullPath), `${file}: missing ${value}`);
      if (fragment && target.endsWith(".html")) {
        assert.ok(
          read(target).includes(`id="${fragment}"`),
          `${file}: missing fragment ${value}`,
        );
      }
    }
  }
});

test("indexable pages have unique metadata, matching canonicals, and parseable structured data", () => {
  const titles = new Set();
  const descriptions = new Set();
  for (const file of pages) {
    const html = read(file);
    assert.equal([...html.matchAll(/<h1\b/g)].length, 1, file);
    const title = html.match(/<title>([^<]+)<\/title>/)?.[1];
    const description = html.match(
      /name="description"\s+content="([^"]+)"/s,
    )?.[1];
    assert.ok(title && description, `${file}: missing metadata`);
    titles.add(title);
    descriptions.add(description);
    const canonical = base + (file === "index.html" ? "" : file);
    assert.equal(
      html.match(/rel="canonical"\s+href="([^"]+)"/s)?.[1],
      canonical,
      file,
    );
    const json = html.match(
      /<script type="application\/ld\+json">([\s\S]*?)<\/script>/,
    )?.[1];
    const schema = JSON.parse(json);
    assert.equal(schema["@context"], "https://schema.org");
    assert.ok(html.includes('type="text/markdown"'), file);
    assert.ok(html.includes('rel="describedby" href="llms.txt"'), file);
    assert.ok(!/noindex|Your Name|Loading\.\.\./.test(html), file);
  }
  assert.equal(titles.size, pages.length);
  assert.equal(descriptions.size, pages.length);
  const locations = [...read("sitemap.xml").matchAll(/<loc>(.*?)<\/loc>/g)].map(
    (m) => m[1],
  );
  assert.deepEqual(
    locations,
    pages.map((file) => base + (file === "index.html" ? "" : file)),
  );
});

test("downloads work without JavaScript and AI references do not advertise an available Pro product", () => {
  const html = read("index.html");
  const downloads = [...html.matchAll(/<a\b[^>]*>/g)]
    .filter(([tag]) => /class="[^"]*\bdownload-button\b/.test(tag))
    .map(([tag]) => tag.match(/href="([^"]+)"/));
  assert.equal(downloads.length, 9);
  for (const link of downloads)
    assert.equal(
      link[1],
      JSON.parse(read("release.json")).assets.find(
        (asset) => asset.browser_download_url === link[1],
      )?.browser_download_url,
    );
  for (const file of [
    "index.md",
    "install.md",
    "desktop-vs-cli.md",
    "llms.txt",
  ]) {
    const content = read(file);
    assert.ok(/Pro/.test(content), file);
    assert.ok(/in development|not available|not announced/.test(content), file);
    for (const match of content.matchAll(
      /https:\/\/abdenasser\.github\.io\/neohtop\/([^\s)]+)/g,
    )) {
      assert.ok(
        fs.existsSync(path.join(root, match[1])),
        `${file}: missing ${match[1]}`,
      );
    }
  }
  assert.ok(fs.existsSync(path.join(root, ".nojekyll")));
});

test("published downloads and their version are available before JavaScript executes", () => {
  const snapshot = JSON.parse(read("release.json"));
  assert.equal(snapshot.draft, false);
  assert.equal(snapshot.prerelease, false);
  const html = read("index.html");
  assert.ok(html.includes(`${snapshot.tag_name} · current release`));
  const downloads = [...html.matchAll(/<a\b[^>]*>/g)]
    .map(([tag]) => tag)
    .filter((tag) => /class="[^"]*\bdownload-button\b/.test(tag));
  assert.equal(downloads.length, 9);
  for (const tag of downloads) {
    assert.ok(tag.includes(`data-version="${snapshot.tag_name}"`));
    assert.ok(tag.includes(`/releases/download/${snapshot.tag_name}/`));
  }
  for (const file of pages) {
    const script = read(file).match(
      /<script type="module" src="(assets\/site-runtime-[A-Za-z0-9_-]+\.js)"/,
    );
    assert.ok(script, `${file}: runtime must use a content hash`);
    assert.ok(fs.existsSync(path.join(root, script[1])));
  }
});
