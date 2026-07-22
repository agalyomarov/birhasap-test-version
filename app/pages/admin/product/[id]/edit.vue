<script setup lang="ts">
import { createRouteHistoryDto, type RouteHistoryDto } from "~/dto/RouteHistoryDto";

const routeHistory = ref<RouteHistoryDto | null>(null);
const pageTitle = "Haryt uytgetmek";
const routeHistoryStore = useRouteHistory();
const formData = reactive({
  barcode: 1001,
  name: "Mysal product",
  price: 1000,
  amount: 1000,
  dimension: "sany",
});

const handleStoreProduct = () => {
  console.log(formData);
  navigateTo(AppRoutes.adminHarytlar());
};

onMounted(() => {
  routeHistory.value = createRouteHistoryDto({
    id: AppRoutes.adminProductEdit(100),
    title: pageTitle,
    href: AppRoutes.adminProductEdit(100),
    canClose: true,
  });
  routeHistoryStore.addHistory(routeHistory.value);
  routeHistoryStore.$patch({ activeId: routeHistory.value.id });
});

onUnmounted(() => {
  routeHistoryStore.deleteHistory(routeHistory.value!.id);
});
</script>
<template>
  <div class="wrapper flex flex-col">
    <ByteFormPanel>{{ pageTitle }}</ByteFormPanel>
    <section class="command-panel flex items-center">
      <ul class="flex items-center justify-between w-full">
        <li class="flex items-center gap-2">
          <BitButtonMainText @click="handleStoreProduct()">
            <span>Hasaba almak we yapmak</span>
          </BitButtonMainText>
          <BitButtonText @click="navigateTo(AppRoutes.adminHarytlar())">
            <span>Yapmak</span>
          </BitButtonText>
        </li>
      </ul>
    </section>
    <section class="section w-150 space-y-3">
      <label class="flex items-center justify-between">
        <BitFieldTitle>Strihkod</BitFieldTitle>
        <BitFieldText
          v-model.number="formData.barcode"
          type="number"
          class="w-120"
          :is-required="true"
        />
      </label>
      <label class="flex items-center justify-between">
        <BitFieldTitle>Harydyn ady</BitFieldTitle>
        <BitFieldText
          v-model="formData.name"
          class="w-120"
          :is-required="true"
        />
      </label>
      <label class="flex items-center justify-between">
        <BitFieldTitle>Bahasy</BitFieldTitle>
        <BitFieldText
          v-model.number="formData.price"
          type="number"
          class="w-120"
          :is-required="true"
        />
      </label>
      <label class="flex items-center justify-between">
        <BitFieldTitle>Mukdary</BitFieldTitle>
        <BitFieldText
          v-model.number="formData.amount"
          type="number"
          class="w-120"
          :is-required="true"
        />
      </label>
      <label class="flex items-center justify-between">
        <BitFieldTitle>Olceg birligi</BitFieldTitle>
        <BitFieldText
          v-model="formData.dimension"
          class="w-120"
          :is-required="true"
        />
      </label>
    </section>
  </div>
</template>

<style scoped></style>
