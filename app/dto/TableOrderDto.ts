import type { TableColumnOrderEnum } from "~/enums/table-column-order-enum";

export type TableOrderDto = {
  key: string;
  order: TableColumnOrderEnum;
};

export function createTableOrderDto(dto: TableOrderDto): TableOrderDto {
  return dto;
}
