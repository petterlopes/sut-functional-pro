import { UserId, Username, UserEmail, Role } from '../value-objects';

export interface User {
  id: UserId;
  username: Username;
  email: UserEmail;
  password: string; // Note: In real app, this should be hashed
  roles: Role[];
  createdAt: Date;
  updatedAt: Date;
}

export interface CreateUserRequest {
  username: string;
  email: string;
  password: string;
  roles: string[];
}

export interface UpdateUserRequest {
  id: string;
  username?: string;
  email?: string;
  password?: string;
  roles?: string[];
}

export interface UserSearchCriteria {
  searchTerm?: string;
  role?: string;
  limit?: number;
  offset?: number;
}

export interface UserSearchResult {
  items: User[];
  total: number;
}
