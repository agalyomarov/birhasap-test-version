export type SaleTableRowDto = {
  id: number;
  barcode: number;
  name: string;
  price: number;
  total_price: number;
  amount: number;
  dimension: string;
  __type: "SaleTableRowDto";
};

export function createSaleTableRowDto(dto: Omit<SaleTableRowDto, "__type">): SaleTableRowDto {
  return { ...dto, __type: "SaleTableRowDto" };
}

export function isSaleTableRowDto(obj: any): obj is SaleTableRowDto {
  return obj?.__type === "SaleTableRowDto";
}
