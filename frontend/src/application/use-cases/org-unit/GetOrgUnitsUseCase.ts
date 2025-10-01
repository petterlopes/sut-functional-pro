import { IOrgUnitRepository } from '../../../domain/repositories/IOrgUnitRepository';
import { OrgUnit, OrgUnitSearchCriteria, OrgUnitSearchResult, OrgUnitHierarchy, OrgUnitId } from '../../../domain';

export class GetOrgUnitsUseCase {
  constructor(private orgUnitRepository: IOrgUnitRepository) {}

  async execute(criteria: OrgUnitSearchCriteria): Promise<OrgUnitSearchResult> {
    return await this.orgUnitRepository.findAll(criteria);
  }

  async executeById(id: OrgUnitId): Promise<OrgUnit> {
    const orgUnit = await this.orgUnitRepository.findById(id);
    if (!orgUnit) {
      throw new Error('OrgUnit not found');
    }
    return orgUnit;
  }

  async executeHierarchy(id: OrgUnitId): Promise<OrgUnitHierarchy> {
    return await this.orgUnitRepository.getHierarchy(id);
  }
}
