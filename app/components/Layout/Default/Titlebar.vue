<script setup lang="ts">
import titlebarLogoImg from "~/assets/images/titlebar-logo.png";
import { getCurrentWindow, currentMonitor, LogicalSize } from "@tauri-apps/api/window";
const appWindow = getCurrentWindow();
const isMaximized = ref(false);
const isFullScreen = ref(false);
const updateMaximized = async () => {
  isMaximized.value = await appWindow.isMaximized();
};
const setMinSize = async () => {
  await appWindow.setSize(new LogicalSize(1200, 800));
  await appWindow.center();
  await updateMaximized();
};

const setWorkspaceSize = async () => {
  const monitor = await currentMonitor();
  const size = monitor?.workArea.size;
  const factor = monitor?.scaleFactor;
  if (size && factor) {
    await appWindow.setSize(new LogicalSize(size.width / factor, size.height / factor));
    await appWindow.center();
    isMaximized.value = true;
  }
};

const setFullScreen = async (val: boolean) => {
  isFullScreen.value = val;
  if (val) {
    await setWorkspaceSize();
    await appWindow.setSimpleFullscreen(val);
  } else {
    await appWindow.setSimpleFullscreen(val);
    await setWorkspaceSize();
  }
};

onMounted(async () => {
  await updateMaximized();
  appWindow.onResized(async () => {
    await updateMaximized();
  });
});
</script>
<template>
  <header
    data-tauri-drag-region
    class="cursor-default select-none bg-[#F9EDA7] px-2 py-1 flex items-center justify-between"
  >
    <div class="flex items-center text-base gap-2">
      <div>
        <img
          class="size-6"
          :src="titlebarLogoImg"
          alt=""
        />
      </div>
      <p>Birhasap - kassa programma / Administrator</p>
    </div>
    <div class="flex items-center gap-4">
      <button
        class="hover:bg-[#F4CC4C] cursor-pointer outline-0"
        @click="appWindow.minimize()"
      >
        <svg
          width="22"
          height="22"
          viewBox="0 0 17 17"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <line
            x1="4"
            y1="12.5"
            x2="13"
            y2="12.5"
            stroke="black"
          />
        </svg>
      </button>
      <template v-if="!isFullScreen">
        <button
          v-if="isMaximized"
          class="hover:bg-[#F4CC4C] cursor-pointer outline-0"
          @click="setMinSize()"
        >
          <svg
            width="22"
            height="22"
            viewBox="0 0 17 17"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <rect
              x="4.5"
              y="6.5"
              width="6"
              height="6"
              stroke="black"
            />
            <path
              d="M13 11H11V10H12V5H7V6H6V4H13V11Z"
              fill="black"
            />
          </svg>
        </button>
        <button
          v-else
          class="hover:bg-[#F4CC4C] cursor-pointer outline-0"
          @click="setWorkspaceSize()"
        >
          <svg
            width="22"
            height="22"
            viewBox="0 0 17 17"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <rect
              x="4.5"
              y="4.5"
              width="8"
              height="8"
              stroke="black"
            />
          </svg>
        </button>
      </template>
      <button
        class="hover:bg-[#F4CC4C] cursor-pointer outline-0 p-1"
        @click="setFullScreen(!isFullScreen)"
      >
        <svg
          width="12"
          height="12"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <g
            id="SVGRepo_bgCarrier"
            stroke-width="0"
          ></g>
          <g
            id="SVGRepo_tracerCarrier"
            stroke-linecap="round"
            stroke-linejoin="round"
          ></g>
          <g id="SVGRepo_iconCarrier">
            <g id="style=linear">
              <g id="fullscreen">
                <path
                  id="vector"
                  d="M8 2H4C2.89543 2 2 2.89543 2 4V8"
                  stroke="#000000"
                  stroke-width="1.5"
                  stroke-linecap="round"
                ></path>
                <path
                  id="vector_2"
                  d="M22 8L22 4C22 2.89543 21.1046 2 20 2H16"
                  stroke="#000000"
                  stroke-width="1.5"
                  stroke-linecap="round"
                ></path>
                <path
                  id="vector_3"
                  d="M16 22L20 22C21.1046 22 22 21.1046 22 20L22 16"
                  stroke="#000000"
                  stroke-width="1.5"
                  stroke-linecap="round"
                ></path>
                <path
                  id="vector_4"
                  d="M8 22L4 22C2.89543 22 2 21.1046 2 20V16"
                  stroke="#000000"
                  stroke-width="1.5"
                  stroke-linecap="round"
                ></path>
              </g>
            </g>
          </g>
        </svg>
      </button>
      <button
        class="hover:bg-[#F4CC4C] cursor-pointer outline-0"
        @click="appWindow.close()"
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
    </div>
  </header>
</template>

<style scoped></style>
