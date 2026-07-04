<script setup lang="ts"></script>
<template>
   <main class="px-4 py-3.75">
      <UITitlebar>Birhasaba girmek</UITitlebar>
      <div class="mt-5.75">
         <label class="flex items-center justify-between mb-4.5">
            <UIFieldTitle>Ady</UIFieldTitle>
            <UIFieldText />
         </label>
         <label class="flex items-center justify-between mb-8.25">
            <UIFieldTitle>Pinkod</UIFieldTitle>
            <UIFieldText type="password" />
         </label>
         <div class="flex gap-2 justify-end">
            <UIButtonDisabled>Dowam etmek</UIButtonDisabled>
            <UIButtonBase>Yapmak</UIButtonBase>
         </div>
      </div>
   </main>
</template>

<style scoped></style>
