export type TableColumnDto = {
  key: string;
  title: string;
  canSort: boolean;
  isHidden: boolean;
};

export function createTableColumnDto(dto: TableColumnDto): TableColumnDto {
  return dto;
}
