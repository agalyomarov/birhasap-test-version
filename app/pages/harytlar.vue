<script setup lang="ts">
import { createProductTableRowDto, type ProductTableRowDto } from "~/dto/ProductTableRowDto";
import { createTableColumnDto } from "~/dto/TableColumnDto";
import { createTableDataDto } from "~/dto/TableDataDto";
import type { TableOrderDto } from "~/dto/TableOrderDto";
import { ModalTypeEnum } from "~/enums/modal-type-enum";
import { TableColumnOrderEnum } from "~/enums/table-column-order-enum";

const { openModal } = useModal();
const page = ref(1);
const order = ref<TableOrderDto | null>(null);
const selectedId = ref<number | null>(null);
const productTableColumns = ref([
  createTableColumnDto({
    key: "id",
    title: "ID",
    canSort: true,
  }),
  createTableColumnDto({
    key: "barcode",
    title: "Strihkod",
    canSort: true,
  }),
  createTableColumnDto({
    key: "name",
    title: "Harydyn ady",
    canSort: true,
  }),
  createTableColumnDto({
    key: "price",
    title: "Harydyn bahasy",
    canSort: true,
  }),
  createTableColumnDto({
    key: "amount",
    title: "Mukdary",
    canSort: true,
  }),
  createTableColumnDto({
    key: "dimension",
    title: "Olceg birligi",
    canSort: false,
  }),
]);

const productTableRows = ref<ProductTableRowDto[]>([]);

const productTableData = ref(
  createTableDataDto({
    columns: productTableColumns.value,
    rows: productTableRows.value,
  }),
);

const fetchProductData = () => {
  order.value = null;
  productTableRows.value = Array.from({ length: page.value * 100 }).map((v, index) => {
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
  selectedId.value = id;
};

const handleSort = (newOrder: TableOrderDto | null) => {
  order.value = newOrder;
  productTableRows.value = productTableRows.value.sort((a, b) => {
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

const handleDeleteRow = () => {
  if (selectedId.value !== null) {
    openModal({
      modalContent: "Harydy yok etmegi dowam etmelimi?",
      modalType: ModalTypeEnum.Confirm,
      modalTitle: "Hereketi tassyklamak",
      onConfirm: () => {
        if (selectedId.value) {
          deleteProduct(selectedId.value);
        }
      },
      onCancel: () => {
        console.log("Отмена удаления");
      },
    });
  }
};

const handleEditRow = () => {
  if (selectedId.value) {
    navigateTo(AppRoutes.productEdit(selectedId.value));
  }
};

const handleCreateRow = () => {
  navigateTo(AppRoutes.productCreate());
};

const deleteProduct = (id: number) => {
  console.log(`DELETE:${id}`);
  selectedId.value = null;
};

watch([page], () => {
  fetchProductData();
});

watch(
  [productTableColumns, productTableRows],
  () => {
    productTableData.value = createTableDataDto({
      columns: productTableColumns.value,
      rows: productTableRows.value,
    });
  },
  { deep: true },
);

onMounted(() => {
  fetchProductData();
});
</script>
<template>
  <div class="wrapper flex flex-col">
    <ByteFormPanel>Harytlar</ByteFormPanel>
    <section class="command-panel flex items-center">
      <ul class="flex items-center justify-between w-full">
        <li class="flex items-center gap-2">
          <BitButtonIconText @click="handleCreateRow()">
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
            <span>Goshmak</span>
          </BitButtonIconText>
          <BitButtonIcon
            :is-disabled="selectedId == null"
            @click="handleEditRow()"
          >
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
                fill="url(#pattern0_9649_4580)"
              />
              <defs>
                <pattern
                  id="pattern0_9649_4580"
                  patternContentUnits="objectBoundingBox"
                  width="1"
                  height="1"
                >
                  <use
                    xlink:href="#image0_9649_4580"
                    transform="scale(0.0625)"
                  />
                </pattern>
                <image
                  id="image0_9649_4580"
                  width="16"
                  height="16"
                  preserveAspectRatio="none"
                  xlink:href="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAA80lEQVQ4jWP8//8/AyWAiSLdpBjgF5X23y8qDcO5RBmArBHdEIIGZC6I+c+V/g+ngXgNyFwQ8/+9IgcDAwMDA7ohBA1A1gwDMEM2LZvFiNcAbJoZGBgYBO//QNGM1QB8mqcnLGFEF2eBMZry/X0ZGBg2STIwMKAbgEszigGOWiyzeTgY/6tIMDH2r3vNcC1IlKBmFC/sv/ZHQEWCiZGBgYGh0IudQWvda4Ka4QZAnc8OE3z9CRLNhDQje6GIgYGBoX/bTwYGBoafDlosHxgYGNIJaWZgYGBg+P//P0O4l+l/dxvtF415fv7///9nIAUzUpqdAXUOfr4hHuT/AAAAAElFTkSuQmCC"
                />
              </defs>
            </svg>
          </BitButtonIcon>
          <BitButtonIcon
            :is-disabled="selectedId == null"
            @click="handleDeleteRow()"
          >
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
                fill="url(#pattern0_9649_4581)"
              />
              <defs>
                <pattern
                  id="pattern0_9649_4581"
                  patternContentUnits="objectBoundingBox"
                  width="1"
                  height="1"
                >
                  <use
                    xlink:href="#image0_9649_4581"
                    transform="scale(0.0625)"
                  />
                </pattern>
                <image
                  id="image0_9649_4581"
                  width="16"
                  height="16"
                  preserveAspectRatio="none"
                  xlink:href="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAACBUlEQVQ4jaWRT0iTYRzHP8/7vE232twUZLgWEaxdAkHBaHkJCYRwh/TQHzFxdIq1DhY7dC12sNCLl2E5RjhoeBgEHZIQVke9FATDy9ZwDMKY0Wy+b08HUeb+RNAHvvCDBz7P78dXKKX4H3SA4J0H2VaPmqZRMa2In+X4WjqeaGlQSjE2FckqpfaVUmZjwk9eZMemItmRibs3lVI0RqsT6UoprTEAmcQ8Npvt3rXJcLBpy8Ph9wE05pBMYh4p5aO37z8eO1evP6Ud95++BOCsv5/E6zeMXgn8u+DZw8nLh7MQguuh2Q9NG+iGoX2OxURhdZVqsYjV48E7Ps75cBjZ2Um9oBE95XLZrjq6L+xkDXHO4cDS10fNMCjH42TX17mUSiGtVuCg1kY0IHpxsN8+lExi9/sRQtA1OMhQMokTyC0uHqutleBWRy7HbibDmXQa1/Q0p5eX+Z5IcLJU4ms6jWmaTa3UC7wWKfmxtkZpdpaeSIRiKER1YwOLlOxtb2MYxl8FhZppcmpkBPfcHN8WFvAsLWEdGKBmmnS43Qgh2p6gAyu/fL7H9mCQ/MQEta0tqpubuGZmKObzeAIBJaVU7VoQK06nrdLdW/IZe/ZehwOLlNRMk3Klwhf03XfDo5/29RNHu3f1uEk+jw4fCZRSpFwuKxAFbgNeoAC8AmI3dnaqTd/W8QdhJA5PN2nIugAAAABJRU5ErkJggg=="
                />
              </defs>
            </svg>
          </BitButtonIcon>
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
        :data="productTableData"
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
