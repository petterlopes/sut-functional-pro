import { IDepartmentRepository } from '../../../domain/repositories/IDepartmentRepository';
import { Department, UpdateDepartmentRequest, DepartmentId } from '../../../domain';
import { ValueObjectFactory } from '../../../domain/value-objects';

export class UpdateDepartmentUseCase {
  constructor(private departmentRepository: IDepartmentRepository) {}

  async execute(request: UpdateDepartmentRequest): Promise<Department> {
    // Find existing department
    const existingDepartment = await this.departmentRepository.findById(request.id);
    if (!existingDepartment) {
      throw new Error('Department not found');
    }

    // Update fields if provided
    const updatedDepartment: Department = {
      ...existingDepartment,
      name: request.name ? ValueObjectFactory.createDepartmentName(request.name) : existingDepartment.name,
      unitId: request.unitId ?? existingDepartment.unitId,
      updatedAt: new Date(),
    };

    return await this.departmentRepository.update(updatedDepartment);
  }
}
