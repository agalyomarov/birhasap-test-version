<script setup lang="ts"></script>

<template>
  <LayoutDefaultTopbar />
  <LayoutDefaultListActivePages />
  <div class="flex items-start flex-1 relative overflow-y-auto">
    <LayoutDefaultSidebar class="sticky left-0 top-0" />
    <NuxtPage class="flex-1 overflow-y-auto" />
  </div>
  <LayoutDefaultBottombar />
</template>

<style scoped></style>
