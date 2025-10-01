import { IDepartmentRepository } from '../../../domain/repositories/IDepartmentRepository';
import { Department, CreateDepartmentRequest, DepartmentId } from '../../../domain';
import { ValueObjectFactory } from '../../../domain/value-objects';

export class CreateDepartmentUseCase {
  constructor(private departmentRepository: IDepartmentRepository) {}

  async execute(request: CreateDepartmentRequest): Promise<Department> {
    // Validate input
    if (!request.name?.trim()) {
      throw new Error('Name is required');
    }
    if (!request.unitId?.trim()) {
      throw new Error('Unit ID is required');
    }

    // Create value objects
    const name = ValueObjectFactory.createDepartmentName(request.name);

    // Create department entity
    const department: Department = {
      id: crypto.randomUUID(),
      unitId: request.unitId,
      name,
      createdAt: new Date(),
      updatedAt: new Date(),
    };

    return await this.departmentRepository.save(department);
  }
}
