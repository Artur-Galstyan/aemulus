import { defineConfig } from "vite";

export default defineConfig({
  optimizeDeps: { exclude: ["@mujoco/mujoco"] },
  server: {
    proxy: { "/aemulus.sim.v1.SimService": "http://127.0.0.1:50051" },
  },
});
