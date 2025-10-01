// Domain Entity - Localidade
export interface LocalidadeId {
  value: string
}

export interface LocalidadeType {
  readonly value: 'ESTADO' | 'DISTRITO_FEDERAL' | 'CIDADE' | 'BAIRRO'
}

export interface LocalidadeProps {
  id: LocalidadeId
  name: string
  type: LocalidadeType
  parentId?: LocalidadeId
  population?: number
  area?: number
  createdAt: Date
  updatedAt: Date
}

export class Localidade {
  private constructor(private props: LocalidadeProps) {}

  static create(props: Omit<LocalidadeProps, 'id' | 'createdAt' | 'updatedAt'>): Localidade {
    const now = new Date()
    return new Localidade({
      ...props,
      id: { value: crypto.randomUUID() },
      createdAt: now,
      updatedAt: now
    })
  }

  static fromPersistence(data: any): Localidade {
    return new Localidade({
      id: { value: data.id },
      name: data.name,
      type: { value: data.type },
      parentId: data.parent_id ? { value: data.parent_id } : undefined,
      population: data.population,
      area: data.area,
      createdAt: new Date(data.created_at),
      updatedAt: new Date(data.updated_at)
    })
  }

  // Getters
  get id(): LocalidadeId { return this.props.id }
  get name(): string { return this.props.name }
  get type(): LocalidadeType { return this.props.type }
  get parentId(): LocalidadeId | undefined { return this.props.parentId }
  get population(): number | undefined { return this.props.population }
  get area(): number | undefined { return this.props.area }
  get createdAt(): Date { return this.props.createdAt }
  get updatedAt(): Date { return this.props.updatedAt }

  // Business methods
  updateName(name: string): void {
    if (!name.trim()) {
      throw new Error('Name cannot be empty')
    }
    this.props.name = name.trim()
    this.props.updatedAt = new Date()
  }

  updatePopulation(population: number): void {
    if (population < 0) {
      throw new Error('Population cannot be negative')
    }
    this.props.population = population
    this.props.updatedAt = new Date()
  }

  updateArea(area: number): void {
    if (area < 0) {
      throw new Error('Area cannot be negative')
    }
    this.props.area = area
    this.props.updatedAt = new Date()
  }

  setParent(parentId: LocalidadeId): void {
    this.props.parentId = parentId
    this.props.updatedAt = new Date()
  }

  removeParent(): void {
    this.props.parentId = undefined
    this.props.updatedAt = new Date()
  }

  toPersistence(): any {
    return {
      id: this.props.id.value,
      name: this.props.name,
      type: this.props.type.value,
      parent_id: this.props.parentId?.value,
      population: this.props.population,
      area: this.props.area,
      created_at: this.props.createdAt.toISOString(),
      updated_at: this.props.updatedAt.toISOString()
    }
  }

  equals(other: Localidade): boolean {
    return this.props.id.value === other.props.id.value
  }
}
