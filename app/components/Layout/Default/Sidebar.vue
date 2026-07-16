<script setup lang="ts">
const list = Array.from({ length: 5 }).map((item, index) => {
  if (index == 0) {
    return {
      title: "Esasy sahypa",
      href: AppRoutes.home(),
    };
  }
  return {
    title: `Beyleki ${index}`,
    href: "#",
  };
});

const activeIndex = ref(0);
</script>
<template>
  <aside
    class="overflow-hidden h-full flex flex-col border-r border-gray-a1 select-none bg-yellow-a7"
  >
    <ul class="">
      <li v-for="(value, index) in list">
        <NuxtLink
          :to="value.href"
          class="flex items-center gap-3.5 pl-4 py-2.5"
          :class="[index == activeIndex ? ' bg-white' : '']"
          @click="activeIndex = index"
        >
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
                fill="url(#pattern0_9612_6925)"
              />
              <defs>
                <pattern
                  id="pattern0_9612_6925"
                  patternContentUnits="objectBoundingBox"
                  width="1"
                  height="1"
                >
                  <use
                    xlink:href="#image0_9612_6925"
                    transform="scale(0.015625)"
                  />
                </pattern>
                <image
                  id="image0_9612_6925"
                  width="64"
                  height="64"
                  preserveAspectRatio="none"
                  xlink:href="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAABS2lUWHRYTUw6Y29tLmFkb2JlLnhtcAAAAAAAPD94cGFja2V0IGJlZ2luPSLvu78iIGlkPSJXNU0wTXBDZWhpSHpyZVN6TlRjemtjOWQiPz4KPHg6eG1wbWV0YSB4bWxuczp4PSJhZG9iZTpuczptZXRhLyIgeDp4bXB0az0iQWRvYmUgWE1QIENvcmUgNS42LWMxMzggNzkuMTU5ODI0LCAyMDE2LzA5LzE0LTAxOjA5OjAxICAgICAgICAiPgogPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4KICA8cmRmOkRlc2NyaXB0aW9uIHJkZjphYm91dD0iIi8+CiA8L3JkZjpSREY+CjwveDp4bXBtZXRhPgo8P3hwYWNrZXQgZW5kPSJyIj8+IEmuOgAAAM1JREFUeJztmMENxCAMwKC69WBZBuQWIPyKq2B/8zGWECiliMjN1GjQWpsnRd5mjLE863Na5GsYgBagMQAtQGMAWoDmt5mFf4RMhIecM9U/qNS6Pur1V8AAtACNAWgBmvAZ7L2f9MDY/QNyvYPBk3/9FTAALUBjAFqAxgC0AI37gGjgPuASDEAL0BiAFqBxH7CZ5XoH3QesMQAtQGMAWoDGALQAjfuAaOA+4BIMQAvQGIAWoHEfsJnlegfdB6wxAC1AYwBagMYAtICICMkfTnUTQA3pCLgAAAAASUVORK5CYII="
                />
              </defs>
            </svg>
          </div>
          <span class="text-nowrap text-ellipsis overflow-hidden">{{ value.title }}</span>
        </NuxtLink>
      </li>
    </ul>
  </aside>
</template>

<style scoped></style>
