export class AppRoutes {
  static home = () => "/";
  static harytlar = () => `/harytlar`;
  static satuwlar = () => `/satuwlar`;
  static productEdit = (id: number) => `/product/${id}/edit`;
  static productCreate = () => `/product/create`;
}
