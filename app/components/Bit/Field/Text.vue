<script setup lang="ts">
const model = defineModel<string | null | number>();
defineProps<{
  placeholder?: string;
  type?: "text" | "password" | "number";
  hasClear?: boolean;
  isRequired?: boolean;
}>();
const emits = defineEmits(["clear"]);

const inputRef = ref<HTMLInputElement | null>(null);
const focusInput = (): void => {
  if (inputRef.value) {
    inputRef.value.focus();
    // inputRef.value.select();
  }
};

defineExpose({
  focusInput,
});
</script>

<template>
  <div
    class="flex h-7 items-center border border-gray-a1 rounded-xs focus-within:border-yellow-4c focus-within:rounded-none focus-within:shadow-[0_0_0_1px] shadow-yellow-4c"
  >
    <input
      ref="inputRef"
      v-model="model"
      class="flex-1 h-5 mx-1.5"
      :class="[
        isRequired && model?.toString().length === 0 ? 'border-b border-dotted border-red-30' : '',
      ]"
      autocapitalize="off"
      autocomplete="off"
      autocorrect="off"
      spellcheck="false"
      :placeholder="placeholder"
      :type="type"
    />
    <button
      v-if="hasClear"
      type="button"
      class="w-6.5 h-7 flex items-center justify-center cursor-pointer border-l border-gray-a1"
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
