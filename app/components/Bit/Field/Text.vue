<script setup lang="ts">
const model = defineModel<string | null>();
</script>

<template>
  <input
    v-model="model"
    class="w-67.5 h-7.5 p-1.5 rounded-xs border border-gray-a1 focus:rounded-none input"
    autocapitalize="off"
    autocomplete="off"
    autocorrect="off"
    spellcheck="false"
  />
</template>

<style scoped>
.input:focus {
  border-color: var(--color-yellow-4c);
  box-shadow: 0 0 0 1px var(--color-yellow-4c);
}
</style>
