export class AppRoutes {
  static adminHome = () => "/admin";
  static adminHarytlar = () => `/admin/harytlar`;
  static adminSatuwlar = () => `/admin/satuwlar`;
  static adminProductEdit = (id: number) => `/admin/product/${id}/edit`;
  static adminProductCreate = () => `/admin/product/create`;
  static adminSaleEdit = (id: number) => `/admin/sale/${id}/edit`;
  static adminSaleCreate = () => `/admin/sale/create`;

  static kassirHome = () => "/kassir";
}
