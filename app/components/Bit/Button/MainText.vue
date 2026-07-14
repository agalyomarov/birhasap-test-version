<script setup lang="ts">
import { ButtonType } from "~/enums/button-type";
const props = defineProps<{
  type?: ButtonType;
  isDisabled?: boolean;
}>();
</script>

<template>
  <button
    :type="props.type ?? 'button'"
    class="border border-yellow-3b h-7.5 rounded-xs select-none px-4 font-bold"
    :class="[isDisabled ? 'opacity-50 cursor-not-allowed btn--disabled' : 'btn cursor-pointer']"
    :disabled="isDisabled ?? false"
  >
    <slot />
  </button>
</template>

<style scoped>
.btn,
.btn--disabled {
  background: linear-gradient(180deg, #fae960 0%, #fbe14c 51.39%, #e9d146 100%);
  box-shadow: 0px 1px 1px rgba(155, 155, 155, 0.75);
}

.btn:active {
  background: linear-gradient(180deg, #cdb62c 4.17%, #ecd447 10.42%);
  box-shadow: none;
}
</style>
