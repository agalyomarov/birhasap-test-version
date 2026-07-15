export type ProductTableRowDto = {
  id: number;
  barcode: number;
  name: string;
  price: number;
  amount: number;
  dimension: string;
};

export function createProductTableRowDto(dto: ProductTableRowDto): ProductTableRowDto {
  return dto;
}
