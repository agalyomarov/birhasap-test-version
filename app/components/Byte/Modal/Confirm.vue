<script setup lang="ts">
import { ButtonTypeEnum } from "~/enums/button-type-enum";
import { ModalTypeEnum } from "~/enums/modal-type-enum";

const { isOpen, type, content, closeModal, title, confirm, cancel } = useModal();

const timer = ref(9);
let timerId: ReturnType<typeof setInterval> | null = null;
const stopTimer = () => {
  if (timerId) {
    clearInterval(timerId);
    timerId = null;
  }
};
const runTimer = () => {
  stopTimer();
  timerId = setInterval(() => {
    timer.value--;
    if (timer.value <= 0) {
      stopTimer();
      closeModal();
    }
  }, 1000);
};

watch(isOpen, (opened) => {
  if (opened) {
    timer.value = 9;
    runTimer();
  } else {
    stopTimer();
  }
});

onUnmounted(() => {
  stopTimer();
});
</script>
<template>
  <section
    v-if="isOpen && type == ModalTypeEnum.Confirm"
    class="fixed z-50 top-0 left-0 w-full h-full flex items-center justify-center select-none bg-white/50"
  >
    <div class="p-4 w-100 z-10 bg-white border border-gray-a1 modal">
      <div class="flex justify-end">
        <button
          class="hover:bg-gray-ec cursor-pointer"
          @click="closeModal()"
        >
          <svg
            width="22"
            height="22"
            viewBox="0 0 17 17"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <line
              x1="0.5"
              y1="-0.5"
              x2="12.2924"
              y2="-0.5"
              transform="matrix(-0.703545 0.710651 -0.703545 -0.710651 13 3)"
              stroke="black"
              stroke-linecap="round"
            />
            <line
              x1="0.5"
              y1="-0.5"
              x2="12.2924"
              y2="-0.5"
              transform="matrix(0.703545 0.710651 -0.703545 0.710651 4 4.09082)"
              stroke="black"
              stroke-linecap="round"
            />
          </svg>
        </button>
      </div>

      <div class="flex items-center gap-4 mb-6">
        <div>
          <svg
            width="64"
            height="64"
            viewBox="0 0 64 64"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              fill-rule="evenodd"
              clip-rule="evenodd"
              d="M32 61C48.0163 61 61 48.0163 61 32C61 15.9837 48.0163 3 32 3C15.9837 3 3 15.9837 3 32C3 48.0163 15.9837 61 32 61ZM32 63C49.1208 63 63 49.1208 63 32C63 14.8792 49.1208 1 32 1C14.8792 1 1 14.8792 1 32C1 49.1208 14.8792 63 32 63Z"
              fill="#4C8CD3"
            />
            <path
              fill-rule="evenodd"
              clip-rule="evenodd"
              d="M32 52C33.1046 52 34 51.1046 34 50C34 48.8954 33.1046 48 32 48C30.8954 48 30 48.8954 30 50C30 51.1046 30.8954 52 32 52ZM32 54C34.2091 54 36 52.2091 36 50C36 47.7909 34.2091 46 32 46C29.7909 46 28 47.7909 28 50C28 52.2091 29.7909 54 32 54Z"
              fill="#4C8CD3"
            />
            <path
              d="M42.1627 26.2095C41.3301 28.2195 39.9202 29.9375 38.1113 31.1462C36.5807 32.1688 34.8227 32.7881 33 32.9545V39H31V31H32C33.7796 31 35.5193 30.4703 36.999 29.4816C38.4787 28.4929 39.632 27.0876 40.3131 25.4434C40.9941 23.7992 41.1723 21.99 40.8251 20.2446C40.4779 18.4991 39.6209 16.8958 38.3625 15.6375C37.1042 14.3791 35.5009 13.5221 33.7554 13.1749C32.01 12.8277 30.2008 13.0059 28.5566 13.6869C26.9124 14.368 25.5071 15.5213 24.5184 17.001C24.1005 17.6264 23.765 18.2983 23.5168 19H21.417C21.7288 17.9001 22.2118 16.8496 22.8538 15.8887C24.0625 14.0798 25.7805 12.6699 27.7905 11.8373C29.8005 11.0048 32.0122 10.7869 34.146 11.2114C36.2798 11.6358 38.2398 12.6834 39.7782 14.2218C41.3166 15.7602 42.3642 17.7202 42.7886 19.854C43.2131 21.9878 42.9952 24.1995 42.1627 26.2095Z"
              fill="#4C8CD3"
            />
          </svg>
        </div>
        <div class="space-y-3">
          <p class="text-h1">{{ title }}</p>
          <p class="">{{ content }}</p>
        </div>
      </div>
      <div class="flex gap-4 justify-end">
        <BitButtonMainText
          :type="ButtonTypeEnum.Button"
          @click="confirm"
        >
          <div class="px-4 uppercase">Hawa</div>
        </BitButtonMainText>
        <BitButtonText
          :type="ButtonTypeEnum.Button"
          @click="cancel"
        >
          <div class="px-4">Yok</div>
        </BitButtonText>
        <BitButtonText
          :type="ButtonTypeEnum.Button"
          @click="cancel"
        >
          Yapmak({{ timer }})
        </BitButtonText>
      </div>
    </div>
  </section>
</template>

<style scoped>
.modal {
  box-shadow: 0px 0px 6px rgba(0, 0, 0, 0.25);
}
</style>
