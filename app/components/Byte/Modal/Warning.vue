<script setup lang="ts">
import { ButtonType } from "~/enums/button-type";
import { ButtonUI } from "~/enums/button-ui";

const { isOpan, type, content, closeModal } = useModal();
</script>
<template>
  <div
    v-if="isOpan && type == 'warning'"
    class="fixed inset-0 flex items-center justify-center select-none"
  >
    <div class="p-4 w-fit max-w-85 z-10 bg-white border border-gray-a1 modal">
      <div class="flex justify-end">
        <button
          class="hover:bg-gray-ec cursor-pointer"
          @click="closeModal()"
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
      <div class="flex items-center gap-4">
        <div>
          <svg
            width="65"
            height="64"
            viewBox="0 0 65 64"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              fill-rule="evenodd"
              clip-rule="evenodd"
              d="M34 5L3 61H64.5L34 5ZM33.9857 9.15528L6.39314 59H61.1333L33.9857 9.15528Z"
              fill="#F38000"
            />
            <path
              d="M33 27H35V43H33V27Z"
              fill="#F38000"
            />
            <path
              d="M36 50C36 51.1046 35.1046 52 34 52C32.8954 52 32 51.1046 32 50C32 48.8954 32.8954 48 34 48C35.1046 48 36 48.8954 36 50Z"
              fill="#F38000"
            />
          </svg>
        </div>
        <p class="">{{ content }}</p>
      </div>
      <div class="flex gap-4 justify-end">
        <BitButton
          :ui="ButtonUI.Main"
          :type="ButtonType.Button"
          @click="closeModal()"
          :is-disabled="true"
        >
          <div class="px-4">OK</div>
        </BitButton>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal {
  box-shadow: 0px 0px 6px rgba(0, 0, 0, 0.25);
}
</style>
