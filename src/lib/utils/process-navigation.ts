export function processButton(target: EventTarget | null) {
  return target &&
    target instanceof HTMLButtonElement &&
    target.hasAttribute("data-process-navigation")
    ? target
    : null;
}

export function moveProcessFocus(event: KeyboardEvent, region: HTMLElement) {
  if (
    event.defaultPrevented ||
    event.isComposing ||
    event.altKey ||
    event.ctrlKey ||
    event.metaKey ||
    event.shiftKey ||
    (event.key !== "ArrowUp" && event.key !== "ArrowDown")
  ) {
    return false;
  }

  const current = processButton(event.target);
  if (!current && event.target !== region) return false;

  const buttons = Array.from(
    region.querySelectorAll<HTMLButtonElement>(
      "button[data-process-navigation]",
    ),
  );
  if (!buttons.length) return false;

  const down = event.key === "ArrowDown";
  const index = current ? buttons.indexOf(current) : down ? -1 : buttons.length;
  const next = Math.max(
    0,
    Math.min(buttons.length - 1, index + (down ? 1 : -1)),
  );
  event.preventDefault();
  buttons[next].focus();
  return true;
}

export function preserveProcessFocus(region: HTMLElement) {
  const document = region.ownerDocument;
  let focused: HTMLButtonElement | null = null;

  function remember(event: FocusEvent) {
    if (event.target === document.body) return;
    const button = processButton(event.target);
    focused = button && region.contains(button) ? button : null;
  }

  function clearOnOutsideClick(event: PointerEvent) {
    if (event.target instanceof Node && !region.contains(event.target))
      focused = null;
  }

  const observer = new MutationObserver(() => {
    if (
      !focused ||
      document.activeElement !== document.body ||
      !document.hasFocus()
    )
      return;
    // A DOM move can blur a keyed button. Restore it if it still represents the
    // same process, or return to the region if the process disappeared.
    (region.contains(focused) ? focused : region).focus({
      preventScroll: true,
    });
  });
  observer.observe(region, { childList: true, subtree: true });
  document.addEventListener("focusin", remember);
  document.addEventListener("pointerdown", clearOnOutsideClick);
  return {
    destroy() {
      observer.disconnect();
      document.removeEventListener("focusin", remember);
      document.removeEventListener("pointerdown", clearOnOutsideClick);
    },
  };
}
