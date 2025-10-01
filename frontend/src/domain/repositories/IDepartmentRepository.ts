import { Department, DepartmentSearchCriteria, DepartmentSearchResult, DepartmentStatistics } from '../entities/Department';
import { DepartmentId, OrgUnitId } from '../value-objects';

export interface IDepartmentRepository {
  findById(id: DepartmentId): Promise<Department | null>;
  findAll(criteria: DepartmentSearchCriteria): Promise<DepartmentSearchResult>;
  save(department: Department): Promise<Department>;
  update(department: Department): Promise<Department>;
  delete(id: DepartmentId): Promise<void>;
  findByName(name: string): Promise<Department[]>;
  findByUnit(unitId: OrgUnitId): Promise<Department[]>;
  getStatistics(): Promise<DepartmentStatistics>;
}
