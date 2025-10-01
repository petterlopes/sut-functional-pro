import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { container } from '../../infrastructure/di/Container'
import { SERVICE_NAMES } from '../../infrastructure/di/ServiceRegistry'
import { CreateUserUseCase, CreateUserRequest } from '../../application/use-cases/user/CreateUserUseCase'
import { UpdateUserUseCase, UpdateUserRequest } from '../../application/use-cases/user/UpdateUserUseCase'
import { DeleteUserUseCase } from '../../application/use-cases/user/DeleteUserUseCase'
import { GetUsersUseCase } from '../../application/use-cases/user/GetUsersUseCase'
import { User, UserSearchCriteria } from '../../domain'

export function useUsers(request?: UserSearchCriteria) {
  return useQuery({
    queryKey: ['users', request],
    queryFn: async () => {
      const useCase = container.resolve<GetUsersUseCase>(SERVICE_NAMES.GET_USERS_USE_CASE)
      return await useCase.execute(request || {})
    },
  })
}

export function useUser(id: string) {
  return useQuery({
    queryKey: ['user', id],
    queryFn: async () => {
      const useCase = container.resolve<GetUsersUseCase>(SERVICE_NAMES.GET_USERS_USE_CASE)
      return await useCase.executeById(id)
    },
    enabled: !!id,
  })
}

export function useUserByUsername(username: string) {
  return useQuery({
    queryKey: ['user-by-username', username],
    queryFn: async () => {
      const useCase = container.resolve<GetUsersUseCase>(SERVICE_NAMES.GET_USERS_USE_CASE)
      return await useCase.executeByUsername(username)
    },
    enabled: !!username,
  })
}

export function useUserByEmail(email: string) {
  return useQuery({
    queryKey: ['user-by-email', email],
    queryFn: async () => {
      const useCase = container.resolve<GetUsersUseCase>(SERVICE_NAMES.GET_USERS_USE_CASE)
      return await useCase.executeByEmail(email)
    },
    enabled: !!email,
  })
}

export function useUsersByRole(role: string) {
  return useQuery({
    queryKey: ['users-by-role', role],
    queryFn: async () => {
      const useCase = container.resolve<GetUsersUseCase>(SERVICE_NAMES.GET_USERS_USE_CASE)
      return await useCase.executeByRole(role)
    },
    enabled: !!role,
  })
}

export function useCreateUser() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: async (request: CreateUserRequest) => {
      const useCase = container.resolve<CreateUserUseCase>(SERVICE_NAMES.CREATE_USER_USE_CASE)
      return await useCase.execute(request)
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['users'] })
    },
  })
}

export function useUpdateUser() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: async (request: UpdateUserRequest) => {
      const useCase = container.resolve<UpdateUserUseCase>(SERVICE_NAMES.UPDATE_USER_USE_CASE)
      return await useCase.execute(request)
    },
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['users'] })
      queryClient.invalidateQueries({ queryKey: ['user', variables.id] })
    },
  })
}

export function useDeleteUser() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: async (id: string) => {
      const useCase = container.resolve<DeleteUserUseCase>(SERVICE_NAMES.DELETE_USER_USE_CASE)
      return await useCase.execute(id)
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['users'] })
    },
  })
}
