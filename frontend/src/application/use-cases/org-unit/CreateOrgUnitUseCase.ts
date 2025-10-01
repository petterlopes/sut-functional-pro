import { IOrgUnitRepository } from '../../../domain/repositories/IOrgUnitRepository';
import { OrgUnit, CreateOrgUnitRequest, OrgUnitId } from '../../../domain';
import { ValueObjectFactory } from '../../../domain/value-objects';

export class CreateOrgUnitUseCase {
  constructor(private orgUnitRepository: IOrgUnitRepository) {}

  async execute(request: CreateOrgUnitRequest): Promise<OrgUnit> {
    // Validate input
    if (!request.name?.trim()) {
      throw new Error('Name is required');
    }

    // Create value objects
    const name = ValueObjectFactory.createOrgUnitName(request.name);

    // Create org unit entity
    const orgUnit: OrgUnit = {
      id: crypto.randomUUID(),
      name,
      parentId: request.parentId,
      createdAt: new Date(),
      updatedAt: new Date(),
    };

    return await this.orgUnitRepository.save(orgUnit);
  }
}
