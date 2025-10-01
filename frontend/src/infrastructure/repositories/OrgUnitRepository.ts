import { IOrgUnitRepository } from '../../domain/repositories/IOrgUnitRepository';
import { OrgUnit, OrgUnitSearchCriteria, OrgUnitSearchResult, OrgUnitHierarchy, OrgUnitId } from '../../domain';
import { IApiClient } from '../api/IApiClient';

export class OrgUnitRepository implements IOrgUnitRepository {
  constructor(private apiClient: IApiClient) {}

  async findById(id: OrgUnitId): Promise<OrgUnit | null> {
    try {
      const response = await this.apiClient.get(`/v1/org-units/${id}`);
      return this.mapToOrgUnit(response.data);
    } catch (error) {
      if (error.response?.status === 404) {
        return null;
      }
      throw error;
    }
  }

  async findAll(criteria: OrgUnitSearchCriteria): Promise<OrgUnitSearchResult> {
    const params = new URLSearchParams();
    if (criteria.searchTerm) params.append('search_term', criteria.searchTerm);
    if (criteria.parentId) params.append('parent_id', criteria.parentId);
    if (criteria.limit) params.append('limit', criteria.limit.toString());
    if (criteria.offset) params.append('offset', criteria.offset.toString());

    const response = await this.apiClient.get(`/v1/org-units?${params.toString()}`);
    return {
      items: response.data.items.map((item: any) => this.mapToOrgUnit(item)),
      total: response.data.total,
    };
  }

  async save(orgUnit: OrgUnit): Promise<OrgUnit> {
    const response = await this.apiClient.post('/v1/org-units', this.mapToCreateRequest(orgUnit));
    return this.mapToOrgUnit(response.data);
  }

  async update(orgUnit: OrgUnit): Promise<OrgUnit> {
    const response = await this.apiClient.patch(`/v1/org-units/${orgUnit.id}`, this.mapToUpdateRequest(orgUnit));
    return this.mapToOrgUnit(response.data);
  }

  async delete(id: OrgUnitId): Promise<void> {
    await this.apiClient.delete(`/v1/org-units/${id}`);
  }

  async findByName(name: string): Promise<OrgUnit[]> {
    const result = await this.findAll({ searchTerm: name });
    return result.items;
  }

  async findChildren(parentId: OrgUnitId): Promise<OrgUnit[]> {
    const result = await this.findAll({ parentId });
    return result.items;
  }

  async findRootUnits(): Promise<OrgUnit[]> {
    const result = await this.findAll({ parentId: undefined });
    return result.items;
  }

  async getHierarchy(id: OrgUnitId): Promise<OrgUnitHierarchy> {
    const response = await this.apiClient.get(`/v1/org-units/${id}/hierarchy`);
    return {
      items: response.data.items.map((item: any) => this.mapToOrgUnit(item)),
      children: response.data.children,
    };
  }

  private mapToOrgUnit(data: any): OrgUnit {
    return {
      id: data.id,
      name: { value: data.name },
      parentId: data.parent_id,
      createdAt: new Date(data.created_at),
      updatedAt: new Date(data.updated_at),
    };
  }

  private mapToCreateRequest(orgUnit: OrgUnit): any {
    return {
      name: orgUnit.name.value,
      parent_id: orgUnit.parentId,
    };
  }

  private mapToUpdateRequest(orgUnit: OrgUnit): any {
    return {
      name: orgUnit.name.value,
      parent_id: orgUnit.parentId,
    };
  }
}
