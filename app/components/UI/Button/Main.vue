<script setup lang="ts"></script>
<template>
  <button class="border border-[#B3B3B3] cursor-pointer rounded-xs text-[#4D4D4D] select-none outline-0 font-bold button-default button">
    <slot />
  </button>
</template>

<style scoped>
.button {
  background: linear-gradient(180deg, #fae960 0%, #fbe14c 51.39%, #e9d146 100%);
  border: 1px solid #b7a53b;
  box-shadow: 0px 1px 1px rgba(155, 155, 155, 0.75);
}

.button:active {
  background: linear-gradient(180deg, #cdb62c 4.17%, #ecd447 10.42%);
  border: 1px solid #8b8b8b;
}
</style>
