<script setup lang="ts">
import { createProductTableRowDto, type ProductTableRowDto } from "~/dto/ProductTableRowDto";
import { createSaleTableRowDto, type SaleTableRowDto } from "~/dto/SaleTableRowDto";
import { createTableColumnDto } from "~/dto/TableColumnDto";
import { createTableDataDto } from "~/dto/TableDataDto";
import type { TableOrderDto } from "~/dto/TableOrderDto";
import { TableColumnOrderEnum } from "~/enums/table-column-order-enum";

const page = ref(1);
const order = ref<TableOrderDto | null>(null);
const selectedId = ref<number | null>(null);
const saleTableColumns = ref([
  createTableColumnDto({
    key: "id",
    title: "ID",
    canSort: true,
    isHidden: true,
  }),
  createTableColumnDto({
    key: "barcode",
    title: "Strihkod",
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
    key: "total_price",
    title: "Umumy baha",
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

const saleTableRows = ref<SaleTableRowDto[]>([]);

const saleTableData = ref(
  createTableDataDto({
    columns: saleTableColumns.value,
    rows: saleTableRows.value,
  }),
);

const fetchSalesData = () => {
  order.value = null;
  saleTableRows.value = Array.from({ length: page.value * 100 }).map((v, index) => {
    return createSaleTableRowDto({
      id: index + 1,
      barcode: 1000000 + index + 1,
      name: `Test haryt ${index + 1} ady Test haryt ${index + 1} ady  Test haryt ${index + 1} ady Test haryt ${index + 1} ady Test haryt ${index + 1} ady Test haryt ${index + 1} ady Test haryt ${index + 1} ady  Test haryt ${index + 1} ady  Test haryt ${index + 1} ady `,
      price: (index + 1) * 1000.0,
      total_price: (index + 1) * 1000.0,
      amount: (index + 1) * 100.0,
      dimension: "sany",
    });
  });
};

const handleSelectId = (id: number | null) => {
  selectedId.value = id;
};

const handleSort = (newOrder: TableOrderDto | null) => {
  order.value = newOrder;
  saleTableRows.value = saleTableRows.value.sort((a, b) => {
    const order = newOrder?.order ?? TableColumnOrderEnum.Asc;
    const column = (newOrder === null ? "id" : newOrder.key) as Exclude<keyof typeof a, "__type">;
    const aValue = a[column];
    const bValue = b[column];
    if (aValue === bValue) return 0;
    const result = aValue > bValue ? 1 : -1;
    return order === TableColumnOrderEnum.Asc ? result : -result;
  });
};

const loadNextPage = () => {
  if (page.value < 10) {
    page.value += 1;
  }
};

watch([page], () => {
  fetchSalesData();
});

watch(
  [saleTableColumns, saleTableRows],
  () => {
    saleTableData.value = createTableDataDto({
      columns: saleTableColumns.value,
      rows: saleTableRows.value,
    });
  },
  { deep: true },
);

onMounted(() => {
  fetchSalesData();
});
</script>
<template>
  <div class="wrapper flex flex-col">
    <ByteFormPanel>Satuwlar</ByteFormPanel>
    <section class="command-panel flex items-center">
      <ul class="flex items-center justify-between w-full">
        <li class="flex items-center gap-2">
          <BitButtonIconText>
            <div>
              <svg
                width="16"
                height="16"
                viewBox="0 0 16 16"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                xmlns:xlink="http://www.w3.org/1999/xlink"
              >
                <rect
                  width="16"
                  height="16"
                  fill="url(#pattern0_9649_4574)"
                />
                <defs>
                  <pattern
                    id="pattern0_9649_4574"
                    patternContentUnits="objectBoundingBox"
                    width="1"
                    height="1"
                  >
                    <use
                      xlink:href="#image0_9649_4574"
                      transform="scale(0.0625)"
                    />
                  </pattern>
                  <image
                    id="image0_9649_4574"
                    width="16"
                    height="16"
                    preserveAspectRatio="none"
                    xlink:href="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAABsUlEQVQ4jZ2TPUscURSGn7tMFtyPa2RcHLXYZtakWru1Mf8gsdHUkrQSCLbCgkKKVNGQkCqCaYON4D+wMVWwkjgIK6zMJjssTGYmskvupJhxMruzBOIpX8597nvOfa8ACMOQu5Y2Ksid+WlgE3gMLMTyOXAEvHWb7W66X6QdyJ35VeCjWTUnZysGxYkiAF7gY3dtrJblAs/dZvswA4gPf16qN4Q+pY+16/QcTs++hMDTW4gAKG/PVQBrqd6Q6cMHy+8BWD/ZGIW4QM1ttr/nYv1FrWrK0Zu1nIaWG16TPqVTq5oSeAlwC1iZmTYylvtqQF8NMrpRMSBacvIKD0uFYmJbiIibz90D4NOjDwCEoWL9ZINi1PsgDUjq0rtCiwFzhcjVddBBCPitVARSIUA/DTj3An9Rlspsf32dwLbqm0DIq7M3Q5cENwHAZXoHR52unZlV5svIvMzo9g8b4DgNeHfRslyn5ww1egMfb+APaU7P4SIK1C6MC9JiQ+j3/zNIqSivAftm1SzPVgxKhRJKKfxfPna3g9WyfgLPxkY5mfvvZ3pC9FR94Fs8894/P9Nd6g/iT7J5V+tghwAAAABJRU5ErkJggg=="
                  />
                </defs>
              </svg>
            </div>
            <span>Satuwlary yuklemek</span>
          </BitButtonIconText>
        </li>
        <li>
          <BitFieldText
            :has-clear="true"
            placeholder="Gozleg (Ctrl+F) / (Cmd+F)"
            class="w-100"
          />
        </li>
      </ul>
    </section>
    <section class="flex-1 section">
      <ByteTable
        class="h-full"
        :data="saleTableData"
        :order="order"
        :selected-id="selectedId"
        @selected="handleSelectId"
        @sorted="handleSort"
        @loadMore="loadNextPage"
      />
    </section>
    <ByteModalConfirm />
  </div>
</template>

<style scoped></style>
