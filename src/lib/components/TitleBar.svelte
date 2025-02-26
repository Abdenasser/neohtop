<script lang="ts">
</script>

<div class="title-bar" data-tauri-drag-region>
  <div class="title">
    <img src="/32x32.png" alt="NeoHtop" class="app-icon" />
    <div class="neon">NeoHtop</div>
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
    font-size: 14px;
    font-weight: bold;
    color: var(--text);
  }

  .app-icon {
    width: 24px;
    height: 24px;
    margin-right: 4px;
    display: flex;
    align-items: center;
  }
</style>
