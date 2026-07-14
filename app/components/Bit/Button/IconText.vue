<script setup lang="ts">
import type { ButtonTypeEnum } from "~/enums/button-type-enum";

const props = defineProps<{
  type?: ButtonTypeEnum;
  isDisabled?: boolean;
}>();
</script>

<template>
  <button
    :type="props.type ?? 'button'"
    class="border border-gray-b3 rounded-xs select-none h-7.5 pl-1.5 pr-3 flex items-center gap-1.5"
    :class="[isDisabled ? ' opacity-50 cursor-not-allowed btn--disabled' : 'cursor-pointer btn']"
    :disabled="isDisabled ?? false"
  >
    <slot />
  </button>
</template>

<style scoped>
.btn,
.btn--disabled {
  background: linear-gradient(180deg, #ffffff 0%, #ffffff 51.39%, #f0f0f0 100%);
  box-shadow: 0px 1px 1px rgba(155, 155, 155, 0.75);
}

.btn:active {
  background: linear-gradient(180deg, #c8c8c8 4.17%, #ebebeb 10.42%);
  border-color: var(--color-gray-8a);
}
</style>
