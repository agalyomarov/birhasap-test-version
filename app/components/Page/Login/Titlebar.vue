<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import titlebarLogoImg from "~/assets/images/titlebar-logo.png";

const handleCloseWindow = async () => {
  const appWindow = getCurrentWindow();
  await appWindow.close();
};
</script>
<template>
  <header
    data-tauri-drag-region
    class="flex items-center select-none justify-between"
  >
    <div
      data-tauri-drag-region
      class="flex items-center gap-2 cursor-default"
    >
      <img
        data-tauri-drag-region
        class="size-5"
        :src="titlebarLogoImg"
        alt=""
      />
      <span
        data-tauri-drag-region
        class="text-color-black text-text-windiw-title"
      >
        <slot />
      </span>
    </div>
    <button
      class="hover:bg-[#ececec] cursor-pointer outline-0"
      @click="handleCloseWindow()"
    >
      <svg
        width="22"
        height="22"
        viewBox="0 0 17 17"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <line
          x1="0.5"
          y1="-0.5"
          x2="12.2924"
          y2="-0.5"
          transform="matrix(-0.703545 0.710651 -0.703545 -0.710651 13 3)"
          stroke="black"
          stroke-linecap="round"
        />
        <line
          x1="0.5"
          y1="-0.5"
          x2="12.2924"
          y2="-0.5"
          transform="matrix(0.703545 0.710651 -0.703545 0.710651 4 4.09082)"
          stroke="black"
          stroke-linecap="round"
        />
      </svg>
    </button>
  </header>
</template>

<style scoped></style>
