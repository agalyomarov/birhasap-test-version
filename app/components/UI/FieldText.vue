<script setup lang="ts">
const model = defineModel<string>();
</script>

<template>
   <input
      v-model="model"
      class="w-67.5 h-7.5 p-1.5 rounded-xs text-[15px] border border-[#A0A0A1] focus:outline-none focus:rounded-none focus:border-[#F4CC4C] focus:shadow-[0_0_0_1px_#F4CC4C]" />
</template>

<style scoped></style>
