<script setup lang="ts">
import { createProductTableRowDto, type ProductTableRowDto } from "~/dto/ProductTableRowDto";
import { createTableColumnDto, type TableColumnDto } from "~/dto/TableColumnDto";
import { createTableOrderDto, type TableOrderDto } from "~/dto/TableOrderDto";
import { TableColumnOrderEnum } from "~/enums/table-column-order-enum";

const tableContainer = ref<HTMLElement | null>(null);
const productsStore = useProductStore();
const { productsPageTableSelectedId } = storeToRefs(productsStore);
const page = ref(1);
const tableOrder = ref<TableOrderDto | null>(null);
const tableColumns = ref([
  createTableColumnDto({
    key: "id",
    title: "ID",
    canSort: true,
    isHidden: true,
  }),
  createTableColumnDto({
    key: "barcode",
    title: "Ştrihkod",
    canSort: true,
    isHidden: false,
  }),
  createTableColumnDto({
    key: "name",
    title: "Harydyn ady",
    canSort: true,
    isHidden: false,
  }),
  createTableColumnDto({
    key: "price",
    title: "Harydyn bahasy",
    canSort: true,
    isHidden: false,
  }),
  createTableColumnDto({
    key: "amount",
    title: "Mukdary",
    canSort: true,
    isHidden: false,
  }),
  createTableColumnDto({
    key: "dimension",
    title: "Olceg birligi",
    canSort: false,
    isHidden: false,
  }),
]);

const tableRows = ref<ProductTableRowDto[]>([]);

const fetchProductData = () => {
  tableOrder.value = null;
  tableRows.value = Array.from({ length: page.value * 100 }).map((v, index) => {
    return createProductTableRowDto({
      id: index + 1,
      barcode: 1000000 + index + 1,
      name: `Test haryt ${index + 1} ady Test haryt ${index + 1} ady  Test haryt ${index + 1} ady Test haryt ${index + 1} ady Test haryt ${index + 1} ady Test haryt ${index + 1} ady Test haryt ${index + 1} ady  Test haryt ${index + 1} ady  Test haryt ${index + 1} ady `,
      price: (index + 1) * 1000.0,
      amount: (index + 1) * 100.0,
      dimension: "sany",
    });
  });
};

const handleSelectId = (id: number | null) => {
  if (productsPageTableSelectedId.value === id) {
    productsStore.$patch({ productsPageTableSelectedId: null });
  } else {
    productsStore.$patch({ productsPageTableSelectedId: id });
  }
};

const handleClickTh = (column: TableColumnDto) => {
  let newTableOrder = null;
  if (tableOrder.value?.key === column.key) {
    if (tableOrder.value?.order === TableColumnOrderEnum.Asc) {
      newTableOrder = createTableOrderDto({ key: column.key, order: TableColumnOrderEnum.Desc });
    } else {
      newTableOrder = null;
    }
  } else {
    newTableOrder = createTableOrderDto({ key: column.key, order: TableColumnOrderEnum.Asc });
  }

  tableOrder.value = newTableOrder;
  tableRows.value = tableRows.value.sort((a, b) => {
    const order = tableOrder?.value?.order ?? TableColumnOrderEnum.Asc;
    const column = (tableOrder.value === null ? "id" : tableOrder.value.key) as keyof typeof a;
    const aValue = a[column];
    const bValue = b[column];
    if (aValue === bValue) return 0;
    const result = aValue > bValue ? 1 : -1;
    return order === TableColumnOrderEnum.Asc ? result : -result;
  });
};

const handleScroll = () => {
  const el = tableContainer.value;
  if (!el) return;
  const isBottom = el.scrollTop + el.clientHeight >= el.scrollHeight - 10;
  if (isBottom) {
    if (page.value < 10) {
      page.value += 1;
    }
  }
};

watch([page], () => {
  fetchProductData();
});

onMounted(() => {
  fetchProductData();
});
</script>
<template>
  <section class="flex-1 section">
    <div
      ref="tableContainer"
      class="table-container h-full overflow-auto"
      @scroll="handleScroll()"
    >
      <table>
        <thead>
          <tr>
            <BitTableTh
              key="number"
              :can-sort="false"
              :is-ordered="false"
              :is-order-desc="false"
              title="№"
              :click="() => {}"
            />
            <template
              v-for="column in tableColumns"
              :key="column.key"
            >
              <BitTableTh
                :can-sort="column.canSort"
                :is-ordered="column.key === tableOrder?.key"
                :is-order-desc="tableOrder?.order === TableColumnOrderEnum.Desc"
                :title="column.title"
                :click="() => handleClickTh(column)"
              />
            </template>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="(row, index) in tableRows"
            :key="row.id"
            v-memo="[productsPageTableSelectedId === row.id]"
            @click="handleSelectId(row.id)"
          >
            <BitTableTd
              :is-selected="productsPageTableSelectedId === row.id"
              :value="index + 1"
            />
            <template v-for="(value, key) in row">
              <BitTableTd
                :is-selected="productsPageTableSelectedId === row.id"
                :value="value"
              />
            </template>
          </tr>
        </tbody>
      </table>
    </div>
  </section>
</template>

<style scoped></style>
