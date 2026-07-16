<script setup lang="ts">
import { createRouteHistoryDto } from "~/dto/RouteHistoryDto";

const pageTitle = "Satuwy uytgetmek";
const routeHistoryStore = useRouteHistory();
const formData = reactive({
  barcode: 10001,
  name: "Test haryt",
  price: 1000,
  amount: 10,
  total_price: 1000,
  dimension: "sant",
});

const handleStoreSale = () => {
  console.log(formData);
  navigateTo(AppRoutes.satuwlar());
};

onMounted(() => {
  const routeHistory = createRouteHistoryDto({
    id: AppRoutes.saleCreate(),
    title: pageTitle,
    href: AppRoutes.saleCreate(),
    canClose: true,
  });
  routeHistoryStore.addHistory(routeHistory);
  routeHistoryStore.$patch({ activeId: routeHistory.id });
});
</script>
<template>
  <div class="wrapper flex flex-col">
    <ByteFormPanel>{{ pageTitle }}</ByteFormPanel>
    <section class="command-panel flex items-center">
      <ul class="flex items-center justify-between w-full">
        <li class="flex items-center gap-2">
          <BitButtonMainText @click="handleStoreSale()">
            <span>Hasaba almak we yapmak</span>
          </BitButtonMainText>
          <BitButtonText @click="navigateTo(AppRoutes.satuwlar())">
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
        <BitFieldTitle>Umumy Bahasy</BitFieldTitle>
        <BitFieldText
          v-model.number="formData.total_price"
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
