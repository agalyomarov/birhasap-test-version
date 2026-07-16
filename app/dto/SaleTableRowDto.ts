export type SaleTableRowDto = {
  id: number;
  barcode: number;
  name: string;
  price: number;
  amount: number;
  total_price: number;
  dimension: string;
};

export function createSaleTableRowDto(dto: SaleTableRowDto): SaleTableRowDto {
  return dto;
}
