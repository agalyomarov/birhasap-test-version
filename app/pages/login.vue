<script setup lang="ts">
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { ButtonType } from "~/enums/button-type";
import { ButtonUI } from "~/enums/button-ui";

definePageMeta({
  layout: "login",
});

const { openModal } = useModal();

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
  if (lowercaseName.value == "admin" && lowercasePassword.value == "1122") {
    hideContent.value = true;
    await appWindow.setAlwaysOnTop(false);
    await appWindow.setResizable(true);
    await appWindow.setMinSize(new LogicalSize(1200, 800));
    await appWindow.center();
    await appWindow.maximize();
    // await new Promise((r) => setTimeout(r, 500));
    await navigateTo("/", { replace: true });
    console.log("form");
  } else {
    openModal({ modalContent: "Ulanyjy ya-da pinkod yalnysh doldurlan", modalType: "warning", modalTitle: null });
  }
};

onMounted(async () => {
  await appWindow.setAlwaysOnTop(true);
  await appWindow.setResizable(false);
  await appWindow.setSize(new LogicalSize(370, 222));
});
</script>
<template>
  <main
    v-if="!hideContent"
    class="p-4 select-none"
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
        <UIButton
          :ui="ButtonUI.Disabled"
          :type="ButtonType.Button"
          v-if="lowercaseName.length < 4 || lowercasePassword.length < 4"
        >
          Dowam etmek
        </UIButton>
        <UIButton
          v-else
          :ui="ButtonUI.Simple"
          :type="ButtonType.Submit"
        >
          Dowam etmek
        </UIButton>
        <UIButton
          :ui="ButtonUI.Simple"
          :type="ButtonType.Button"
          @click="handleCloseWindow()"
        >
          Yapmak
        </UIButton>
      </div>
    </form>
  </main>
  <UIModalWarning />
</template>

<style scoped></style>
