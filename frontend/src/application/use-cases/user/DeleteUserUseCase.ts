import { IUserRepository } from '../../../domain/repositories/IUserRepository';
import { UserId } from '../../../domain';

export class DeleteUserUseCase {
  constructor(private userRepository: IUserRepository) {}

  async execute(id: UserId): Promise<void> {
    // Check if user exists
    const existingUser = await this.userRepository.findById(id);
    if (!existingUser) {
      throw new Error('User not found');
    }

    await this.userRepository.delete(id);
  }
}
