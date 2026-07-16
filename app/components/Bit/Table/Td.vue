<script setup lang="ts">
defineProps<{ isSelected?: boolean; value: string | number | boolean }>();
</script>
<template>
  <td :class="[isSelected ? ' bg-yellow-a7' : '']">
    {{ value }}
  </td>
</template>

<style scoped>
@reference "~/assets/css/main.css";

td {
  @apply cursor-default px-2 h-7.5 text-nowrap max-w-225 truncate border-[0px_1px_1px_0px] border-gray-f2 min-w-10;
}
</style>
