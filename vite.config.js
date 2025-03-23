import { defineConfig } from "vite";

export default defineConfig({
  build: {
    target: "esnext",
    assetsInlineLimit: 0,
    rollupOptions: {
      output: {
        assetFileNames: "[name][extname]",
      },
    },
  }
});
