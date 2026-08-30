import { defineConfig } from 'vite';

export default defineConfig({
  base: "/", 
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});
