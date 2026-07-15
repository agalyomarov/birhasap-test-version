<script setup lang="ts">
defineProps<{
  isOrdered: boolean;
  isOrderDesc: boolean;
  canSort: boolean;
  click: () => void;
  title: string;
}>();
</script>

<template>
  <th>
    <button
      class="bg-gray-e6"
      :class="[canSort ? 'cursor-pointer active:bg-gray-d6' : ' cursor-default']"
      @click="click()"
    >
      <span class="text-nowrap"> {{ title }}</span>

      <svg
        v-if="canSort"
        :class="[isOrdered === false ? 'opacity-0' : '', isOrderDesc ? ' rotate-180' : '']"
        class="block"
        width="6"
        height="12"
        viewBox="0 0 5 10"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M2.5 10L0 7.5H5L2.5 10Z"
          fill="#4D4D4D"
        />
        <line
          x1="2.5"
          y1="2.18557e-08"
          x2="2.5"
          y2="8"
          stroke="#4D4D4D"
        />
      </svg>
    </button>
  </th>
</template>

<style scoped>
@reference "~/assets/css/main.css";
th {
  @apply sticky  top-0 z-10 border-[1px_0px_1px_0px] border-gray-e6;
}

th button {
  @apply px-2 h-7.5  w-full text-start font-medium flex items-center justify-between gap-2  border-r border-white;
}
</style>
