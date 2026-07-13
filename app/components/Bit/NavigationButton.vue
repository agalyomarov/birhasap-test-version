<script setup lang="ts">
import { NavigationButtonUI } from "~/enums/navigation-button-ui";

const props = defineProps<{
  ui: NavigationButtonUI;
  type?: "button";
}>();
</script>

<template>
  <button
    :type="type ?? 'button'"
    class="border border-gray-b3 select-none h-7 px-3 cursor-pointer btn--simple"
    :class="ui == NavigationButtonUI.Left ? ' rounded-l-xs' : 'rounded-r-xs'"
  >
    <svg
      :class="ui == NavigationButtonUI.Left ? '' : ' rotate-180'"
      width="17"
      height="17"
      viewBox="0 0 17 17"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M7.2963 7.07895V4L2 8.73684L7.2963 13V10.1579H15V7.07895H7.2963Z"
        fill="#4D4D4D"
      />
    </svg>
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
</style>
