import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { container } from '../../infrastructure/di/Container'
import { SERVICE_NAMES } from '../../infrastructure/di/ServiceRegistry'
import { CreateDepartmentUseCase, CreateDepartmentRequest } from '../../application/use-cases/department/CreateDepartmentUseCase'
import { UpdateDepartmentUseCase, UpdateDepartmentRequest } from '../../application/use-cases/department/UpdateDepartmentUseCase'
import { DeleteDepartmentUseCase } from '../../application/use-cases/department/DeleteDepartmentUseCase'
import { GetDepartmentsUseCase, GetDepartmentStatisticsUseCase } from '../../application/use-cases/department/GetDepartmentsUseCase'
import { Department, DepartmentSearchCriteria, DepartmentStatistics } from '../../domain'

export function useDepartments(request?: DepartmentSearchCriteria) {
  return useQuery({
    queryKey: ['departments', request],
    queryFn: async () => {
      const useCase = container.resolve<GetDepartmentsUseCase>(SERVICE_NAMES.GET_DEPARTMENTS_USE_CASE)
      return await useCase.execute(request || {})
    },
  })
}

export function useDepartment(id: string) {
  return useQuery({
    queryKey: ['department', id],
    queryFn: async () => {
      const useCase = container.resolve<GetDepartmentsUseCase>(SERVICE_NAMES.GET_DEPARTMENTS_USE_CASE)
      return await useCase.executeById(id)
    },
    enabled: !!id,
  })
}

export function useDepartmentsByUnit(unitId: string) {
  return useQuery({
    queryKey: ['departments-by-unit', unitId],
    queryFn: async () => {
      const useCase = container.resolve<GetDepartmentsUseCase>(SERVICE_NAMES.GET_DEPARTMENTS_USE_CASE)
      return await useCase.executeByUnit(unitId)
    },
    enabled: !!unitId,
  })
}

export function useDepartmentStatistics() {
  return useQuery({
    queryKey: ['department-statistics'],
    queryFn: async () => {
      const useCase = container.resolve<GetDepartmentStatisticsUseCase>(SERVICE_NAMES.GET_DEPARTMENT_STATISTICS_USE_CASE)
      return await useCase.execute()
    },
  })
}

export function useCreateDepartment() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: async (request: CreateDepartmentRequest) => {
      const useCase = container.resolve<CreateDepartmentUseCase>(SERVICE_NAMES.CREATE_DEPARTMENT_USE_CASE)
      return await useCase.execute(request)
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['departments'] })
      queryClient.invalidateQueries({ queryKey: ['department-statistics'] })
    },
  })
}

export function useUpdateDepartment() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: async (request: UpdateDepartmentRequest) => {
      const useCase = container.resolve<UpdateDepartmentUseCase>(SERVICE_NAMES.UPDATE_DEPARTMENT_USE_CASE)
      return await useCase.execute(request)
    },
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['departments'] })
      queryClient.invalidateQueries({ queryKey: ['department', variables.id] })
      queryClient.invalidateQueries({ queryKey: ['department-statistics'] })
    },
  })
}

export function useDeleteDepartment() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: async (id: string) => {
      const useCase = container.resolve<DeleteDepartmentUseCase>(SERVICE_NAMES.DELETE_DEPARTMENT_USE_CASE)
      return await useCase.execute(id)
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['departments'] })
      queryClient.invalidateQueries({ queryKey: ['department-statistics'] })
    },
  })
}
