export default defineNuxtRouteMiddleware((to, from) => {
  const { authToken } = storeToRefs(useUserStore());

  const publicRoutes = ["/login", "/register", "/"];

  if (!authToken.value && !publicRoutes.includes(to.path)) {
    return navigateTo("/login");
  }
});
