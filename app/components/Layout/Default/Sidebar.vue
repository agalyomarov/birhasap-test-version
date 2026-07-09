<script setup lang="ts">
const activeIndex = ref(0);
const list = Array.from({ length: 5 }).map((item, index) => {
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
  <aside class="w-46 overflow-hidden border-b h-full flex flex-col border-r border-[#A0A0A0] select-none">
    <ul class="bg-[#F9EDA7] border-b border-[#A0A0A0]">
      <li
        v-for="(value, index) in list"
        class="flex items-center gap-3.5 py-2.5 pl-4"
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
        <span class="text-text-simple text-color-simple text-nowrap text-ellipsis overflow-hidden">{{ value.title }}</span>
      </li>
    </ul>
    <ul class="bg-[#F2F2F2] flex-1 overflow-y-auto ui-small-scrollbar">
      <li
        v-for="(value, index) in list"
        class="flex items-center gap-1.25 border-b border-[#A0A0A0]"
        :class="[index == activeIndex ? ' bg-white' : '']"
        @click="activeIndex = index"
      >
        <div
          class="w-0.5 my-0.75 ml-1 bg-[#3B863B] h-6.75"
          :class="[index != activeIndex ? ' opacity-0' : '']"
        ></div>
        <span class="text-text-simple text-color-simple text-nowrap text-ellipsis overflow-hidden">Заголовок формы</span>
      </li>
    </ul>
  </aside>
</template>

<style scoped></style>
