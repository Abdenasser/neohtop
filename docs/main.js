import { resolveDownloadAssets, selectPublishedRelease } from "./downloads.js";
import { refreshCommunityStats } from "./community.js";
import releaseSnapshot from "./release.json" with { type: "json" };

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

// The HTML already contains verified direct downloads. Live lookup can advance
// them to a newer stable release, but cannot downgrade them or break offline use.
let currentRelease = releaseSnapshot;
let releaseRequestInFlight = false;
let releaseCheckedAt = 0;
function renderRelease(release) {
  const assets = resolveDownloadAssets(release);
  const version = document.getElementById("current-version");
  if (version) version.textContent = `${release.tag_name} · current release`;
  for (const link of document.querySelectorAll(".download-button[data-type]")) {
    const url = assets[link.dataset.type];
    const detail = link.querySelector(".secondary");
    if (url) {
      link.href = url;
      link.dataset.version = release.tag_name;
      link.removeAttribute("aria-disabled");
      link.removeAttribute("tabindex");
      if (detail)
        detail.textContent = `${link.dataset.format} · ${release.tag_name}`;
    } else {
      link.removeAttribute("href");
      link.setAttribute("aria-disabled", "true");
      link.tabIndex = -1;
      if (detail) detail.textContent = `Unavailable in ${release.tag_name}`;
    }
  }
  for (const link of document.querySelectorAll("[data-release-notes]"))
    link.href = release.html_url;
  const checksum = release.assets.find((asset) => asset.name === "SHA256SUMS");
  for (const link of document.querySelectorAll("[data-release-checksums]"))
    link.href = checksum?.browser_download_url || release.html_url;
}
async function updateRelease() {
  if (
    !document.getElementById("current-version") ||
    releaseRequestInFlight ||
    Date.now() - releaseCheckedAt < 60000
  )
    return;
  const status = document.getElementById("release-status");
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), 8000);
  releaseRequestInFlight = true;
  try {
    const response = await fetch(
      "https://api.github.com/repos/Abdenasser/neohtop/releases/latest",
      {
        cache: "no-store",
        signal: controller.signal,
        headers: { Accept: "application/vnd.github+json" },
      },
    );
    if (!response.ok) throw new Error("Release lookup unavailable");
    currentRelease = selectPublishedRelease(
      await response.json(),
      currentRelease,
    );
    renderRelease(currentRelease);
    if (status)
      status.textContent = `Official ${currentRelease.tag_name} installers. Choose your operating system and architecture.`;
  } catch {
    if (status)
      status.textContent = `${currentRelease.tag_name} downloads are ready. Live release lookup is unavailable; these verified links still work.`;
  } finally {
    clearTimeout(timeout);
    releaseCheckedAt = Date.now();
    releaseRequestInFlight = false;
  }
}
renderRelease(currentRelease);
void updateRelease();
// Recheck when returning to a page left open before a new release was published.
document.addEventListener("visibilitychange", () => {
  if (document.visibilityState === "visible") void updateRelease();
});
window.addEventListener("pageshow", (event) => {
  if (event.persisted) void updateRelease();
});

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

// The static HTML already shows the checked snapshot. A failed source never
// clears it or prevents the other sources from refreshing.
if (document.querySelector("[data-community-stat]")) {
  void refreshCommunityStats()
    .then(({ stats }) => {
      const numbers = new Intl.NumberFormat("en");
      const dates = new Intl.DateTimeFormat("en", {
        month: "short",
        day: "numeric",
        year: "numeric",
        timeZone: "UTC",
      });
      for (const [source, stat] of Object.entries(stats)) {
        const value = document.querySelector(
          `[data-community-stat="${source}"]`,
        );
        const updated = document.querySelector(
          `[data-stat-updated="${source}"]`,
        );
        if (value) value.textContent = numbers.format(stat.value);
        if (updated) {
          updated.setAttribute("datetime", stat.updatedAt);
          updated.textContent = `Updated ${dates.format(new Date(stat.updatedAt))}`;
        }
      }
    })
    .catch(() => {});
}

// Progressive enhancement: the real desktop screenshot remains useful without
// JavaScript; the shadcn buttons let visitors inspect two more shipped screens.
const preview = document.getElementById("product-preview");
const previewControls = document.querySelector("[data-preview-controls]");
if (preview && previewControls) {
  previewControls.hidden = false;
  for (const button of previewControls.querySelectorAll("[data-preview-src]")) {
    button.addEventListener("click", () => {
      preview.src = button.dataset.previewSrc;
      preview.alt = button.dataset.previewAlt;
      for (const item of previewControls.querySelectorAll("[aria-pressed]"))
        item.setAttribute("aria-pressed", String(item === button));
      document.querySelector("[data-preview-caption-text]").textContent =
        button.dataset.previewCaption;
      document.querySelector("[data-preview-status]").textContent =
        `Showing ${button.textContent.trim()} screenshot.`;
    });
  }
}

// Local copies avoid third-party image requests. Keep a quiet initials fallback
// if an image is missing or blocked, including when the request fails early.
for (const avatar of document.querySelectorAll("[data-profile-avatar]")) {
  const fallback = () => {
    avatar.hidden = true;
  };
  avatar.addEventListener("error", fallback, { once: true });
  if (avatar.complete && avatar.naturalWidth === 0) fallback();
}

// Decorative animation is optional, respects the OS, and sleeps off screen.
const motionToggle = document.querySelector("[data-motion-toggle]");
if (motionToggle) {
  const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)");
  let motionChoice = null;
  try {
    const saved = localStorage.getItem("neohtop-site-motion");
    if (saved === "on" || saved === "off") motionChoice = saved;
  } catch {}
  function updateAmbientMotion() {
    const enabled =
      motionChoice === "on" ||
      (motionChoice !== "off" && !reducedMotion.matches);
    document.documentElement.dataset.motion = enabled ? "on" : "off";
    if (motionChoice === "on")
      document.documentElement.dataset.motionOverride = "on";
    else delete document.documentElement.dataset.motionOverride;
    motionToggle.hidden = false;
    motionToggle.setAttribute("aria-pressed", String(enabled));
    motionToggle.title = enabled
      ? "Pause ambient animation"
      : reducedMotion.matches && motionChoice === null
        ? "Enable motion for this site; your system reduced-motion setting stays unchanged"
        : "Resume ambient animation";
    motionToggle.querySelector("[data-motion-copy]").textContent = enabled
      ? "Motion on"
      : "Motion off";
    motionToggle.querySelector("[data-motion-playing]").hidden = !enabled;
    motionToggle.querySelector("[data-motion-paused]").hidden = enabled;
  }
  motionToggle.addEventListener("click", () => {
    motionChoice =
      document.documentElement.dataset.motion === "on" ? "off" : "on";
    try {
      localStorage.setItem("neohtop-site-motion", motionChoice);
    } catch {}
    updateAmbientMotion();
  });
  reducedMotion.addEventListener("change", updateAmbientMotion);
  function updatePageVisibility() {
    document.documentElement.classList.toggle(
      "page-hidden",
      document.visibilityState !== "visible",
    );
  }
  document.addEventListener("visibilitychange", updatePageVisibility);
  const animated = [
    ...document.querySelectorAll(
      "[data-ambient], .hero h1 span, .section-heading h2 span, .theme-copy h2 span",
    ),
  ];
  for (const element of animated)
    if (element.matches("h1 span, h2 span"))
      element.classList.add("animated-title");
  if ("IntersectionObserver" in window) {
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries)
          entry.target.classList.toggle("is-in-view", entry.isIntersecting);
      },
      { threshold: 0.05 },
    );
    for (const element of animated) observer.observe(element);
  } else for (const element of animated) element.classList.add("is-in-view");
  updatePageVisibility();
  updateAmbientMotion();
}
