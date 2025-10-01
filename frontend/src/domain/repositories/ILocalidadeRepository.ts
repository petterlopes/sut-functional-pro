import { Localidade } from '../entities/Localidade'

export interface LocalidadeSearchCriteria {
  name?: string
  type?: string
  parentId?: string
  limit?: number
  offset?: number
}

export interface LocalidadeSearchResult {
  items: Localidade[]
  total: number
  hasMore: boolean
}

export interface ILocalidadeRepository {
  // Basic CRUD operations
  findById(id: string): Promise<Localidade | null>
  findAll(criteria?: LocalidadeSearchCriteria): Promise<LocalidadeSearchResult>
  save(localidade: Localidade): Promise<Localidade>
  update(localidade: Localidade): Promise<Localidade>
  delete(id: string): Promise<void>

  // Business operations
  findByName(name: string): Promise<Localidade[]>
  findByType(type: string): Promise<Localidade[]>
  findByParent(parentId: string): Promise<Localidade[]>
  findRoots(): Promise<Localidade[]> // Estados e Distrito Federal
  findHierarchy(id: string): Promise<Localidade[]> // Hierarquia completa

  // Statistics
  countByType(type: string): Promise<number>
  getTotalPopulation(): Promise<number>
  getTotalArea(): Promise<number>
}
