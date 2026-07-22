import { createRouteHistoryDto, type RouteHistoryDto } from "~/dto/RouteHistoryDto";

export const useRouteHistory = defineStore("route-history", {
  state: () => {
    return {
      history: [
        createRouteHistoryDto({
          id: AppRoutes.adminHome(),
          title: "Esasy sahypa",
          href: AppRoutes.adminHome(),
          canClose: false,
        }),
      ],
      activeId: AppRoutes.adminHome() as string,
    };
  },
  getters: {},
  actions: {
    addHistory(dto: RouteHistoryDto) {
      const hasDto = this.history.findIndex((item) => item.id === dto.id);
      if (hasDto == -1) {
        this.history.push(dto);
      }
    },
    deleteHistory(id: string): string {
      const index = this.history.findIndex((item, i) => item.id == id);
      const prevDto = this.history[index - 1];
      const newList = this.history.filter((item, i) => item.id !== id);
      this.history = newList;
      return prevDto!.href;
    },
  },
});
