import { IUserRepository } from '../../../domain/repositories/IUserRepository';
import { User, CreateUserRequest, UserId } from '../../../domain';
import { ValueObjectFactory } from '../../../domain/value-objects';

export class CreateUserUseCase {
  constructor(private userRepository: IUserRepository) {}

  async execute(request: CreateUserRequest): Promise<User> {
    // Validate input
    if (!request.username?.trim()) {
      throw new Error('Username is required');
    }
    if (!request.email?.trim()) {
      throw new Error('Email is required');
    }
    if (!request.password?.trim()) {
      throw new Error('Password is required');
    }

    // Create value objects
    const username = ValueObjectFactory.createUsername(request.username);
    const email = ValueObjectFactory.createUserEmail(request.email);
    const roles = request.roles?.map(role => ValueObjectFactory.createRole(role)) || [];

    // Create user entity
    const user: User = {
      id: crypto.randomUUID(),
      username,
      email,
      password: request.password, // Note: In real app, this should be hashed
      roles,
      createdAt: new Date(),
      updatedAt: new Date(),
    };

    return await this.userRepository.save(user);
  }
}
