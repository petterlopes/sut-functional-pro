// Domain Entity - Departamento
export interface DepartamentoId {
  value: string
}

export interface DepartamentoStatus {
  readonly value: 'ATIVO' | 'INATIVO' | 'SUSPENSO'
}

export interface DepartamentoProps {
  id: DepartamentoId
  name: string
  unitId: string
  budget?: number
  employees?: number
  status: DepartamentoStatus
  createdAt: Date
  updatedAt: Date
}

export class Departamento {
  private constructor(private props: DepartamentoProps) {}

  static create(props: Omit<DepartamentoProps, 'id' | 'createdAt' | 'updatedAt'>): Departamento {
    const now = new Date()
    return new Departamento({
      ...props,
      id: { value: crypto.randomUUID() },
      createdAt: now,
      updatedAt: now
    })
  }

  static fromPersistence(data: any): Departamento {
    return new Departamento({
      id: { value: data.id },
      name: data.name,
      unitId: data.unit_id,
      budget: data.budget,
      employees: data.employees,
      status: { value: data.status },
      createdAt: new Date(data.created_at),
      updatedAt: new Date(data.updated_at)
    })
  }

  // Getters
  get id(): DepartamentoId { return this.props.id }
  get name(): string { return this.props.name }
  get unitId(): string { return this.props.unitId }
  get budget(): number | undefined { return this.props.budget }
  get employees(): number | undefined { return this.props.employees }
  get status(): DepartamentoStatus { return this.props.status }
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

  updateBudget(budget: number): void {
    if (budget < 0) {
      throw new Error('Budget cannot be negative')
    }
    this.props.budget = budget
    this.props.updatedAt = new Date()
  }

  updateEmployees(employees: number): void {
    if (employees < 0) {
      throw new Error('Employee count cannot be negative')
    }
    this.props.employees = employees
    this.props.updatedAt = new Date()
  }

  updateStatus(status: DepartamentoStatus): void {
    this.props.status = status
    this.props.updatedAt = new Date()
  }

  isActive(): boolean {
    return this.props.status.value === 'ATIVO'
  }

  toPersistence(): any {
    return {
      id: this.props.id.value,
      name: this.props.name,
      unit_id: this.props.unitId,
      budget: this.props.budget,
      employees: this.props.employees,
      status: this.props.status.value,
      created_at: this.props.createdAt.toISOString(),
      updated_at: this.props.updatedAt.toISOString()
    }
  }

  equals(other: Departamento): boolean {
    return this.props.id.value === other.props.id.value
  }
}
