import { resolveDownloadAssets } from "./downloads.js";

document.documentElement.classList.add("js");
const menu = document.querySelector(".menu-button");
const links = document.querySelector(".nav-links");
function closeMenu() {
  links?.classList.remove("is-open");
  menu?.setAttribute("aria-expanded", "false");
}
menu?.addEventListener("click", () => {
  const open = menu.getAttribute("aria-expanded") !== "true";
  menu.setAttribute("aria-expanded", String(open));
  links?.classList.toggle("is-open", open);
});
links?.addEventListener("click", (event) => {
  if (event.target.closest("a")) closeMenu();
});
document.addEventListener("keydown", (event) => {
  if (
    event.key === "Escape" &&
    menu?.getAttribute("aria-expanded") === "true"
  ) {
    closeMenu();
    menu.focus();
  }
});
document.addEventListener("click", (event) => {
  if (!event.target.closest(".navigation")) closeMenu();
});

for (const button of document.querySelectorAll("[data-copy]")) {
  button.addEventListener("click", async () => {
    const command = document.getElementById(button.dataset.copy)?.textContent;
    const status = document.getElementById("copy-status");
    if (!command) return;
    try {
      await navigator.clipboard.writeText(command);
      button.textContent = "Copied";
      if (status) status.textContent = "Command copied to clipboard.";
      setTimeout(() => {
        button.textContent = "Copy";
      }, 2000);
    } catch {
      if (status)
        status.textContent =
          "Copy unavailable. Select and copy the command manually.";
      const selection = window.getSelection();
      const range = document.createRange();
      range.selectNodeContents(document.getElementById(button.dataset.copy));
      selection?.removeAllRanges();
      selection?.addRange(range);
    }
  });
}

// Only published GitHub releases can provide download links. Static links remain
// useful without JavaScript, with a blocked API, or while GitHub is rate limited.
async function updateRelease() {
  const version = document.getElementById("current-version");
  const status = document.getElementById("release-status");
  if (!version) return;
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), 8000);
  try {
    const response = await fetch(
      "https://api.github.com/repos/Abdenasser/neohtop/releases/latest",
      {
        signal: controller.signal,
        headers: { Accept: "application/vnd.github+json" },
      },
    );
    if (!response.ok) throw new Error("Release lookup unavailable");
    const release = await response.json();
    if (
      release.draft ||
      release.prerelease ||
      !/^v\d+\.\d+\.\d+$/.test(release.tag_name)
    ) {
      throw new Error("Not a published stable release");
    }
    const assets = resolveDownloadAssets(release);
    if (!Object.keys(assets).length)
      throw new Error("No recognized release assets");
    version.textContent = `${release.tag_name} · published release`;
    for (const link of document.querySelectorAll(
      ".download-button[data-type]",
    )) {
      const url = assets[link.dataset.type];
      if (url) {
        link.href = url;
        link.dataset.version = release.tag_name;
      } else {
        link.removeAttribute("href");
        link.setAttribute("aria-disabled", "true");
        link.tabIndex = -1;
        link.querySelector(".secondary").textContent = "Not in this release";
      }
    }
    if (status)
      status.textContent =
        "Direct links use the assets in this published release. Development features may not be included.";
  } catch {
    if (status)
      status.textContent =
        "Live release lookup is unavailable. Download links open GitHub Releases so you can choose an available package.";
  } finally {
    clearTimeout(timeout);
  }
}
updateRelease();

// Preserve the site's existing analytics account. Loading or blocking analytics
// never changes navigation, copy controls, or downloads.
if (
  /^https?:$/.test(location.protocol) &&
  location.hostname !== "localhost" &&
  location.hostname !== "127.0.0.1"
) {
  window.dataLayer = window.dataLayer || [];
  window.gtag = function () {
    window.dataLayer.push(arguments);
  };
  window.gtag("js", new Date());
  window.gtag("config", "G-88HFXRBNLS");
  const analytics = document.createElement("script");
  analytics.async = true;
  analytics.src = "https://www.googletagmanager.com/gtag/js?id=G-88HFXRBNLS";
  document.head.append(analytics);
}
for (const link of document.querySelectorAll(".download-button")) {
  link.addEventListener("click", (event) => {
    if (link.getAttribute("aria-disabled") === "true") {
      event.preventDefault();
      return;
    }
    if (typeof window.gtag === "function") {
      window.gtag("event", "download", {
        event_category: "App",
        event_label: link.dataset.type,
        value: link.dataset.version || "release-page",
      });
    }
  });
}
