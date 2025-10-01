import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { container } from '../../infrastructure/di/Container'
import { SERVICE_NAMES } from '../../infrastructure/di/ServiceRegistry'
import { CreateContactUseCase, CreateContactRequest } from '../../application/use-cases/contact/CreateContactUseCase'
import { UpdateContactUseCase, UpdateContactRequest } from '../../application/use-cases/contact/UpdateContactUseCase'
import { DeleteContactUseCase, DeleteContactRequest } from '../../application/use-cases/contact/DeleteContactUseCase'
import { GetContactsUseCase, GetContactsRequest } from '../../application/use-cases/contact/GetContactsUseCase'
import { GetContactStatisticsUseCase } from '../../application/use-cases/contact/GetContactStatisticsUseCase'
import { Contact, ContactSearchCriteria, ContactStatistics } from '../../domain'

// Custom hooks that use Clean Architecture with new API endpoints

export function useContactsClean(request?: ContactSearchCriteria) {
  return useQuery({
    queryKey: ['contacts-clean', request],
    queryFn: async () => {
      const useCase = container.resolve<GetContactsUseCase>(SERVICE_NAMES.GET_CONTACTS_USE_CASE)
      return await useCase.execute(request || {})
    },
  })
}

export function useContactClean(id: string) {
  return useQuery({
    queryKey: ['contact-clean', id],
    queryFn: async () => {
      const useCase = container.resolve<GetContactsUseCase>(SERVICE_NAMES.GET_CONTACTS_USE_CASE)
      return await useCase.executeById(id)
    },
    enabled: !!id,
  })
}

export function useCreateContactClean() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: async (request: CreateContactRequest) => {
      const useCase = container.resolve<CreateContactUseCase>(SERVICE_NAMES.CREATE_CONTACT_USE_CASE)
      return await useCase.execute(request)
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['contacts-clean'] })
      queryClient.invalidateQueries({ queryKey: ['contact-statistics-clean'] })
    },
  })
}

export function useUpdateContactClean() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: async (request: UpdateContactRequest) => {
      const useCase = container.resolve<UpdateContactUseCase>(SERVICE_NAMES.UPDATE_CONTACT_USE_CASE)
      return await useCase.execute(request)
    },
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['contacts-clean'] })
      queryClient.invalidateQueries({ queryKey: ['contact-clean', variables.id] })
      queryClient.invalidateQueries({ queryKey: ['contact-statistics-clean'] })
    },
  })
}

export function useDeleteContactClean() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: async (request: { id: string }) => {
      const useCase = container.resolve<DeleteContactUseCase>(SERVICE_NAMES.DELETE_CONTACT_USE_CASE)
      return await useCase.execute(request.id)
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['contacts-clean'] })
      queryClient.invalidateQueries({ queryKey: ['contact-statistics-clean'] })
    },
  })
}

export function useContactStatisticsClean() {
  return useQuery({
    queryKey: ['contact-statistics-clean'],
    queryFn: async () => {
      const useCase = container.resolve<GetContactStatisticsUseCase>(SERVICE_NAMES.GET_CONTACT_STATISTICS_USE_CASE)
      return await useCase.execute()
    },
  })
}