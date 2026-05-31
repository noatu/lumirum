import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      "/auth": "http://localhost:3000",
      "/devices": "http://localhost:3000",
      "/profiles": "http://localhost:3000",
      "/telemetry": "http://localhost:3000",
      "/users": "http://localhost:3000",
      "/health": "http://localhost:3000",
      "/stats": "http://localhost:3000",
    },
  },
});
