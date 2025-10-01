import { OrgUnitId, OrgUnitName } from '../value-objects';

export interface OrgUnit {
  id: OrgUnitId;
  name: OrgUnitName;
  parentId?: OrgUnitId;
  createdAt: Date;
  updatedAt: Date;
}

export interface CreateOrgUnitRequest {
  name: string;
  parentId?: string;
}

export interface UpdateOrgUnitRequest {
  id: string;
  name?: string;
  parentId?: string;
}

export interface OrgUnitSearchCriteria {
  searchTerm?: string;
  parentId?: string;
  limit?: number;
  offset?: number;
}

export interface OrgUnitSearchResult {
  items: OrgUnit[];
  total: number;
}

export interface OrgUnitHierarchy {
  items: OrgUnit[];
  children: Record<string, OrgUnit[]>;
}
