<script setup lang="ts"></script>

<template>
  <LayoutDefaultTopbar />
  <LayoutDefaultListActivePages />
  <main class="flex-1 grid grid-cols-[184px_auto]">
    <LayoutDefaultSidebar />
    <NuxtPage class="min-w-0" />
  </main>
  <LayoutDefaultBottombar />
</template>

<style scoped></style>
