<script setup lang="ts"></script>
<template>
  <button class="border border-[#B3B3B3] cursor-pointer rounded-xs text-[#4D4D4D] select-none outline-0 button-default button">
    <slot />
  </button>
</template>

<style scoped>
.button {
  background: linear-gradient(180deg, #ffffff 0%, #ffffff 51.39%, #f0f0f0 100%);
  box-shadow: 0px 1px 1px rgba(155, 155, 155, 0.75);
}

.button:active {
  background: linear-gradient(180deg, #c8c8c8 4.17%, #ebebeb 10.42%);
  border-color: #8b8b8b;
}
</style>
