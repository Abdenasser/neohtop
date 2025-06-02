<script lang="ts">
  import { overlayStore } from "$lib/stores/overlay";
  import { onDestroy, onMount } from "svelte";

  export let searchTerm: string;

  let containerElement: HTMLDivElement;
  let overlayElement: HTMLDivElement;
  let searchInputElement: HTMLInputElement;
  let placeholderIndex = 0;
  let placeholderInterval: NodeJS.Timeout;

  $: showHelp = $overlayStore === "searchHelp";
  $: hasActiveSearch = searchTerm.trim().length > 0;

  const searchExamples = [
    {
      query: "systemd, dbus",
      description: "Multiple terms (comma-separated)",
      type: "multi",
    },
    {
      query: "d$",
      description: "Processes ending with 'd' (daemons)",
      type: "regex",
    },
    {
      query: "^kernel",
      description: "Kernel processes",
      type: "regex",
    },
    {
      query: "ssh.*server",
      description: "SSH server processes",
      type: "regex",
    },
    {
      query: "1234",
      description: "Search by PID",
      type: "pid",
    },
    {
      query: "python, node, nginx",
      description: "Find web/app server processes",
      type: "multi",
    },
    {
      query: "docker, containerd",
      description: "Container processes",
      type: "multi",
    },
    {
      query: "gnome, plasma",
      description: "Desktop environment processes",
      type: "multi",
    },
  ];

  const placeholders = [
    "Search processes...",
    "Try: systemd, dbus",
    "Try: d$ (daemons)",
    "Try: ^kernel (regex)",
    "Search by name, command, or PID",
    "Try: docker, nginx",
  ];

  function rotatePlaceholder() {
    placeholderIndex = (placeholderIndex + 1) % placeholders.length;
  }

  function updateOverlayPosition() {
    if (overlayElement && containerElement) {
      const toolbarContent = containerElement.closest(".toolbar-content");
      if (toolbarContent) {
        const toolbarRect = toolbarContent.getBoundingClientRect();
        const containerRect = containerElement.getBoundingClientRect();

        const leftOffset = containerRect.left - toolbarRect.left;
        const rightOffset = toolbarRect.right - containerRect.right;
        const topOffset = containerRect.top - toolbarRect.top;

        overlayElement.style.left = `-${leftOffset}px`;
        overlayElement.style.right = `-${rightOffset}px`;
        overlayElement.style.top = `-${topOffset}px`;
      }
    }
  }

  function handleFocus() {
    overlayStore.open("searchHelp");
    setTimeout(() => {
      updateOverlayPosition();
      // Focus the enhanced search input in the overlay
      const overlayInput = overlayElement?.querySelector(
        ".overlay-search-input",
      ) as HTMLInputElement;
      if (overlayInput) {
        overlayInput.focus();
      }
    }, 0);
  }

  function handleBlur(event: FocusEvent) {
    // Small delay to allow clicking on examples
    setTimeout(() => {
      const activeElement = document.activeElement;
      if (!containerElement?.contains(activeElement as Node)) {
        overlayStore.close();
      }
    }, 150);
  }

  function useExample(example: string) {
    searchTerm = example;
    // Keep focus on the overlay search input
    const overlayInput = overlayElement?.querySelector(
      ".overlay-search-input",
    ) as HTMLInputElement;
    if (overlayInput) {
      overlayInput.focus();
      // Position cursor at end
      setTimeout(() => {
        overlayInput.setSelectionRange(example.length, example.length);
      }, 0);
    }
  }

  function handleClickOutside(event: MouseEvent) {
    if (
      showHelp &&
      containerElement &&
      !containerElement.contains(event.target as Node)
    ) {
      overlayStore.close();
    }
  }

  function setupClickOutside() {
    if (typeof document !== "undefined") {
      document.addEventListener("click", handleClickOutside);
    }
  }

  function cleanupClickOutside() {
    if (typeof document !== "undefined") {
      document.removeEventListener("click", handleClickOutside);
    }
  }

  $: if (showHelp) {
    setTimeout(setupClickOutside, 0);
  } else {
    cleanupClickOutside();
  }

  onMount(() => {
    // Start placeholder rotation
    placeholderInterval = setInterval(rotatePlaceholder, 3000);
  });

  onDestroy(() => {
    cleanupClickOutside();
    if (placeholderInterval) {
      clearInterval(placeholderInterval);
    }
  });
</script>

<div class="search-box" bind:this={containerElement}>
  <div class="search-input-wrapper">
    <input
      type="text"
      placeholder={placeholders[placeholderIndex]}
      bind:value={searchTerm}
      bind:this={searchInputElement}
      class="search-input"
      class:has-search={hasActiveSearch}
      on:focus={handleFocus}
      on:blur={handleBlur}
    />
    {#if searchTerm}
      <button class="btn-clear" on:click={() => (searchTerm = "")}>
        Clear
      </button>
    {/if}
  </div>

  {#if showHelp}
    <div
      class="touchbar-full-overlay"
      bind:this={overlayElement}
      role="dialog"
      aria-label="Search help overlay"
    >
      <div class="search-help-content">
        <div class="enhanced-search-input">
          <input
            type="text"
            placeholder={placeholders[placeholderIndex]}
            bind:value={searchTerm}
            bind:this={searchInputElement}
            class="overlay-search-input"
            on:blur={handleBlur}
            autocomplete="off"
          />
          {#if searchTerm}
            <button
              class="overlay-clear-btn"
              on:click={() => (searchTerm = "")}
            >
              Clear
            </button>
          {/if}
        </div>

        <div class="help-sections">
          <div class="examples-section">
            <span class="section-label">Examples:</span>
            <div class="examples-grid">
              {#each searchExamples.slice(0, 5) as example}
                <button
                  class="example-query"
                  class:regex={example.type === "regex"}
                  class:multi={example.type === "multi"}
                  class:pid={example.type === "pid"}
                  on:click|stopPropagation={() => useExample(example.query)}
                  title={example.description}
                >
                  {example.query}
                </button>
              {/each}
            </div>
          </div>

          <div class="regex-section">
            <span class="section-label">Regex:</span>
            <div class="regex-tips">
              <code>^</code><span>start</span>
              <code>$</code><span>end</span>
              <code>.*</code><span>any</span>
              <code>\d+</code><span>numbers</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .search-box {
    width: 240px;
    position: relative;
  }

  .search-input-wrapper {
    position: relative;
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .search-input {
    width: 100%;
    height: 28px;
    padding: 0 12px;
    padding-right: 70px;
    border: 1px solid var(--surface1);
    border-radius: 6px;
    font-size: 12px;
    background-color: var(--surface0);
    color: var(--text);
    transition: all 0.2s ease;
    box-sizing: border-box;
  }

  .btn-clear {
    position: absolute;
    right: 4px;
    top: 50%;
    transform: translateY(-50%);
    padding: 4px 8px;
    font-size: 11px;
    color: var(--subtext0);
    background: var(--surface1);
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-clear:hover {
    background: var(--surface2);
    color: var(--text);
  }

  .search-input:hover {
    background-color: var(--surface1);
  }

  .search-input:focus {
    outline: none;
    border-color: var(--blue);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--blue) 25%, transparent);
  }

  .search-input.has-search {
    border-color: var(--blue);
    background: var(--surface1);
  }

  .touchbar-full-overlay {
    position: absolute;
    top: -0px;
    height: 44px;
    background: var(--mantle);
    border: none;
    border-radius: 0;
    box-shadow: none;
    z-index: 1000;
    display: flex;
    align-items: center;
    padding: 0 12px;
    gap: 12px;
    overflow-x: auto;
    overflow-y: hidden;
    scrollbar-width: none;
    -ms-overflow-style: none;
  }

  .touchbar-full-overlay::-webkit-scrollbar {
    display: none;
  }

  .search-help-content {
    display: flex;
    align-items: center;
    gap: 24px;
    width: 100%;
    animation: slideIn 0.3s ease-out;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(-8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .enhanced-search-input {
    position: relative;
    display: flex;
    align-items: center;
    flex-shrink: 0;
    min-width: 300px;
  }

  .overlay-search-input {
    width: 100%;
    height: 32px;
    padding: 0 16px;
    padding-right: 80px;
    border: 2px solid var(--blue);
    border-radius: 8px;
    font-size: 13px;
    background-color: var(--surface0);
    color: var(--text);
    transition: all 0.2s ease;
    box-sizing: border-box;
    font-weight: 500;
  }

  .overlay-search-input:focus {
    outline: none;
    border-color: var(--blue);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--blue) 20%, transparent);
  }

  .overlay-clear-btn {
    position: absolute;
    right: 8px;
    top: 50%;
    transform: translateY(-50%);
    padding: 4px 10px;
    font-size: 11px;
    color: var(--subtext0);
    background: var(--surface1);
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .overlay-clear-btn:hover {
    background: var(--surface2);
    color: var(--text);
  }

  .help-sections {
    display: flex;
    align-items: center;
    gap: 32px;
    flex: 1;
  }

  .examples-section {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-shrink: 0;
  }

  .section-label {
    font-size: 12px;
    font-weight: 500;
    color: var(--subtext0);
    flex-shrink: 0;
  }

  .examples-grid {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .example-query {
    padding: 4px 10px;
    height: 26px;
    font-size: 11px;
    font-family: "SF Mono", "Monaco", "Inconsolata", "Fira Code", monospace;
    color: var(--text);
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.15s ease;
    white-space: nowrap;
    display: flex;
    align-items: center;
    box-sizing: border-box;
  }

  .example-query:hover {
    background: var(--surface1);
    border-color: var(--blue);
    transform: translateY(-1px);
  }

  .example-query.regex {
    border-color: var(--yellow);
    color: var(--yellow);
  }

  .example-query.multi {
    border-color: var(--green);
    color: var(--green);
  }

  .example-query.pid {
    border-color: var(--blue);
    color: var(--blue);
  }

  .regex-section {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-shrink: 0;
  }

  .regex-tips {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .regex-tips code {
    font-size: 11px;
    font-family: "SF Mono", "Monaco", "Inconsolata", "Fira Code", monospace;
    background: var(--surface0);
    color: var(--yellow);
    padding: 3px 6px;
    border-radius: 3px;
    border: 1px solid var(--surface1);
    margin-right: 2px;
  }

  .regex-tips span {
    font-size: 10px;
    color: var(--overlay0);
    margin-right: 8px;
  }
</style>
