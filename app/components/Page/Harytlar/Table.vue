<script setup lang="ts">
import { createProductTableRowDto, type ProductTableRowDto } from "~/dto/ProductTableRowDto";
import { createTableColumnDto, type TableColumnDto } from "~/dto/TableColumnDto";
import { createTableOrderDto, type TableOrderDto } from "~/dto/TableOrderDto";

const tableContainer = ref<HTMLElement | null>(null);
const productsStore = useProductStore();
const { productsPageTableSelectedId } = storeToRefs(productsStore);
const page = ref(1);
const limit = ref(50);
const tableOrder = ref<TableOrderDto>(
  createTableOrderDto({ key: "id", order: SortDirectionEnum.Asc }),
);
const tableColumns = ref([
  createTableColumnDto({
    key: "id",
    title: "ID",
    canSort: true,
    isHidden: false,
  }),
  createTableColumnDto({
    key: "barcode",
    title: "Barkod",
    canSort: true,
    isHidden: false,
  }),
  createTableColumnDto({
    key: "title",
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

const fetchProductData = async () => {
  try {
    const response = await adminProductListCommand({
      params: {
        page: page.value,
        limit: limit.value,
        sort_column: tableOrder.value.key,
        sort_direction: tableOrder.value.order,
      },
    });
    if (response.products.length > 0) {
      tableRows.value.push(
        ...response.products.map((p) => ({
          id: p.id,
          barcode: p.barcode,
          title: p.title,
          price: p.price,
          amount: p.amount,
          dimension: p.dimension,
        })),
      );
    }
  } catch (error) {}
};

const handleSelectId = (id: number | null) => {
  if (productsPageTableSelectedId.value === id) {
    productsStore.$patch({ productsPageTableSelectedId: null });
  } else {
    productsStore.$patch({ productsPageTableSelectedId: id });
  }
};

const handleClickTh = (column: TableColumnDto) => {
  let newTableOrder = createTableOrderDto({ key: "id", order: SortDirectionEnum.Asc });
  if (tableOrder.value.key === column.key) {
    if (tableOrder.value?.order === SortDirectionEnum.Asc) {
      newTableOrder = createTableOrderDto({ key: column.key, order: SortDirectionEnum.Desc });
    }
  } else {
    newTableOrder = createTableOrderDto({ key: column.key, order: SortDirectionEnum.Asc });
  }
  tableOrder.value = newTableOrder;
};

const handleScroll = () => {
  const el = tableContainer.value;
  if (!el) return;
  const isBottom = el.scrollTop + el.clientHeight >= el.scrollHeight - 10;
  if (isBottom) {
    // if (page.value < 10) {
    page.value += 1;
    // }
  }
};

watch([page, tableOrder], () => {
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
                :is-order-desc="tableOrder?.order === SortDirectionEnum.Desc"
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
