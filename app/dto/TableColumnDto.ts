export type TableColumnDto<TRow extends object> = {
  key: keyof TRow & string;
  title: string;
  canSort: boolean;
  __type: "TableColumnDto";
};

export function createTableColumnDto<TRow extends object>(dto: Omit<TableColumnDto<TRow>, "__type">): TableColumnDto<TRow> {
  return {
    ...dto,
    __type: "TableColumnDto",
  };
}
