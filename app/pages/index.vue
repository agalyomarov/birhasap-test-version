<script setup lang="ts">
import BitFieldText from "~/components/Bit/Field/Text.vue";

const routeHistoryStore = useRouteHistory();
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
  routeHistoryStore.$patch({ activeId: AppRoutes.home() });
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleKeyDown);
});
</script>
<template>
  <div class="wrapper overflow-y-auto">
    <section class="command-panel flex justify-end items-center">
      <BitFieldText
        ref="searchInputRef"
        placeholder="Gozleg (Ctrl+F) / (Cmd+F)"
        v-model="searchValueRef"
        @clear="handleClearSearch()"
        :hasClear="true"
      />
    </section>
    <section class="section">
      <ul class="flex gap-x-15 2xl:gap-x-10 gap-y-7.5 flex-wrap">
        <li
          v-for="value in 10"
          class="lg:w-1/3 2xl:w-1/4"
        >
          <ByteLinkGroup />
        </li>
      </ul>
    </section>
  </div>
</template>

<style scoped></style>
