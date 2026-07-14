import type { TableColumnDto } from "./TableColumnDto";

export type TableDataDto<TRow extends object> = {
  columns: TableColumnDto<TRow>[];
  rows: TRow[];
  __type: "TableDataDto";
};

export function createTableDataDto<TRow extends object>(dto: Omit<TableDataDto<TRow>, "__type">): TableDataDto<TRow> {
  return {
    ...dto,
    __type: "TableDataDto",
  };
}
