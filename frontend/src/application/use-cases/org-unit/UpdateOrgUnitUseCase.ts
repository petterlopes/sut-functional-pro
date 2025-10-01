import { IOrgUnitRepository } from '../../../domain/repositories/IOrgUnitRepository';
import { OrgUnit, UpdateOrgUnitRequest, OrgUnitId } from '../../../domain';
import { ValueObjectFactory } from '../../../domain/value-objects';

export class UpdateOrgUnitUseCase {
  constructor(private orgUnitRepository: IOrgUnitRepository) {}

  async execute(request: UpdateOrgUnitRequest): Promise<OrgUnit> {
    // Find existing org unit
    const existingOrgUnit = await this.orgUnitRepository.findById(request.id);
    if (!existingOrgUnit) {
      throw new Error('OrgUnit not found');
    }

    // Update fields if provided
    const updatedOrgUnit: OrgUnit = {
      ...existingOrgUnit,
      name: request.name ? ValueObjectFactory.createOrgUnitName(request.name) : existingOrgUnit.name,
      parentId: request.parentId ?? existingOrgUnit.parentId,
      updatedAt: new Date(),
    };

    return await this.orgUnitRepository.update(updatedOrgUnit);
  }
}
