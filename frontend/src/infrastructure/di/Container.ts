// Dependency Injection Container
export type Constructor<T = {}> = new (...args: any[]) => T
export type Factory<T = any> = (...args: any[]) => T

export interface ServiceDefinition<T = any> {
  factory: Factory<T>
  singleton?: boolean
  dependencies?: string[]
}

export class Container {
  private services = new Map<string, ServiceDefinition>()
  private instances = new Map<string, any>()

  // Register a service
  register<T>(name: string, factory: Factory<T>, options?: { singleton?: boolean; dependencies?: string[] }): void {
    this.services.set(name, {
      factory,
      singleton: options?.singleton ?? true,
      dependencies: options?.dependencies ?? []
    })
  }

  // Register a class
  registerClass<T>(name: string, constructor: Constructor<T>, options?: { singleton?: boolean; dependencies?: string[] }): void {
    this.register(name, (...args: any[]) => new constructor(...args), options)
  }

  // Register an instance
  registerInstance<T>(name: string, instance: T): void {
    this.instances.set(name, instance)
  }

  // Resolve a service
  resolve<T>(name: string): T {
    // Check if instance already exists
    if (this.instances.has(name)) {
      return this.instances.get(name)
    }

    const service = this.services.get(name)
    if (!service) {
      throw new Error(`Service '${name}' not found`)
    }

    // Resolve dependencies
    const dependencies = service.dependencies?.map(dep => this.resolve(dep)) ?? []

    // Create instance
    const instance = service.factory(...dependencies)

    // Store instance if singleton
    if (service.singleton) {
      this.instances.set(name, instance)
    }

    return instance
  }

  // Check if service is registered
  isRegistered(name: string): boolean {
    return this.services.has(name) || this.instances.has(name)
  }

  // Clear all services and instances
  clear(): void {
    this.services.clear()
    this.instances.clear()
  }
}

// Global container instance
export const container = new Container()
