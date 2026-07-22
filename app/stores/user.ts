import type { UserRole } from "~/types";

export const useUserStore = defineStore("users", {
  state: () => ({
    authToken: null as string | null,
    role: null as null | UserRole,
  }),
  getters: {},
  actions: {},
});
