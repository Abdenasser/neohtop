import { defineConfig } from "vite";
import { svelte, vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";
import { fileURLToPath } from "node:url";
export default defineConfig({
  plugins: [
    tailwindcss(),
    svelte({ configFile: false, preprocess: vitePreprocess() }),
  ],
  resolve: {
    alias: { $lib: fileURLToPath(new URL("../src/lib", import.meta.url)) },
  },
  ssr: { noExternal: ["@lucide/svelte"] },
  build: {
    ssr: "site/entry-server.js",
    outDir: ".site-build",
    emptyOutDir: true,
    ssrEmitAssets: true,
    cssCodeSplit: false,
    rollupOptions: {
      output: {
        entryFileNames: "entry-server.mjs",
        assetFileNames: "assets/site-[name]-[hash][extname]",
      },
    },
  },
});
