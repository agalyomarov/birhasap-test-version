<script setup lang="ts"></script>

<template>
  <LayoutDefaultTopbar />
  <LayoutDefaultListActivePages />
  <div class="flex items-start flex-1">
    <LayoutDefaultSidebar />
    <NuxtPage class="flex-1" />
  </div>
  <LayoutDefaultBottombar />
</template>

<style scoped></style>
