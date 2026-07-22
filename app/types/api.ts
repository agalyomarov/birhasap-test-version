export interface ApiError {
  message: string;
  code: string;
  __type: "AppError";
}

export function isApiError(err: unknown): err is ApiError {
  return typeof err === "object" && err !== null && "__type" in err && err.__type == "AppError";
}
