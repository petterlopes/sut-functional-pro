// Infrastructure - API Client Interface
export interface ApiResponse<T = any> {
  data: T
  status: number
  headers: Record<string, string>
}

export interface ApiRequestConfig {
  headers?: Record<string, string>
  params?: Record<string, any>
}

export interface IApiClient {
  get<T = any>(url: string, config?: ApiRequestConfig): Promise<ApiResponse<T>>
  post<T = any>(url: string, data?: any, config?: ApiRequestConfig): Promise<ApiResponse<T>>
  put<T = any>(url: string, data?: any, config?: ApiRequestConfig): Promise<ApiResponse<T>>
  patch<T = any>(url: string, data?: any, config?: ApiRequestConfig): Promise<ApiResponse<T>>
  delete<T = any>(url: string, config?: ApiRequestConfig): Promise<ApiResponse<T>>
}
