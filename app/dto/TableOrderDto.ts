export type TableOrderDto = {
  key: string;
  order: SortDirectionEnum;
};

export function createTableOrderDto(dto: TableOrderDto): TableOrderDto {
  return dto;
}
