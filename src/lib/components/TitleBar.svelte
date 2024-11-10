<script lang="ts">
  import { Window } from "@tauri-apps/api/window";

  const appWindow = new Window("main");
  let isMaximized = false;

  async function toggleMaximize() {
    isMaximized = await appWindow.isMaximized();
    if (isMaximized) {
      await appWindow.unmaximize();
    } else {
      await appWindow.maximize();
    }
    isMaximized = await appWindow.isMaximized();
  }
</script>

<div class="title-bar" data-tauri-drag-region>
  <div class="title">
    <div class="neon">NeoHtop</div>
    <div class="scanlines"></div>
  </div>
</div>

<style>
  .title-bar {
    height: 32px;
    /* background: var(--mantle); */
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0 12px;
    -webkit-user-select: none;
    user-select: none;
    position: relative;
    overflow: hidden;
  }

  .title {
    display: flex;
    align-items: center;
    position: relative;
    height: 100%;
  }

  .neon {
    font-family: "Courier New", monospace;
    font-size: 16px;
    font-weight: bold;
    color: var(--text);
    text-shadow:
      0 0 5px var(--text),
      0 0 10px var(--text),
      0 0 20px var(--blue),
      0 0 40px var(--blue),
      0 0 80px var(--blue);
    animation: flicker 3s infinite alternate;
  }

  @keyframes flicker {
    0%,
    19.999%,
    22%,
    62.999%,
    64%,
    64.999%,
    70%,
    100% {
      opacity: 1;
      text-shadow:
        0 0 5px var(--text),
        0 0 10px var(--text),
        0 0 20px var(--blue),
        0 0 40px var(--blue),
        0 0 80px var(--blue);
    }
    20%,
    21.999%,
    63%,
    63.999%,
    65%,
    69.999% {
      opacity: 0.4;
      text-shadow: none;
    }
  }

  .scanlines {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: linear-gradient(
      to bottom,
      transparent 50%,
      rgba(0, 0, 0, 0.2) 51%
    );
    background-size: 100% 4px;
    animation: scanlines 0.6s steps(40) infinite;
    pointer-events: none;
  }

  @keyframes scanlines {
    from {
      transform: translateY(0);
    }
    to {
      transform: translateY(2px);
    }
  }
</style>
