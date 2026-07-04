// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
   compatibilityDate: "2025-07-15",
   devtools: { enabled: true },
   ssr: false,
   telemetry: false,
   vite: {
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
         include: ["@tauri-apps/api/core", "@vue/devtools-core", "@vue/devtools-kit"],
      },
   },
   // Avoids error [unhandledRejection] EMFILE: too many open files, watch
   ignore: ["**/src-tauri/**"],
});
