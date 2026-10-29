<script lang="ts">
  import { onDestroy } from "svelte";
  import { siGhostery } from "simple-icons";
  import { themeStore } from "$lib/stores/theme";
  import { getNativeProcessIcon } from "$lib/stores/native-icons";
  export let pid: number | undefined = undefined;
  export let startTime: number | undefined = undefined;
  export let size: number = 16;
  let nativeIcon: string | null = null;
  let generation = 0;
  async function load(pid: number | undefined, start: number | undefined) {
    const current = ++generation;
    nativeIcon = null;
    if (
      pid === undefined ||
      start === undefined ||
      typeof window === "undefined"
    )
      return;
    const icon = await getNativeProcessIcon(pid, start);
    // Recycled rows and closed details must not receive an earlier process's icon.
    if (current === generation) nativeIcon = icon;
  }
  $: load(pid, startTime);
  $: fallback = `data:image/svg+xml,${encodeURIComponent(siGhostery.svg.replace("<svg", `<svg fill="${$themeStore.colors.text}"`))}`;
  $: icon = nativeIcon ?? fallback;
  onDestroy(() => {
    generation++;
  });
</script>

<img
  class="shrink-0 object-contain"
  style={`width:${size}px;height:${size}px`}
  src={icon}
  alt=""
  aria-hidden="true"
  height={size}
  width={size}
  draggable="false"
  on:error={() => {
    if (nativeIcon) nativeIcon = null;
  }}
/>
