import { OrgUnit, OrgUnitSearchCriteria, OrgUnitSearchResult, OrgUnitHierarchy } from '../entities/OrgUnit';
import { OrgUnitId } from '../value-objects';

export interface IOrgUnitRepository {
  findById(id: OrgUnitId): Promise<OrgUnit | null>;
  findAll(criteria: OrgUnitSearchCriteria): Promise<OrgUnitSearchResult>;
  save(orgUnit: OrgUnit): Promise<OrgUnit>;
  update(orgUnit: OrgUnit): Promise<OrgUnit>;
  delete(id: OrgUnitId): Promise<void>;
  findByName(name: string): Promise<OrgUnit[]>;
  findChildren(parentId: OrgUnitId): Promise<OrgUnit[]>;
  findRootUnits(): Promise<OrgUnit[]>;
  getHierarchy(id: OrgUnitId): Promise<OrgUnitHierarchy>;
}
