import { defineConfig } from "vite";
import gleamPlugin from "vite-gleam";
import tailwindPlugin from "@tailwindcss/vite";

export default defineConfig({
  plugins: [gleamPlugin(), tailwindPlugin()],
});
