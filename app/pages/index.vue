<script setup lang="ts">
const searchRef = ref<string | null>(null);
const handleClearSearch = () => {
  searchRef.value = null;
};
</script>
<template>
  <main class="p-[15px_15px_15px_22px]">
    <section class="flex justify-end w-full mb-12.5">
      <ByteFieldTextButton
        placeholder="Gozleg (Ctrl+F)"
        v-model="searchRef"
        @clear="handleClearSearch()"
      />
    </section>
    <section>
      <ul class="grid grid-cols-3 gap-y-8">
        <li v-for="value in 10">
          <ByteLinkGroup />
        </li>
      </ul>
    </section>
  </main>
</template>

<style scoped></style>
