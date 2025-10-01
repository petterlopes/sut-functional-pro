import axios, { AxiosInstance, AxiosResponse } from 'axios'
import { IApiClient, ApiResponse, ApiRequestConfig } from './IApiClient'

export class AxiosApiClient implements IApiClient {
  private client: AxiosInstance

  constructor(baseURL: string, token?: string) {
    this.client = axios.create({
      baseURL,
      timeout: 10000,
      headers: {
        'Content-Type': 'application/json',
        ...(token && { Authorization: `Bearer ${token}` })
      }
    })

    // Request interceptor
    this.client.interceptors.request.use(
      (config) => {
        // Add any request modifications here
        return config
      },
      (error) => {
        return Promise.reject(error)
      }
    )

    // Response interceptor
    this.client.interceptors.response.use(
      (response: AxiosResponse) => {
        return response
      },
      (error) => {
        // Transform axios error to our error format
        const apiError = {
          message: error.response?.data?.message || error.message,
          status: error.response?.status || 500,
          data: error.response?.data
        }
        return Promise.reject(apiError)
      }
    )
  }

  async get<T = any>(url: string, config?: ApiRequestConfig): Promise<ApiResponse<T>> {
    const response = await this.client.get<T>(url, {
      headers: config?.headers,
      params: config?.params
    })
    return this.transformResponse(response)
  }

  async post<T = any>(url: string, data?: any, config?: ApiRequestConfig): Promise<ApiResponse<T>> {
    const response = await this.client.post<T>(url, data, {
      headers: config?.headers,
      params: config?.params
    })
    return this.transformResponse(response)
  }

  async put<T = any>(url: string, data?: any, config?: ApiRequestConfig): Promise<ApiResponse<T>> {
    const response = await this.client.put<T>(url, data, {
      headers: config?.headers,
      params: config?.params
    })
    return this.transformResponse(response)
  }

  async patch<T = any>(url: string, data?: any, config?: ApiRequestConfig): Promise<ApiResponse<T>> {
    const response = await this.client.patch<T>(url, data, {
      headers: config?.headers,
      params: config?.params
    })
    return this.transformResponse(response)
  }

  async delete<T = any>(url: string, config?: ApiRequestConfig): Promise<ApiResponse<T>> {
    const response = await this.client.delete<T>(url, {
      headers: config?.headers,
      params: config?.params
    })
    return this.transformResponse(response)
  }

  private transformResponse<T>(response: AxiosResponse<T>): ApiResponse<T> {
    return {
      data: response.data,
      status: response.status,
      headers: response.headers as Record<string, string>
    }
  }

  updateToken(token: string): void {
    this.client.defaults.headers.Authorization = `Bearer ${token}`
  }

  removeToken(): void {
    delete this.client.defaults.headers.Authorization
  }
}
