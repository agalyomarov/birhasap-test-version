import type { TableColumnOrderEnum } from "~/enums/table-column-order-enum";

export type TableOrderDto = {
  key: string;
  order: TableColumnOrderEnum;
  __type: "TableOrderDto";
};

export function createTableOrderDto(dto: Omit<TableOrderDto, "__type">): TableOrderDto {
  return { ...dto, __type: "TableOrderDto" };
}

export function isTableOrderDto(obj: any): obj is TableOrderDto {
  return obj?.__type === "TableOrderDto";
}
