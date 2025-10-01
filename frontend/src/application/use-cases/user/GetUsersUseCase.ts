import { IUserRepository } from '../../../domain/repositories/IUserRepository';
import { User, UserSearchCriteria, UserSearchResult, UserId } from '../../../domain';

export class GetUsersUseCase {
  constructor(private userRepository: IUserRepository) {}

  async execute(criteria: UserSearchCriteria): Promise<UserSearchResult> {
    return await this.userRepository.findAll(criteria);
  }

  async executeById(id: UserId): Promise<User> {
    const user = await this.userRepository.findById(id);
    if (!user) {
      throw new Error('User not found');
    }
    return user;
  }

  async executeByUsername(username: string): Promise<User> {
    const user = await this.userRepository.findByUsername(username);
    if (!user) {
      throw new Error('User not found');
    }
    return user;
  }

  async executeByEmail(email: string): Promise<User> {
    const user = await this.userRepository.findByEmail(email);
    if (!user) {
      throw new Error('User not found');
    }
    return user;
  }

  async executeByRole(role: string): Promise<UserSearchResult> {
    const users = await this.userRepository.findByRole(role);
    return {
      items: users,
      total: users.length,
    };
  }
}
