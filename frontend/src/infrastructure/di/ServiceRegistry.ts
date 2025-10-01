import { container } from './Container'
import { IApiClient } from '../api/IApiClient'
import { AxiosApiClient } from '../api/AxiosApiClient'
import { IContactRepository } from '../../domain/repositories/IContactRepository'
import { IOrgUnitRepository } from '../../domain/repositories/IOrgUnitRepository'
import { IDepartmentRepository } from '../../domain/repositories/IDepartmentRepository'
import { IUserRepository } from '../../domain/repositories/IUserRepository'
import { ContactRepository } from '../repositories/ContactRepository'
import { OrgUnitRepository } from '../repositories/OrgUnitRepository'
import { DepartmentRepository } from '../repositories/DepartmentRepository'
import { UserRepository } from '../repositories/UserRepository'
import { CreateContactUseCase } from '../../application/use-cases/contact/CreateContactUseCase'
import { UpdateContactUseCase } from '../../application/use-cases/contact/UpdateContactUseCase'
import { DeleteContactUseCase } from '../../application/use-cases/contact/DeleteContactUseCase'
import { GetContactsUseCase } from '../../application/use-cases/contact/GetContactsUseCase'
import { GetContactStatisticsUseCase } from '../../application/use-cases/contact/GetContactStatisticsUseCase'
import { CreateOrgUnitUseCase } from '../../application/use-cases/org-unit/CreateOrgUnitUseCase'
import { UpdateOrgUnitUseCase } from '../../application/use-cases/org-unit/UpdateOrgUnitUseCase'
import { DeleteOrgUnitUseCase } from '../../application/use-cases/org-unit/DeleteOrgUnitUseCase'
import { GetOrgUnitsUseCase } from '../../application/use-cases/org-unit/GetOrgUnitsUseCase'
import { CreateDepartmentUseCase } from '../../application/use-cases/department/CreateDepartmentUseCase'
import { UpdateDepartmentUseCase } from '../../application/use-cases/department/UpdateDepartmentUseCase'
import { DeleteDepartmentUseCase } from '../../application/use-cases/department/DeleteDepartmentUseCase'
import { GetDepartmentsUseCase, GetDepartmentStatisticsUseCase } from '../../application/use-cases/department/GetDepartmentsUseCase'
import { CreateUserUseCase } from '../../application/use-cases/user/CreateUserUseCase'
import { UpdateUserUseCase } from '../../application/use-cases/user/UpdateUserUseCase'
import { DeleteUserUseCase } from '../../application/use-cases/user/DeleteUserUseCase'
import { GetUsersUseCase } from '../../application/use-cases/user/GetUsersUseCase'

// Service names
export const SERVICE_NAMES = {
  API_CLIENT: 'apiClient',
  CONTACT_REPOSITORY: 'contactRepository',
  ORG_UNIT_REPOSITORY: 'orgUnitRepository',
  DEPARTMENT_REPOSITORY: 'departmentRepository',
  USER_REPOSITORY: 'userRepository',
  CREATE_CONTACT_USE_CASE: 'createContactUseCase',
  UPDATE_CONTACT_USE_CASE: 'updateContactUseCase',
  DELETE_CONTACT_USE_CASE: 'deleteContactUseCase',
  GET_CONTACTS_USE_CASE: 'getContactsUseCase',
  GET_CONTACT_STATISTICS_USE_CASE: 'getContactStatisticsUseCase',
  CREATE_ORG_UNIT_USE_CASE: 'createOrgUnitUseCase',
  UPDATE_ORG_UNIT_USE_CASE: 'updateOrgUnitUseCase',
  DELETE_ORG_UNIT_USE_CASE: 'deleteOrgUnitUseCase',
  GET_ORG_UNITS_USE_CASE: 'getOrgUnitsUseCase',
  CREATE_DEPARTMENT_USE_CASE: 'createDepartmentUseCase',
  UPDATE_DEPARTMENT_USE_CASE: 'updateDepartmentUseCase',
  DELETE_DEPARTMENT_USE_CASE: 'deleteDepartmentUseCase',
  GET_DEPARTMENTS_USE_CASE: 'getDepartmentsUseCase',
  GET_DEPARTMENT_STATISTICS_USE_CASE: 'getDepartmentStatisticsUseCase',
  CREATE_USER_USE_CASE: 'createUserUseCase',
  UPDATE_USER_USE_CASE: 'updateUserUseCase',
  DELETE_USER_USE_CASE: 'deleteUserUseCase',
  GET_USERS_USE_CASE: 'getUsersUseCase'
} as const

export class ServiceRegistry {
  static registerServices(baseURL: string, token?: string): void {
    // Infrastructure services
    container.registerInstance<IApiClient>(
      SERVICE_NAMES.API_CLIENT,
      new AxiosApiClient(baseURL, token)
    )

    // Repositories
    container.registerClass<IContactRepository>(
      SERVICE_NAMES.CONTACT_REPOSITORY,
      ContactRepository,
      { dependencies: [SERVICE_NAMES.API_CLIENT] }
    )

    container.registerClass<IOrgUnitRepository>(
      SERVICE_NAMES.ORG_UNIT_REPOSITORY,
      OrgUnitRepository,
      { dependencies: [SERVICE_NAMES.API_CLIENT] }
    )

    container.registerClass<IDepartmentRepository>(
      SERVICE_NAMES.DEPARTMENT_REPOSITORY,
      DepartmentRepository,
      { dependencies: [SERVICE_NAMES.API_CLIENT] }
    )

    container.registerClass<IUserRepository>(
      SERVICE_NAMES.USER_REPOSITORY,
      UserRepository,
      { dependencies: [SERVICE_NAMES.API_CLIENT] }
    )

    // Contact Use cases
    container.registerClass<CreateContactUseCase>(
      SERVICE_NAMES.CREATE_CONTACT_USE_CASE,
      CreateContactUseCase,
      { dependencies: [SERVICE_NAMES.CONTACT_REPOSITORY] }
    )

    container.registerClass<UpdateContactUseCase>(
      SERVICE_NAMES.UPDATE_CONTACT_USE_CASE,
      UpdateContactUseCase,
      { dependencies: [SERVICE_NAMES.CONTACT_REPOSITORY] }
    )

    container.registerClass<DeleteContactUseCase>(
      SERVICE_NAMES.DELETE_CONTACT_USE_CASE,
      DeleteContactUseCase,
      { dependencies: [SERVICE_NAMES.CONTACT_REPOSITORY] }
    )

    container.registerClass<GetContactsUseCase>(
      SERVICE_NAMES.GET_CONTACTS_USE_CASE,
      GetContactsUseCase,
      { dependencies: [SERVICE_NAMES.CONTACT_REPOSITORY] }
    )

    container.registerClass<GetContactStatisticsUseCase>(
      SERVICE_NAMES.GET_CONTACT_STATISTICS_USE_CASE,
      GetContactStatisticsUseCase,
      { dependencies: [SERVICE_NAMES.CONTACT_REPOSITORY] }
    )

    // OrgUnit Use cases
    container.registerClass<CreateOrgUnitUseCase>(
      SERVICE_NAMES.CREATE_ORG_UNIT_USE_CASE,
      CreateOrgUnitUseCase,
      { dependencies: [SERVICE_NAMES.ORG_UNIT_REPOSITORY] }
    )

    container.registerClass<UpdateOrgUnitUseCase>(
      SERVICE_NAMES.UPDATE_ORG_UNIT_USE_CASE,
      UpdateOrgUnitUseCase,
      { dependencies: [SERVICE_NAMES.ORG_UNIT_REPOSITORY] }
    )

    container.registerClass<DeleteOrgUnitUseCase>(
      SERVICE_NAMES.DELETE_ORG_UNIT_USE_CASE,
      DeleteOrgUnitUseCase,
      { dependencies: [SERVICE_NAMES.ORG_UNIT_REPOSITORY] }
    )

    container.registerClass<GetOrgUnitsUseCase>(
      SERVICE_NAMES.GET_ORG_UNITS_USE_CASE,
      GetOrgUnitsUseCase,
      { dependencies: [SERVICE_NAMES.ORG_UNIT_REPOSITORY] }
    )

    // Department Use cases
    container.registerClass<CreateDepartmentUseCase>(
      SERVICE_NAMES.CREATE_DEPARTMENT_USE_CASE,
      CreateDepartmentUseCase,
      { dependencies: [SERVICE_NAMES.DEPARTMENT_REPOSITORY] }
    )

    container.registerClass<UpdateDepartmentUseCase>(
      SERVICE_NAMES.UPDATE_DEPARTMENT_USE_CASE,
      UpdateDepartmentUseCase,
      { dependencies: [SERVICE_NAMES.DEPARTMENT_REPOSITORY] }
    )

    container.registerClass<DeleteDepartmentUseCase>(
      SERVICE_NAMES.DELETE_DEPARTMENT_USE_CASE,
      DeleteDepartmentUseCase,
      { dependencies: [SERVICE_NAMES.DEPARTMENT_REPOSITORY] }
    )

    container.registerClass<GetDepartmentsUseCase>(
      SERVICE_NAMES.GET_DEPARTMENTS_USE_CASE,
      GetDepartmentsUseCase,
      { dependencies: [SERVICE_NAMES.DEPARTMENT_REPOSITORY] }
    )

    container.registerClass<GetDepartmentStatisticsUseCase>(
      SERVICE_NAMES.GET_DEPARTMENT_STATISTICS_USE_CASE,
      GetDepartmentStatisticsUseCase,
      { dependencies: [SERVICE_NAMES.DEPARTMENT_REPOSITORY] }
    )

    // User Use cases
    container.registerClass<CreateUserUseCase>(
      SERVICE_NAMES.CREATE_USER_USE_CASE,
      CreateUserUseCase,
      { dependencies: [SERVICE_NAMES.USER_REPOSITORY] }
    )

    container.registerClass<UpdateUserUseCase>(
      SERVICE_NAMES.UPDATE_USER_USE_CASE,
      UpdateUserUseCase,
      { dependencies: [SERVICE_NAMES.USER_REPOSITORY] }
    )

    container.registerClass<DeleteUserUseCase>(
      SERVICE_NAMES.DELETE_USER_USE_CASE,
      DeleteUserUseCase,
      { dependencies: [SERVICE_NAMES.USER_REPOSITORY] }
    )

    container.registerClass<GetUsersUseCase>(
      SERVICE_NAMES.GET_USERS_USE_CASE,
      GetUsersUseCase,
      { dependencies: [SERVICE_NAMES.USER_REPOSITORY] }
    )
  }

  static updateToken(token: string): void {
    if (!container.isRegistered(SERVICE_NAMES.API_CLIENT)) {
      console.warn('API Client not registered. Call registerServices() first.')
      return
    }
    const apiClient = container.resolve<IApiClient>(SERVICE_NAMES.API_CLIENT)
    if (apiClient instanceof AxiosApiClient) {
      apiClient.updateToken(token)
    }
  }

  static removeToken(): void {
    if (!container.isRegistered(SERVICE_NAMES.API_CLIENT)) {
      console.warn('API Client not registered. Call registerServices() first.')
      return
    }
    const apiClient = container.resolve<IApiClient>(SERVICE_NAMES.API_CLIENT)
    if (apiClient instanceof AxiosApiClient) {
      apiClient.removeToken()
    }
  }

  static clear(): void {
    container.clear()
  }
}
