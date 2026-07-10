<script setup lang="ts">
defineProps<{
  placeholder?: string;
  type?: "text" | "password";
}>();

const model = defineModel<string | null>();
const emits = defineEmits(["clear"]);
</script>

<template>
  <div class="flex items-center">
    <BitFieldText
      v-model="model"
      :placeholder="placeholder"
      :type="type ?? 'text'"
      class="rounded-r-none"
    />
    <button
      type="button"
      class="w-6.5 h-7.5 flex items-center justify-center border border-l-0 border-color-border rounded-r-xs cursor-pointer"
      @click="emits('clear')"
    >
      <svg
        width="16"
        height="16"
        viewBox="0 0 17 17"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <line
          x1="0.5"
          y1="-0.5"
          x2="12.2924"
          y2="-0.5"
          transform="matrix(-0.703545 0.710651 -0.703545 -0.710651 13 3)"
          stroke="black"
          stroke-linecap="round"
        />
        <line
          x1="0.5"
          y1="-0.5"
          x2="12.2924"
          y2="-0.5"
          transform="matrix(0.703545 0.710651 -0.703545 0.710651 4 4.09082)"
          stroke="black"
          stroke-linecap="round"
        />
      </svg>
    </button>
  </div>
</template>

<style scoped></style>
