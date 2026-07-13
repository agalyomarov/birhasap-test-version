<script setup lang="ts">
import { ButtonType } from "~/enums/button-type";
import { ButtonUI } from "~/enums/button-ui";
const props = defineProps<{
  ui: ButtonUI;
  type?: ButtonType;
}>();
</script>

<template>
  <button
    :type="props.type ?? 'button'"
    :class="[
      props.ui == ButtonUI.Text && 'border border-gray-b3 rounded-xs  select-none h-7.5 px-4 cursor-pointer btn--simple',
      props.ui == ButtonUI.IconText && 'border border-gray-b3 rounded-xs  select-none h-7.5 pl-1.5 pr-3 cursor-pointer btn--simple',
      props.ui == ButtonUI.Icon && 'border border-gray-b3 rounded-xs  select-none size-7.5 cursor-pointer btn--simple',
      props.ui == ButtonUI.Main && 'border border-yellow-3b h-7.5 rounded-xs select-none px-4 cursor-pointer font-bold btn--main',
      props.ui == ButtonUI.Disabled && 'border border-gray-b3 rounded-xs select-none  h-7.5  px-4  cursor-not-allowed btn--disabled opacity-50',
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
  border-color: var(--color-gray-8a);
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
