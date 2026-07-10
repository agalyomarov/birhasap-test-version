<script setup lang="ts">
import BitFieldText from "~/components/Bit/Field/Text.vue";
const searchValueRef = ref<string | null>(null);
const handleClearSearch = () => {
  searchValueRef.value = null;
};
const searchInputRef = ref<InstanceType<typeof BitFieldText> | null>(null);

const handleKeyDown = (event: KeyboardEvent): void => {
  const isMac = navigator.userAgent.includes("Mac");
  const isModifierPressed = isMac ? event.metaKey : event.ctrlKey;
  if (isModifierPressed && event.key.toLowerCase() === "f") {
    event.preventDefault();
    if (searchInputRef.value) {
      searchInputRef.value.focusInput();
    }
  }
};

onMounted(() => {
  window.addEventListener("keydown", handleKeyDown);
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleKeyDown);
});
</script>
<template>
  <main class="p-[15px_15px_15px_22px]">
    <section class="flex justify-end w-full mb-12.5">
      <BitFieldText
        ref="searchInputRef"
        placeholder="Gozleg (Ctrl+F) / (Cmd+F)"
        v-model="searchValueRef"
        @clear="handleClearSearch()"
        :hasClear="true"
      />
    </section>
    <section>
      <ul class="flex gap-x-15 2xl:gap-x-10 gap-y-7.5 flex-wrap">
        <li
          v-for="value in 10"
          class="lg:w-1/3 2xl:w-1/4"
        >
          <ByteLinkGroup />
        </li>
      </ul>
    </section>
  </main>
</template>

<style scoped></style>
