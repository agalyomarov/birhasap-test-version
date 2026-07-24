export type ProductTableRowDto = {
  id: number;
  barcode: string;
  title: string;
  price: number;
  amount: number;
  dimension: string;
};

export function createProductTableRowDto(dto: ProductTableRowDto): ProductTableRowDto {
  return dto;
}
