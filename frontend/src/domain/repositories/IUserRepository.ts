import { User, UserSearchCriteria, UserSearchResult } from '../entities/User';
import { UserId } from '../value-objects';

export interface IUserRepository {
  findById(id: UserId): Promise<User | null>;
  findAll(criteria: UserSearchCriteria): Promise<UserSearchResult>;
  save(user: User): Promise<User>;
  update(user: User): Promise<User>;
  delete(id: UserId): Promise<void>;
  findByUsername(username: string): Promise<User | null>;
  findByEmail(email: string): Promise<User | null>;
  findByRole(role: string): Promise<User[]>;
}
