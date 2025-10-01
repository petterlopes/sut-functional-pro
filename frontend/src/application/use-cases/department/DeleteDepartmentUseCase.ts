import { IDepartmentRepository } from '../../../domain/repositories/IDepartmentRepository';
import { DepartmentId } from '../../../domain';

export class DeleteDepartmentUseCase {
  constructor(private departmentRepository: IDepartmentRepository) {}

  async execute(id: DepartmentId): Promise<void> {
    // Check if department exists
    const existingDepartment = await this.departmentRepository.findById(id);
    if (!existingDepartment) {
      throw new Error('Department not found');
    }

    await this.departmentRepository.delete(id);
  }
}
