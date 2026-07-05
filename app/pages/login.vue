<script setup lang="ts">
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
// import { LogicalSize } from "@tauri-apps/api/dpi";

const appWindow = getCurrentWindow();
const hideContent = ref(false);
const inputName = ref("");
const inputPassword = ref("");

const lowercaseName = computed({
  get: () => inputName.value,
  set: (newValue) => {
    inputName.value = newValue.toLowerCase();
  },
});

const lowercasePassword = computed({
  get: () => inputPassword.value,
  set: (newValue) => {
    inputPassword.value = newValue.toLowerCase();
  },
});

const handleCloseWindow = async () => {
  await appWindow.close();
};

const handleSubmit = async () => {
  hideContent.value = true;
  await appWindow.setAlwaysOnTop(false);
  await appWindow.setResizable(true);
  await appWindow.maximize();
  await appWindow.center();
  await navigateTo("/", { replace: true });
  console.log("form");
};

onMounted(async () => {
  await appWindow.setAlwaysOnTop(true);
  await appWindow.setResizable(false);
  await appWindow.setSize(new LogicalSize(370, 222));
  await appWindow.center();
});
</script>
<template>
  <main
    v-if="!hideContent"
    class="p-4"
  >
    <PageLoginTitlebar>Birhasaba girmek</PageLoginTitlebar>
    <form
      class="mt-7"
      @submit.prevent="handleSubmit()"
    >
      <label class="flex items-center justify-between mb-5">
        <UIFieldTitle>Ulanyjy</UIFieldTitle>
        <UIFieldText v-model.trim="lowercaseName" />
      </label>
      <label class="flex items-center justify-between mb-7">
        <UIFieldTitle>Pinkod</UIFieldTitle>
        <UIFieldText
          v-model.trim="lowercasePassword"
          type="password"
        />
      </label>
      <div class="flex gap-4 justify-end">
        <UIButtonDisabled
          type="button"
          v-if="lowercaseName.length < 4 || lowercasePassword.length < 4"
        >
          Dowam etmek
        </UIButtonDisabled>
        <UIButtonBase
          v-else
          type="submit"
        >
          Dowam etmek
        </UIButtonBase>
        <UIButtonBase
          type="button"
          @click="handleCloseWindow()"
        >
          Yapmak
        </UIButtonBase>
      </div>
    </form>
  </main>
</template>

<style scoped></style>
