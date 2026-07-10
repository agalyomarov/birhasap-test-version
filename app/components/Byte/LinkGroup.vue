<script setup lang="ts">
const links = [
  {
    title: "Ahli harytlar",
    is_favorited: true,
    href: AppRoutes.harytlar(),
  },
  {
    title: "Beyleki bolumler/Beyleki bolumler 1",
    is_favorited: false,
    href: "#",
  },
  {
    title: "Beyleki bolumler 2",
    is_favorited: false,
    href: "#",
  },
  {
    title: "Beyleki bolumler 3",
    is_favorited: false,
    href: "#",
  },
];
</script>
<template>
  <article>
    <header class="text-windiw-title text-green-46 mb-3.75 select-none">Harytlar</header>
    <ul class="">
      <li
        v-for="link in links"
        class=""
      >
        <NuxtLink
          :to="link.href"
          class="group/link flex items-center gap-1.5"
        >
          <div class="group/parent py-1.5">
            <svg
              width="20"
              height="20"
              viewBox="0 0 16 16"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              class="group-hover/parent:fill-yellow-4c"
              :class="[link.is_favorited ? ' fill-yellow-4c' : ' fill-transparent group-hover/link:fill-gray-a0']"
            >
              <path
                d="M8 1.5L9.9103 5.37069L14.1819 5.99139L11.0909 9.00431L11.8206 13.2586L8 11.25L4.1794 13.2586L4.90907 9.00431L1.81813 5.99139L6.0897 5.37069L8 1.5Z"
              />
            </svg>
          </div>
          <p class="">{{ link.title }}</p>
        </NuxtLink>
      </li>
    </ul>
  </article>
</template>

<style scoped></style>
