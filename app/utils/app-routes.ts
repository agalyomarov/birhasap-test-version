export class AppRoutes {
  static adminHome = () => "/admin";
  static adminHarytlar = () => `/admin/harytlar`;
  static adminSatuwlar = () => `/admin/satuwlar`;
  static adminProductEdit = (uuid: string) => `/admin/product/${uuid}/edit`;
  static adminProductCreate = () => `/admin/product/create`;
  static adminSaleEdit = (id: number) => `/admin/sale/${id}/edit`;
  static adminSaleCreate = () => `/admin/sale/create`;

  static kassirHome = () => "/kassir";
}
