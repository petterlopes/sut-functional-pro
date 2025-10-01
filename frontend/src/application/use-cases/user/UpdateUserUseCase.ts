import { IUserRepository } from '../../../domain/repositories/IUserRepository';
import { User, UpdateUserRequest, UserId } from '../../../domain';
import { ValueObjectFactory } from '../../../domain/value-objects';

export class UpdateUserUseCase {
  constructor(private userRepository: IUserRepository) {}

  async execute(request: UpdateUserRequest): Promise<User> {
    // Find existing user
    const existingUser = await this.userRepository.findById(request.id);
    if (!existingUser) {
      throw new Error('User not found');
    }

    // Update fields if provided
    const updatedUser: User = {
      ...existingUser,
      username: request.username ? ValueObjectFactory.createUsername(request.username) : existingUser.username,
      email: request.email ? ValueObjectFactory.createUserEmail(request.email) : existingUser.email,
      password: request.password ?? existingUser.password,
      roles: request.roles?.map(role => ValueObjectFactory.createRole(role)) ?? existingUser.roles,
      updatedAt: new Date(),
    };

    return await this.userRepository.update(updatedUser);
  }
}
