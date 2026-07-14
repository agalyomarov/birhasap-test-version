<script setup lang="ts">
const selectedIndex = ref<number | null>(null);
const tableColumns = [
  {
    key: "number",
    title: "№",
    sorted: false,
    canSort: false,
  },
  {
    key: "id",
    title: "ID",
    sorted: true,
    canSort: true,
  },
  {
    key: "name",
    title: "Harydyn ady",
    sorted: true,
    canSort: true,
  },
  {
    key: "price",
    title: "Bahasy",
    sorted: false,
    canSort: true,
  },
  {
    key: "amount",
    title: "Mukdary",
    sorted: false,
    canSort: true,
  },
  {
    key: "dimension",
    title: "Olceg birligi",
    sorted: false,
    canSort: true,
  },
];

const tableRows = Array.from({ length: 100 }).map((v, index) => {
  return {
    number: index + 1,
    id: index + 1,
    name: `Test haryt ${index + 1} ady Test haryt ${index + 1} ady  Test haryt ${index + 1} ady Test haryt ${index + 1} ady Test haryt ${index + 1} ady Test haryt ${index + 1} ady Test haryt ${index + 1} ady  Test haryt ${index + 1} ady  Test haryt ${index + 1} ady `,
    price: `${index + 1}000,00`,
    amount: `${index + 1}00,000`,
    dimension: "sany",
  };
});
</script>

<template>
  <div class="overflow-auto min-w-0">
    <table class="w-full bg-white table">
      <thead class="">
        <tr class="bg-gray-f2">
          <th
            v-for="column in tableColumns"
            :key="column.key"
            class="sticky top-0 border-tb border-gray-e6"
          >
            <button
              class="px-2 h-7.5 w-full text-start font-medium flex items-center justify-between gap-2 bg-gray-e6 active:bg-gray-d6 border-r border-white"
              :class="[column.canSort ? 'cursor-pointer' : ' cursor-default']"
            >
              <span class="text-nowrap"> {{ column.title }}</span>
              <div v-if="column.canSort">
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
          v-for="(row, index) in tableRows"
          class="cursor-default  border-gray-f2""
          @click="selectedIndex = index"
        >
          <td
            v-for="(value, key) in row"
            :key="key"
            class="pl-2 h-7.5 text-nowrap min-w-0 max-w-225 truncate border-[0px_1px_1px_0] border-gray-f2"
            :class="[selectedIndex === index ? ' bg-yellow-a7' : '']"
          >
            {{ value }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped></style>
