<script setup lang="ts">
const routeHistoryStore = useRouteHistory();
const { activeId, history } = storeToRefs(routeHistoryStore);

const handleDelete = (id: string) => {
  const prevRoute = routeHistoryStore.deleteHistory(id);
  navigateTo(prevRoute);
};
</script>
<template>
  <nav>
    <ul
      class="border-b border-gray-a1 bg-gray-f2 flex items-center overflow-x-auto ui-small-scrollbar select-none"
    >
      <li v-for="item in history">
        <NuxtLink
          class="border-r cursor-default flex items-center relative h-8.25 border-gray-a1"
          :class="[activeId == item.id ? 'bg-white' : '']"
          :to="item.href"
          @click="routeHistoryStore.$patch({ activeId: item.id })"
        >
          <div class="flex items-center gap-1.75 pl-4 pr-7 relative h-full">
            <span class="text-nowrap">{{ item.title }}</span>
            <div
              v-if="item.canClose && activeId == item.id"
              class="absolute right-1 top-1.5 cursor-pointer"
              @click.prevent="handleDelete(item.id)"
            >
              <svg
                width="20"
                height="20"
                viewBox="0 0 16 16"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  fill-rule="evenodd"
                  clip-rule="evenodd"
                  d="M11.4359 5.15234C11.639 5.35546 11.639 5.68479 11.4359 5.88791L5.88791 11.4359C5.68479 11.639 5.35546 11.639 5.15234 11.4359C4.94922 11.2328 4.94922 10.9034 5.15234 10.7003L10.7003 5.15234C10.9034 4.94922 11.2328 4.94922 11.4359 5.15234Z"
                  fill="#545454"
                />
                <path
                  fill-rule="evenodd"
                  clip-rule="evenodd"
                  d="M5.15234 5.15234C5.35546 4.94922 5.68479 4.94922 5.88791 5.15234L11.4359 10.7003C11.639 10.9034 11.639 11.2328 11.4359 11.4359C11.2328 11.639 10.9034 11.639 10.7003 11.4359L5.15234 5.88791C4.94922 5.68479 4.94922 5.35546 5.15234 5.15234Z"
                  fill="#545454"
                />
                <path
                  fill-rule="evenodd"
                  clip-rule="evenodd"
                  d="M11.4359 5.15234C11.639 5.35546 11.639 5.68479 11.4359 5.88791L5.88791 11.4359C5.68479 11.639 5.35546 11.639 5.15234 11.4359C4.94922 11.2328 4.94922 10.9034 5.15234 10.7003L10.7003 5.15234C10.9034 4.94922 11.2328 4.94922 11.4359 5.15234Z"
                  stroke="#545454"
                  stroke-width="0.2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                />
                <path
                  fill-rule="evenodd"
                  clip-rule="evenodd"
                  d="M5.15234 5.15234C5.35546 4.94922 5.68479 4.94922 5.88791 5.15234L11.4359 10.7003C11.639 10.9034 11.639 11.2328 11.4359 11.4359C11.2328 11.639 10.9034 11.639 10.7003 11.4359L5.15234 5.88791C4.94922 5.68479 4.94922 5.35546 5.15234 5.15234Z"
                  stroke="#545454"
                  stroke-width="0.2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                />
              </svg>
            </div>
          </div>
          <div
            class="h-0.5 bg-green-3b absolute bottom-0.75 left-1.5 right-1.5"
            :class="[activeId == item.id && item.canClose ? '' : ' opacity-0']"
          ></div>
        </NuxtLink>
      </li>
    </ul>
  </nav>
</template>

<style scoped></style>
