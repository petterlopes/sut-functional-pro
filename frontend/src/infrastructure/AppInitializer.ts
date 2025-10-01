import { ServiceRegistry } from './di/ServiceRegistry'

export class AppInitializer {
  private static initialized = false

  static initialize(baseURL: string, token?: string): void {
    if (this.initialized) {
      // If already initialized, just update the token
      if (token) {
        this.updateToken(token)
      } else {
        this.removeToken()
      }
      return
    }

    // Register all services
    ServiceRegistry.registerServices(baseURL, token)
    
    this.initialized = true
  }

  static updateToken(token: string): void {
    if (!this.initialized) {
      console.warn('AppInitializer not initialized. Call initialize() first.')
      return
    }
    ServiceRegistry.updateToken(token)
  }

  static removeToken(): void {
    if (!this.initialized) {
      console.warn('AppInitializer not initialized. Call initialize() first.')
      return
    }
    ServiceRegistry.removeToken()
  }

  static reset(): void {
    ServiceRegistry.clear()
    this.initialized = false
  }
}
