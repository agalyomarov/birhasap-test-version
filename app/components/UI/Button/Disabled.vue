<script setup lang="ts"></script>
<template>
   <UIButtonBase class="opacity-50 cursor-not-allowed!">
      <slot />
   </UIButtonBase>
</template>

<style scoped>
.button:active {
   background: none;
   border-color: #b3b3b3;
}
</style>
