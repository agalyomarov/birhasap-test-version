// https://nuxt.com/docs/api/configuration/nuxt-config
import tailwindcss from "@tailwindcss/vite";

export default defineNuxtConfig({
  compatibilityDate: "2025-07-15",
  devtools: { enabled: true },
  ssr: false,
  telemetry: false,
  css: ["./app/assets/css/main.css", "./app/assets/css/font-roboto.css"],
  app: {
    head: {
      script: [
        {
          src: "http://localhost:8098",
          defer: true,
        },
      ],
    },
  },
  vite: {
    plugins: [tailwindcss()],
    // Better support for Tauri CLI output
    clearScreen: false,
    // Enable environment variables
    // Additional environment variables can be found at
    // https://v2.tauri.app/reference/environment-variables/
    envPrefix: ["VITE_", "TAURI_"],
    server: {
      // Tauri requires a consistent port
      strictPort: true,
      watch: {
        ignored: ["**/src-tauri/**"],
      },
    },
    optimizeDeps: {
      include: ["@tauri-apps/api/core", "@tauri-apps/api/window", "@vue/devtools-core", "@vue/devtools-kit"],
    },
  },
  // Avoids error [unhandledRejection] EMFILE: too many open files, watch
  ignore: ["**/src-tauri/**"],
});
