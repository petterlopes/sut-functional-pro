import { IUserRepository } from '../../domain/repositories/IUserRepository';
import { User, UserSearchCriteria, UserSearchResult, UserId } from '../../domain';
import { IApiClient } from '../api/IApiClient';

export class UserRepository implements IUserRepository {
  constructor(private apiClient: IApiClient) {}

  async findById(id: UserId): Promise<User | null> {
    try {
      const response = await this.apiClient.get(`/v1/users/${id}`);
      return this.mapToUser(response.data);
    } catch (error) {
      if (error.response?.status === 404) {
        return null;
      }
      throw error;
    }
  }

  async findAll(criteria: UserSearchCriteria): Promise<UserSearchResult> {
    const params = new URLSearchParams();
    if (criteria.searchTerm) params.append('search_term', criteria.searchTerm);
    if (criteria.role) params.append('role', criteria.role);
    if (criteria.limit) params.append('limit', criteria.limit.toString());
    if (criteria.offset) params.append('offset', criteria.offset.toString());

    const response = await this.apiClient.get(`/v1/users?${params.toString()}`);
    return {
      items: response.data.items.map((item: any) => this.mapToUser(item)),
      total: response.data.total,
    };
  }

  async save(user: User): Promise<User> {
    const response = await this.apiClient.post('/v1/users', this.mapToCreateRequest(user));
    return this.mapToUser(response.data);
  }

  async update(user: User): Promise<User> {
    const response = await this.apiClient.patch(`/v1/users/${user.id}`, this.mapToUpdateRequest(user));
    return this.mapToUser(response.data);
  }

  async delete(id: UserId): Promise<void> {
    await this.apiClient.delete(`/v1/users/${id}`);
  }

  async findByUsername(username: string): Promise<User | null> {
    try {
      const response = await this.apiClient.get(`/v1/users/by-username/${username}`);
      return this.mapToUser(response.data);
    } catch (error) {
      if (error.response?.status === 404) {
        return null;
      }
      throw error;
    }
  }

  async findByEmail(email: string): Promise<User | null> {
    try {
      const response = await this.apiClient.get(`/v1/users/by-email/${email}`);
      return this.mapToUser(response.data);
    } catch (error) {
      if (error.response?.status === 404) {
        return null;
      }
      throw error;
    }
  }

  async findByRole(role: string): Promise<User[]> {
    const response = await this.apiClient.get(`/v1/users/by-role/${role}`);
    return response.data.items.map((item: any) => this.mapToUser(item));
  }

  private mapToUser(data: any): User {
    return {
      id: data.id,
      username: { value: data.username },
      email: { value: data.email },
      password: data.password,
      roles: data.roles.map((role: string) => ({ value: role })),
      createdAt: new Date(data.created_at),
      updatedAt: new Date(data.updated_at),
    };
  }

  private mapToCreateRequest(user: User): any {
    return {
      username: user.username.value,
      email: user.email.value,
      password: user.password,
      roles: user.roles.map(role => role.value),
    };
  }

  private mapToUpdateRequest(user: User): any {
    return {
      username: user.username.value,
      email: user.email.value,
      password: user.password,
      roles: user.roles.map(role => role.value),
    };
  }
}
