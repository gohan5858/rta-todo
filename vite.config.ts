import vue from "@vitejs/plugin-vue";
import { join } from 'node:path';
import { defineConfig } from "vite";

const PACKAGE_ROOT = __dirname;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],

  resolve: {
    alias: {
      '@base': join(PACKAGE_ROOT, 'src/components/base') + '/',
      '@layout': join(PACKAGE_ROOT, 'src/components/layout') + '/',
      '@page': join(PACKAGE_ROOT, 'src/components/page') + '/',
      '@assets': join(PACKAGE_ROOT, 'assets') + '/',
      '@': join(PACKAGE_ROOT, 'src') + '/',
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
