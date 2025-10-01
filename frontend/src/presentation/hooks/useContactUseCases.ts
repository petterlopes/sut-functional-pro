import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { container } from '../../infrastructure/di/Container'
import { SERVICE_NAMES } from '../../infrastructure/di/ServiceRegistry'
import { CreateContactUseCase, CreateContactRequest } from '../../application/use-cases/contact/CreateContactUseCase'
import { UpdateContactUseCase, UpdateContactRequest } from '../../application/use-cases/contact/UpdateContactUseCase'
import { DeleteContactUseCase, DeleteContactRequest } from '../../application/use-cases/contact/DeleteContactUseCase'
import { GetContactsUseCase, GetContactsRequest } from '../../application/use-cases/contact/GetContactsUseCase'
import { GetContactStatisticsUseCase } from '../../application/use-cases/contact/GetContactStatisticsUseCase'

// Custom hooks that use Clean Architecture

export function useContacts(request?: GetContactsRequest) {
  return useQuery({
    queryKey: ['contacts', request],
    queryFn: async () => {
      const useCase = container.resolve<GetContactsUseCase>(SERVICE_NAMES.GET_CONTACTS_USE_CASE)
      return await useCase.execute(request || {})
    },
    staleTime: 5 * 60 * 1000, // 5 minutes
  })
}

export function useContactStatistics() {
  return useQuery({
    queryKey: ['contact-statistics'],
    queryFn: async () => {
      const useCase = container.resolve<GetContactStatisticsUseCase>(SERVICE_NAMES.GET_CONTACT_STATISTICS_USE_CASE)
      return await useCase.execute()
    },
    staleTime: 10 * 60 * 1000, // 10 minutes
  })
}

export function useCreateContact() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (request: CreateContactRequest) => {
      const useCase = container.resolve<CreateContactUseCase>(SERVICE_NAMES.CREATE_CONTACT_USE_CASE)
      return await useCase.execute(request)
    },
    onSuccess: () => {
      // Invalidate and refetch contacts
      queryClient.invalidateQueries({ queryKey: ['contacts'] })
      queryClient.invalidateQueries({ queryKey: ['contact-statistics'] })
    },
  })
}

export function useUpdateContact() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (request: UpdateContactRequest) => {
      const useCase = container.resolve<UpdateContactUseCase>(SERVICE_NAMES.UPDATE_CONTACT_USE_CASE)
      return await useCase.execute(request)
    },
    onSuccess: () => {
      // Invalidate and refetch contacts
      queryClient.invalidateQueries({ queryKey: ['contacts'] })
      queryClient.invalidateQueries({ queryKey: ['contact-statistics'] })
    },
  })
}

export function useDeleteContact() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (request: DeleteContactRequest) => {
      const useCase = container.resolve<DeleteContactUseCase>(SERVICE_NAMES.DELETE_CONTACT_USE_CASE)
      return await useCase.execute(request)
    },
    onSuccess: () => {
      // Invalidate and refetch contacts
      queryClient.invalidateQueries({ queryKey: ['contacts'] })
      queryClient.invalidateQueries({ queryKey: ['contact-statistics'] })
    },
  })
}
