<script setup lang="ts"></script>
<template>
   <button class="px-4 py-0.5 border border-[#B3B3B3] button text-[13.7px] text-[#4D4D4D] cursor-pointer select-none">
      <slot />
   </button>
</template>

<style scoped>
.button {
   background: linear-gradient(180deg, #ffffff 0%, #ffffff 51.39%, #f0f0f0 100%);
   box-shadow: 0px 1px 1px rgba(155, 155, 155, 0.75);
}
</style>
