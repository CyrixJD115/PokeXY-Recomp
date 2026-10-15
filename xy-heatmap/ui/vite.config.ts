import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// Dev: the Rust server owns /api and /view; Vite proxies them so the UI
// can be developed against `cargo run -- serve`.
export default defineConfig({
  plugins: [svelte()],
  server: {
    proxy: {
      "/api": "http://127.0.0.1:8765",
      "/view": "http://127.0.0.1:8765",
    },
  },
  build: {
    target: "es2022",
  },
});
