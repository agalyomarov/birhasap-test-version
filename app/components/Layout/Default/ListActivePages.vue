<script setup lang="ts">
const activeIndex = ref(0);
const list = Array.from({ length: 15 }).map((item, index) => {
  if (index == 0) {
    return {
      title: "Esasy sahypa",
    };
  }
  return {
    title: `Beyleki ${index}`,
  };
});
</script>
<template>
  <ul class="border-b border-[#A0A0A0] bg-[#F2F2F2] flex items-center overflow-x-auto small-scrollbar select-none">
    <li
      v-for="(value, index) in list"
      class="pt-1.75 pb-0.75 gap-1 border-r h-full cursor-default"
      :class="[activeIndex == index ? 'bg-white' : 'border-[#A0A0A0]']"
      @click="activeIndex = index"
    >
      <div class="flex items-center gap-1.75 mb-1 px-3 py-[0.5px]">
        <div v-if="index == 0">
          <svg
            width="16"
            height="16"
            viewBox="0 0 16 16"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
          >
            <path
              d="M0 0H16V16H0V0Z"
              fill="url(#pattern0_9604_469)"
            />
            <defs>
              <pattern
                id="pattern0_9604_469"
                patternContentUnits="objectBoundingBox"
                width="1"
                height="1"
              >
                <use
                  xlink:href="#image0_9604_469"
                  transform="translate(0 0.0625) scale(0.0625)"
                />
              </pattern>
              <image
                id="image0_9604_469"
                width="400"
                height="120"
                preserveAspectRatio="none"
                xlink:href="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAZAAAAB4CAYAAADc36SXAAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAA2ZpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDUuMy1jMDExIDY2LjE0NTY2MSwgMjAxMi8wMi8wNi0xNDo1NjoyNyAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wTU09Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9tbS8iIHhtbG5zOnN0UmVmPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VSZWYjIiB4bWxuczp4bXA9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8iIHhtcE1NOk9yaWdpbmFsRG9jdW1lbnRJRD0ieG1wLmRpZDoyRTU4MEYyN0EzNEZFNDExQjIwNTg1Mzk4RDUyNkY1NCIgeG1wTU06RG9jdW1lbnRJRD0ieG1wLmRpZDpEMTFCMTlEMjZCMzgxMUU5ODVCM0ExQjEwMkJFNzQxMiIgeG1wTU06SW5zdGFuY2VJRD0ieG1wLmlpZDpEMTFCMTlEMTZCMzgxMUU5ODVCM0ExQjEwMkJFNzQxMiIgeG1wOkNyZWF0b3JUb29sPSJBZG9iZSBQaG90b3Nob3AgQ1M2IChXaW5kb3dzKSI+IDx4bXBNTTpEZXJpdmVkRnJvbSBzdFJlZjppbnN0YW5jZUlEPSJ4bXAuaWlkOjlCMDkzQjEzNzE1N0U5MTFBMUI0RkM3QzQ5OTNCMURDIiBzdFJlZjpkb2N1bWVudElEPSJ4bXAuZGlkOjJFNTgwRjI3QTM0RkU0MTFCMjA1ODUzOThENTI2RjU0Ii8+IDwvcmRmOkRlc2NyaXB0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"
              />
            </defs>
          </svg>
        </div>
        <span class="text14 text-nowrap">{{ value.title }}</span>
        <div
          v-if="index != 0"
          class="cursor-pointer"
        >
          <svg
            width="16"
            height="16"
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
        class="h-0.5 bg-[#3B863B] mx-2"
        :class="[activeIndex > 0 && activeIndex == index ? '' : ' opacity-0']"
      ></div>
    </li>
  </ul>
</template>

<style scoped></style>
