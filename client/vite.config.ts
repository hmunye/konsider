import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import { TanStackRouterVite } from "@tanstack/router-vite-plugin";

export default defineConfig({
  plugins: [react(), TanStackRouterVite()],

  server: {
    host: "127.0.0.1",
    port: 3000,
    strictPort: true,
  },
});
