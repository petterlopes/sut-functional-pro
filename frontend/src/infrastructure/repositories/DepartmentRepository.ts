import { IDepartmentRepository } from '../../domain/repositories/IDepartmentRepository';
import { Department, DepartmentSearchCriteria, DepartmentSearchResult, DepartmentStatistics, DepartmentId, OrgUnitId } from '../../domain';
import { IApiClient } from '../api/IApiClient';

export class DepartmentRepository implements IDepartmentRepository {
  constructor(private apiClient: IApiClient) {}

  async findById(id: DepartmentId): Promise<Department | null> {
    try {
      const response = await this.apiClient.get(`/v1/departments/${id}`);
      return this.mapToDepartment(response.data);
    } catch (error) {
      if (error.response?.status === 404) {
        return null;
      }
      throw error;
    }
  }

  async findAll(criteria: DepartmentSearchCriteria): Promise<DepartmentSearchResult> {
    const params = new URLSearchParams();
    if (criteria.searchTerm) params.append('search_term', criteria.searchTerm);
    if (criteria.unitId) params.append('unit_id', criteria.unitId);
    if (criteria.limit) params.append('limit', criteria.limit.toString());
    if (criteria.offset) params.append('offset', criteria.offset.toString());

    const response = await this.apiClient.get(`/v1/departments?${params.toString()}`);
    return {
      items: response.data.items.map((item: any) => this.mapToDepartment(item)),
      total: response.data.total,
    };
  }

  async save(department: Department): Promise<Department> {
    const response = await this.apiClient.post('/v1/departments', this.mapToCreateRequest(department));
    return this.mapToDepartment(response.data);
  }

  async update(department: Department): Promise<Department> {
    const response = await this.apiClient.patch(`/v1/departments/${department.id}`, this.mapToUpdateRequest(department));
    return this.mapToDepartment(response.data);
  }

  async delete(id: DepartmentId): Promise<void> {
    await this.apiClient.delete(`/v1/departments/${id}`);
  }

  async findByName(name: string): Promise<Department[]> {
    const result = await this.findAll({ searchTerm: name });
    return result.items;
  }

  async findByUnit(unitId: OrgUnitId): Promise<Department[]> {
    const response = await this.apiClient.get(`/v1/departments/by-unit/${unitId}`);
    return response.data.items.map((item: any) => this.mapToDepartment(item));
  }

  async getStatistics(): Promise<DepartmentStatistics> {
    const response = await this.apiClient.get('/v1/departments/statistics');
    return response.data;
  }

  private mapToDepartment(data: any): Department {
    return {
      id: data.id,
      unitId: data.unit_id,
      name: { value: data.name },
      createdAt: new Date(data.created_at),
      updatedAt: new Date(data.updated_at),
    };
  }

  private mapToCreateRequest(department: Department): any {
    return {
      unit_id: department.unitId,
      name: department.name.value,
    };
  }

  private mapToUpdateRequest(department: Department): any {
    return {
      unit_id: department.unitId,
      name: department.name.value,
    };
  }
}
