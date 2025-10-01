import { Departamento } from '../entities/Departamento'

export interface DepartamentoSearchCriteria {
  name?: string
  unitId?: string
  status?: string
  limit?: number
  offset?: number
}

export interface DepartamentoSearchResult {
  items: Departamento[]
  total: number
  hasMore: boolean
}

export interface IDepartamentoRepository {
  // Basic CRUD operations
  findById(id: string): Promise<Departamento | null>
  findAll(criteria?: DepartamentoSearchCriteria): Promise<DepartamentoSearchResult>
  save(departamento: Departamento): Promise<Departamento>
  update(departamento: Departamento): Promise<Departamento>
  delete(id: string): Promise<void>

  // Business operations
  findByName(name: string): Promise<Departamento[]>
  findByUnit(unitId: string): Promise<Departamento[]>
  findByStatus(status: string): Promise<Departamento[]>
  findActive(): Promise<Departamento[]>

  // Statistics
  countByUnit(unitId: string): Promise<number>
  countByStatus(status: string): Promise<number>
  getTotalBudget(): Promise<number>
  getTotalEmployees(): Promise<number>
  getStatistics(): Promise<{
    total: number
    active: number
    inactive: number
    totalBudget: number
    totalEmployees: number
  }>
}
