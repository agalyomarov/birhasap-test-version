export type RouteHistoryDto = {
  id: string;
  title: string;
  href: string;
  canClose: boolean;
};

export function createRouteHistoryDto(dto: RouteHistoryDto): RouteHistoryDto {
  return dto;
}
