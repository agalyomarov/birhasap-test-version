export interface ApiErrorResponse {
  message: string;
  code: string;
  data: Record<string, string>;
  __type: "AppError";
}

export function isApiErrorResponse(err: unknown): err is ApiErrorResponse {
  return (
    typeof err === "object" && err !== null && "__type" in err && err.__type == "ApiErrorResponse"
  );
}
