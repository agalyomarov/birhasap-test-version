export type ProductTableRowDto = {
  id: number;
  barcode: number;
  name: string;
  price: number;
  amount: number;
  dimension: string;
  __type: "ProductTableRowDto";
};

export function createProductTableRowDto(
  dto: Omit<ProductTableRowDto, "__type">,
): ProductTableRowDto {
  return { ...dto, __type: "ProductTableRowDto" };
}

export function isProductTableRowDto(obj: any): obj is ProductTableRowDto {
  return obj?.__type === "ProductTableRowDto";
}
