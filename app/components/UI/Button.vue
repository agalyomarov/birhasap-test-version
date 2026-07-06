<script setup lang="ts">
import { ButtonType } from "~/enums/button-type";
import { ButtonUI } from "~/enums/button-ui";
const props = defineProps<{
  ui: ButtonUI;
  type: ButtonType;
}>();
</script>

<template>
  <button
    :type="props.type"
    :class="[
      props.ui == ButtonUI.Simple && 'outline-0 border border-[#B3B3B3] rounded-xs select-none text-[15px] px-4 py-0.75 cursor-pointer btn--simple',
      props.ui == ButtonUI.Main && 'outline-0 border border-[#B7A53B] rounded-xs select-none text-[15px] px-4 py-0.75 cursor-pointer font-bold btn--main',
      props.ui == ButtonUI.Disabled &&
        'outline-0 border border-[#B3B3B3] rounded-xs select-none text-[15px] px-4 py-0.75 cursor-not-allowed btn--disabled opacity-50',
    ]"
  >
    <slot />
  </button>
</template>

<style scoped>
.btn--simple {
  background: linear-gradient(180deg, #ffffff 0%, #ffffff 51.39%, #f0f0f0 100%);
  box-shadow: 0px 1px 1px rgba(155, 155, 155, 0.75);
}

.btn--simple:active {
  background: linear-gradient(180deg, #c8c8c8 4.17%, #ebebeb 10.42%);
  border-color: #8b8b8b;
}

.btn--disabled {
  background: linear-gradient(180deg, #ffffff 0%, #ffffff 51.39%, #f0f0f0 100%);
  box-shadow: 0px 1px 1px rgba(155, 155, 155, 0.75);
}

.btn--main {
  background: linear-gradient(180deg, #fae960 0%, #fbe14c 51.39%, #e9d146 100%);
  box-shadow: 0px 1px 1px rgba(155, 155, 155, 0.75);
}

.btn--main:active {
  background: linear-gradient(180deg, #cdb62c 4.17%, #ecd447 10.42%);
  box-shadow: none;
}
</style>
