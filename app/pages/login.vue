<script setup lang="ts">
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { ButtonTypeEnum } from "~/enums/button-type-enum";
import { ModalTypeEnum } from "~/enums/modal-type-enum";
import { authLoginCommand, isApiError, type AuthLoginCommandParams } from "~/types";
import { UserRoleEnum } from "~/utils/enums";

definePageMeta({
  layout: "empty",
});

const { openModal } = useModal();
const userStore = useUserStore();

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
  try {
    const params: AuthLoginCommandParams = {
      params: { login: lowercaseName.value, password: lowercasePassword.value },
    };
    const response = await authLoginCommand(params);
    userStore.$patch({ authToken: response.token, role: response.role });
    hideContent.value = true;
    await appWindow.setAlwaysOnTop(false);
    await appWindow.setResizable(true);
    await appWindow.setMinSize(new LogicalSize(1000, 700));
    await appWindow.center();
    await appWindow.maximize();
    if (response.role === UserRoleEnum.Admin) {
      await navigateTo(AppRoutes.adminHome(), { replace: true });
      return;
    }

    if (response.role === UserRoleEnum.Kassir) {
      await navigateTo(AppRoutes.kassirHome(), { replace: true });
      return;
    }
  } catch (err: any) {
    if (isApiError(err)) {
      openModal({
        modalContent: err.message,
        modalType: ModalTypeEnum.Warning,
        modalTitle: null,
      });
    }
  }
};

onMounted(async () => {
  await appWindow.setAlwaysOnTop(true);
  await appWindow.setResizable(false);
  await appWindow.setSize(new LogicalSize(370, 222));
  hideContent.value = false;
});
</script>
<template>
  <main
    v-if="!hideContent"
    class="p-4 select-none"
  >
    <PageLoginTopbar>Birhasaba girmek </PageLoginTopbar>
    <form
      class="mt-7"
      @submit.prevent="handleSubmit()"
    >
      <label class="flex items-center justify-between mb-5">
        <BitFieldTitle>Ulanyjy</BitFieldTitle>
        <BitFieldText
          v-model.trim="lowercaseName"
          class="w-70"
        />
      </label>
      <label class="flex items-center justify-between mb-7">
        <BitFieldTitle>Pinkod</BitFieldTitle>
        <BitFieldText
          v-model.trim="lowercasePassword"
          type="password"
          class="w-70"
        />
      </label>
      <div class="flex gap-4 justify-end">
        <BitButtonText
          :type="ButtonTypeEnum.Submit"
          :is-disabled="lowercaseName.length < 4 || lowercasePassword.length < 4"
        >
          Dowam etmek
        </BitButtonText>
        <BitButtonText
          :type="ButtonTypeEnum.Button"
          @click="handleCloseWindow()"
        >
          Ýapmak
        </BitButtonText>
      </div>
    </form>
  </main>
  <ByteModalWarning />
</template>

<style scoped></style>
