import { DepartmentId, DepartmentName, OrgUnitId } from '../value-objects';

export interface Department {
  id: DepartmentId;
  unitId: OrgUnitId;
  name: DepartmentName;
  createdAt: Date;
  updatedAt: Date;
}

export interface CreateDepartmentRequest {
  unitId: string;
  name: string;
}

export interface UpdateDepartmentRequest {
  id: string;
  unitId?: string;
  name?: string;
}

export interface DepartmentSearchCriteria {
  searchTerm?: string;
  unitId?: string;
  limit?: number;
  offset?: number;
}

export interface DepartmentSearchResult {
  items: Department[];
  total: number;
}

export interface DepartmentStatistics {
  totalDepartments: number;
  departmentsByUnit: Record<string, number>;
}
