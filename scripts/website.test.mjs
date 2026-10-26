import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../docs/", import.meta.url));
const base = "https://abdenasser.github.io/neohtop/";
const pages = ["index.html", "install.html", "desktop-vs-cli.html"];
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
  const downloads = [
    ...html.matchAll(/<a\s+class="download-button"[\s\S]*?href="([^"]+)"/g),
  ];
  assert.equal(downloads.length, 9);
  for (const link of downloads)
    assert.equal(
      link[1],
      "https://github.com/Abdenasser/neohtop/releases/latest",
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
