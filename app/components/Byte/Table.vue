<script setup lang="ts" generic="TRow extends TableRowBaseDto">
import type { TableColumnDto } from "~/dto/TableColumnDto";
import type { TableDataDto } from "~/dto/TableDataDto";
import { createTableOrderDto, type TableOrderDto } from "~/dto/TableOrderDto";
import type { TableRowBaseDto } from "~/dto/TableRowBaseDto";
import { TableColumnOrderEnum } from "~/enums/table-column-order-enum";

const tableContainer = ref<HTMLElement | null>(null);
const props = defineProps<{
  data: TableDataDto<TRow>;
  order: TableOrderDto | null;
  selectedId: number | null;
}>();
const emit = defineEmits(["selected", "sorted", "loadMore"]);

const handleScroll = () => {
  const el = tableContainer.value;
  if (!el) return;
  const isBottom = el.scrollTop + el.clientHeight >= el.scrollHeight - 10;
  if (isBottom) {
    emit("loadMore");
  }
};

const handleSelect = (id: number) => {
  if (props.selectedId == id) {
    emit("selected", null);
  } else {
    emit("selected", id);
  }
};

const handleSort = (column: TableColumnDto<TRow>) => {
  if (props.order === null) {
    emit("sorted", createTableOrderDto({ key: column.key, order: TableColumnOrderEnum.Asc }));
    return;
  }
  if (column.key !== props.order.key) {
    emit("sorted", createTableOrderDto({ key: column.key, order: TableColumnOrderEnum.Asc }));
    return;
  }
  if (column.key === props.order.key && props.order.order === TableColumnOrderEnum.Asc) {
    emit("sorted", createTableOrderDto({ key: column.key, order: TableColumnOrderEnum.Desc }));
  } else {
    emit("sorted", null);
  }
};
</script>

<template>
  <div
    ref="tableContainer"
    class="overflow-auto min-w-0 border-[0px_1px_0px_1px] border-gray-f2"
    @scroll="handleScroll"
  >
    <table class="w-full bg-white table">
      <thead class="">
        <tr class="">
          <th key="number">
            <button class="cursor-default bg-gray-e6">№</button>
          </th>
          <th
            v-for="column in data.columns"
            :key="column.key"
          >
            <button
              class="bg-gray-e6"
              :class="[column.canSort ? 'cursor-pointer active:bg-gray-d6' : ' cursor-default']"
              @click="handleSort(column)"
            >
              <span class="text-nowrap"> {{ column.title }}</span>
              <div
                v-if="column.canSort"
                :class="[
                  order?.key !== column.key ? ' opacity-0' : '',
                  order?.order === TableColumnOrderEnum.Desc ? ' rotate-180' : '',
                ]"
              >
                <svg
                  width="6"
                  height="12"
                  viewBox="0 0 5 10"
                  fill="none"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path
                    d="M2.5 10L0 7.5H5L2.5 10Z"
                    fill="#4D4D4D"
                  />
                  <line
                    x1="2.5"
                    y1="2.18557e-08"
                    x2="2.5"
                    y2="8"
                    stroke="#4D4D4D"
                  />
                </svg>
              </div>
            </button>
          </th>
        </tr>
      </thead>
      <tbody class="">
        <tr
          v-for="(row, index) in data.rows"
          :key="row.id"
          v-memo="[selectedId === row.id]"
          class="cursor-default"
          @click="handleSelect(row.id as number)"
        >
          <td
            class=" "
            :class="[selectedId === row.id ? ' bg-yellow-a7' : '']"
          >
            {{ index + 1 }}
          </td>
          <template v-for="(value, key) in row">
            <td
              v-if="key !== '__type'"
              :class="[selectedId === row.id ? ' bg-yellow-a7' : '']"
            >
              {{ value }}
            </td>
          </template>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
@reference "~/assets/css/main.css";

th {
  @apply sticky  top-0 z-10 border-[1px_0px_1px_0px] border-gray-e6;
}

th button {
  @apply px-2 h-7.5  w-full text-start font-medium flex items-center justify-between gap-2  border-r border-white;
}

td {
  @apply px-2 h-7.5 text-nowrap max-w-225 truncate border-[0px_1px_1px_0px] border-gray-f2 min-w-10;
}

table {
  border-collapse: separate;
  border-spacing: 0;
}
</style>
