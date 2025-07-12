import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import tailwindPlugin from "@tailwindcss/vite";

const __dirname = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  build: {
    rollupOptions: {
      input: {
        crosshair: resolve(__dirname, "./crosshair.html"),
        settings: resolve(__dirname, "./settings.html"),
      },
    },
  },
  plugins: [tailwindPlugin()],
});
