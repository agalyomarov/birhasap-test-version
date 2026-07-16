<script setup lang="ts">
import { createRouteHistoryDto, type RouteHistoryDto } from "~/dto/RouteHistoryDto";
import { ModalTypeEnum } from "~/enums/modal-type-enum";

const productsStore = useProductStore();
const routeHistoryStore = useRouteHistory();
const { productsPageTableSelectedId } = storeToRefs(productsStore);
const { openModal } = useModal();

const handleDeleteRow = () => {
  if (productsPageTableSelectedId.value === null) {
    return;
  }
  openModal({
    modalContent: "Harydy yok etmegi dowam etmelimi?",
    modalType: ModalTypeEnum.Confirm,
    modalTitle: "Hereketi tassyklamak",
    onConfirm: () => {
      if (productsPageTableSelectedId.value) {
        deleteProduct(productsPageTableSelectedId.value);
      }
    },
    onCancel: () => {
      console.log("Отмена удаления");
    },
  });
};

const deleteProduct = (id: number) => {
  console.log(`DELETE:${id}`);
  productsStore.$patch({ productsPageTableSelectedId: null });
};

const handleEditRow = () => {
  if (productsPageTableSelectedId.value === null) {
    return;
  }
  navigateTo(AppRoutes.productEdit(productsPageTableSelectedId.value));
};

onMounted(() => {
  const routeHistory = createRouteHistoryDto({
    id: AppRoutes.harytlar(),
    title: "Harytlar",
    href: AppRoutes.harytlar(),
    canClose: true,
  });
  routeHistoryStore.addHistory(routeHistory);
  routeHistoryStore.$patch({ activeId: routeHistory.id });
});
</script>
<template>
  <div class="wrapper flex flex-col">
    <ByteFormPanel>Harytlar</ByteFormPanel>
    <section class="command-panel flex items-center">
      <ul class="flex items-center justify-between w-full">
        <li class="flex items-center gap-2">
          <BitButtonIconText @click="navigateTo(AppRoutes.productCreate())">
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
            :is-disabled="productsPageTableSelectedId == null"
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
            :is-disabled="productsPageTableSelectedId == null"
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
    <PageHarytlarTable />
    <ByteModalConfirm />
  </div>
</template>

<style scoped></style>
