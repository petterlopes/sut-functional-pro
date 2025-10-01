import { IDepartmentRepository } from '../../../domain/repositories/IDepartmentRepository';
import { Department, DepartmentSearchCriteria, DepartmentSearchResult, DepartmentStatistics, DepartmentId, OrgUnitId } from '../../../domain';

export class GetDepartmentsUseCase {
  constructor(private departmentRepository: IDepartmentRepository) {}

  async execute(criteria: DepartmentSearchCriteria): Promise<DepartmentSearchResult> {
    return await this.departmentRepository.findAll(criteria);
  }

  async executeById(id: DepartmentId): Promise<Department> {
    const department = await this.departmentRepository.findById(id);
    if (!department) {
      throw new Error('Department not found');
    }
    return department;
  }

  async executeByUnit(unitId: OrgUnitId): Promise<DepartmentSearchResult> {
    const departments = await this.departmentRepository.findByUnit(unitId);
    return {
      items: departments,
      total: departments.length,
    };
  }
}

export class GetDepartmentStatisticsUseCase {
  constructor(private departmentRepository: IDepartmentRepository) {}

  async execute(): Promise<DepartmentStatistics> {
    return await this.departmentRepository.getStatistics();
  }
}
