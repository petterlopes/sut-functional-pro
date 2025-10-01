import { IOrgUnitRepository } from '../../../domain/repositories/IOrgUnitRepository';
import { OrgUnitId } from '../../../domain';

export class DeleteOrgUnitUseCase {
  constructor(private orgUnitRepository: IOrgUnitRepository) {}

  async execute(id: OrgUnitId): Promise<void> {
    // Check if org unit exists
    const existingOrgUnit = await this.orgUnitRepository.findById(id);
    if (!existingOrgUnit) {
      throw new Error('OrgUnit not found');
    }

    // Check if org unit has children
    const children = await this.orgUnitRepository.findChildren(id);
    if (children.length > 0) {
      throw new Error('Cannot delete org unit with children');
    }

    await this.orgUnitRepository.delete(id);
  }
}
