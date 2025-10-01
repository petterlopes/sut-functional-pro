// ============================================================================
// AUTHENTICATION API CLIENT
// ============================================================================
// Cliente de API com autenticação JWT integrada
// Implementa interceptors para adicionar tokens automaticamente

import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse, AxiosError } from 'axios';

// Configuração base da API
const API_BASE_URL = import.meta.env.VITE_API_BASE || 'http://localhost:8080';

// Interface para configuração de autenticação
interface AuthConfig {
  token: string | null;
  onTokenExpired?: () => void;
  onUnauthorized?: () => void;
}

// Classe para cliente de API autenticado
export class AuthenticatedApiClient {
  private client: AxiosInstance;
  private authConfig: AuthConfig;

  constructor(authConfig: AuthConfig) {
    this.authConfig = authConfig;
    
    // Criar instância do Axios
    this.client = axios.create({
      baseURL: API_BASE_URL,
      timeout: 30000, // 30 segundos
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
    });

    // Configurar interceptors
    this.setupRequestInterceptor();
    this.setupResponseInterceptor();
  }

  // Interceptor de requisição para adicionar token
  private setupRequestInterceptor(): void {
    this.client.interceptors.request.use(
      (config: AxiosRequestConfig) => {
        // Adicionar token de autorização se disponível
        if (this.authConfig.token) {
          config.headers = {
            ...config.headers,
            'Authorization': `Bearer ${this.authConfig.token}`,
          };
        }

        // Adicionar headers de segurança
        config.headers = {
          ...config.headers,
          'X-Requested-With': 'XMLHttpRequest',
          'X-Client-Version': '1.0.0',
        };

        // Log da requisição em desenvolvimento
        if (process.env.NODE_ENV === 'development') {
          console.debug('[API] Request:', {
            method: config.method?.toUpperCase(),
            url: config.url,
            baseURL: config.baseURL,
            headers: config.headers,
          });
        }

        return config;
      },
      (error: AxiosError) => {
        console.error('[API] Request error:', error);
        return Promise.reject(error);
      }
    );
  }

  // Interceptor de resposta para tratar erros de autenticação
  private setupResponseInterceptor(): void {
    this.client.interceptors.response.use(
      (response: AxiosResponse) => {
        // Log da resposta em desenvolvimento
        if (process.env.NODE_ENV === 'development') {
          console.debug('[API] Response:', {
            status: response.status,
            statusText: response.statusText,
            url: response.config.url,
            data: response.data,
          });
        }

        return response;
      },
      (error: AxiosError) => {
        const status = error.response?.status;
        const config = error.config as AxiosRequestConfig & { _retry?: boolean };

        // Log do erro
        console.error('[API] Response error:', {
          status,
          statusText: error.response?.statusText,
          url: config?.url,
          message: error.message,
          data: error.response?.data,
        });

        // Tratar erros de autenticação
        if (status === 401) {
          if (this.authConfig.onTokenExpired) {
            this.authConfig.onTokenExpired();
          }
        } else if (status === 403) {
          if (this.authConfig.onUnauthorized) {
            this.authConfig.onUnauthorized();
          }
        }

        return Promise.reject(error);
      }
    );
  }

  // Métodos HTTP
  async get<T = any>(url: string, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.get<T>(url, config);
    return response.data;
  }

  async post<T = any>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.post<T>(url, data, config);
    return response.data;
  }

  async put<T = any>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.put<T>(url, data, config);
    return response.data;
  }

  async patch<T = any>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.patch<T>(url, data, config);
    return response.data;
  }

  async delete<T = any>(url: string, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.delete<T>(url, config);
    return response.data;
  }

  // Atualizar token de autenticação
  updateToken(token: string | null): void {
    this.authConfig.token = token;
  }

  // Obter instância do cliente Axios (para casos especiais)
  getClient(): AxiosInstance {
    return this.client;
  }
}

// Instância global do cliente de API
let apiClient: AuthenticatedApiClient | null = null;

// Função para inicializar o cliente de API
export function initializeApiClient(authConfig: AuthConfig): AuthenticatedApiClient {
  apiClient = new AuthenticatedApiClient(authConfig);
  return apiClient;
}

// Função para obter o cliente de API
export function getApiClient(): AuthenticatedApiClient {
  if (!apiClient) {
    throw new Error('API client not initialized. Call initializeApiClient first.');
  }
  return apiClient;
}

// Função para atualizar o token no cliente global
export function updateApiToken(token: string | null): void {
  if (apiClient) {
    apiClient.updateToken(token);
  }
}

// Hook para usar o cliente de API
export function useApiClient(): AuthenticatedApiClient {
  return getApiClient();
}

// Utilitários para tratamento de erros
export class ApiError extends Error {
  public status?: number;
  public statusText?: string;
  public data?: any;

  constructor(message: string, status?: number, statusText?: string, data?: any) {
    super(message);
    this.name = 'ApiError';
    this.status = status;
    this.statusText = statusText;
    this.data = data;
  }
}

// Função para tratar erros de API
export function handleApiError(error: any): ApiError {
  if (axios.isAxiosError(error)) {
    const status = error.response?.status;
    const statusText = error.response?.statusText;
    const data = error.response?.data;
    
    let message = error.message;
    if (data?.error) {
      message = data.error;
    } else if (data?.message) {
      message = data.message;
    }

    return new ApiError(message, status, statusText, data);
  }
  
  return new ApiError(error.message || 'Unknown error');
}

// Configurações de segurança para requisições
export const SECURITY_CONFIG = {
  // Headers de segurança
  SECURITY_HEADERS: {
    'X-Content-Type-Options': 'nosniff',
    'X-Frame-Options': 'DENY',
    'X-XSS-Protection': '1; mode=block',
  },
  
  // Configurações de timeout
  TIMEOUTS: {
    DEFAULT: 30000,      // 30 segundos
    UPLOAD: 300000,      // 5 minutos
    DOWNLOAD: 600000,    // 10 minutos
  },
  
  // Configurações de retry
  RETRY: {
    MAX_ATTEMPTS: 3,
    DELAY: 1000,         // 1 segundo
    BACKOFF_FACTOR: 2,
  },
} as const;
