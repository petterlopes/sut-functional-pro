import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { container } from '../../infrastructure/di/Container'
import { SERVICE_NAMES } from '../../infrastructure/di/ServiceRegistry'
import { CreateOrgUnitUseCase, CreateOrgUnitRequest } from '../../application/use-cases/org-unit/CreateOrgUnitUseCase'
import { UpdateOrgUnitUseCase, UpdateOrgUnitRequest } from '../../application/use-cases/org-unit/UpdateOrgUnitUseCase'
import { DeleteOrgUnitUseCase } from '../../application/use-cases/org-unit/DeleteOrgUnitUseCase'
import { GetOrgUnitsUseCase } from '../../application/use-cases/org-unit/GetOrgUnitsUseCase'
import { OrgUnit, OrgUnitSearchCriteria, OrgUnitHierarchy } from '../../domain'

export function useOrgUnits(request?: OrgUnitSearchCriteria) {
  return useQuery({
    queryKey: ['org-units', request],
    queryFn: async () => {
      const useCase = container.resolve<GetOrgUnitsUseCase>(SERVICE_NAMES.GET_ORG_UNITS_USE_CASE)
      return await useCase.execute(request || {})
    },
  })
}

export function useOrgUnit(id: string) {
  return useQuery({
    queryKey: ['org-unit', id],
    queryFn: async () => {
      const useCase = container.resolve<GetOrgUnitsUseCase>(SERVICE_NAMES.GET_ORG_UNITS_USE_CASE)
      return await useCase.executeById(id)
    },
    enabled: !!id,
  })
}

export function useOrgUnitHierarchy(id: string) {
  return useQuery({
    queryKey: ['org-unit-hierarchy', id],
    queryFn: async () => {
      const useCase = container.resolve<GetOrgUnitsUseCase>(SERVICE_NAMES.GET_ORG_UNITS_USE_CASE)
      return await useCase.executeHierarchy(id)
    },
    enabled: !!id,
  })
}

export function useCreateOrgUnit() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: async (request: CreateOrgUnitRequest) => {
      const useCase = container.resolve<CreateOrgUnitUseCase>(SERVICE_NAMES.CREATE_ORG_UNIT_USE_CASE)
      return await useCase.execute(request)
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['org-units'] })
    },
  })
}

export function useUpdateOrgUnit() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: async (request: UpdateOrgUnitRequest) => {
      const useCase = container.resolve<UpdateOrgUnitUseCase>(SERVICE_NAMES.UPDATE_ORG_UNIT_USE_CASE)
      return await useCase.execute(request)
    },
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['org-units'] })
      queryClient.invalidateQueries({ queryKey: ['org-unit', variables.id] })
      queryClient.invalidateQueries({ queryKey: ['org-unit-hierarchy', variables.id] })
    },
  })
}

export function useDeleteOrgUnit() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: async (id: string) => {
      const useCase = container.resolve<DeleteOrgUnitUseCase>(SERVICE_NAMES.DELETE_ORG_UNIT_USE_CASE)
      return await useCase.execute(id)
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['org-units'] })
    },
  })
}
