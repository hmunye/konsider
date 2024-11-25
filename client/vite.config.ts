import fs from "node:fs";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [sveltekit()],
  server: {
    host: "localhost",
    port: 3030,
    strictPort: true,
    https: {
      key: fs.readFileSync("./certs/server.key"),
      cert: fs.readFileSync("./certs/server.crt"),
    },
    proxy: {},
  },
});
