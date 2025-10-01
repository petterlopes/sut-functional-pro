import { IContactRepository } from '../../domain/repositories/IContactRepository';
import { Contact, ContactSearchCriteria, ContactSearchResult, ContactStatistics, ContactId } from '../../domain';
import { IApiClient } from '../api/IApiClient';

export class ContactRepository implements IContactRepository {
  constructor(private apiClient: IApiClient) {}

  async findById(id: ContactId): Promise<Contact | null> {
    try {
      const response = await this.apiClient.get(`/v1/contacts-clean/${id}`);
      return this.mapToContact(response.data);
    } catch (error) {
      if (error.response?.status === 404) {
        return null;
      }
      throw error;
    }
  }

  async findAll(criteria: ContactSearchCriteria): Promise<ContactSearchResult> {
    const params = new URLSearchParams();
    if (criteria.searchTerm) params.append('search_term', criteria.searchTerm);
    if (criteria.contactType) params.append('contact_type', criteria.contactType);
    if (criteria.status) params.append('status', criteria.status);
    if (criteria.unitId) params.append('unit_id', criteria.unitId);
    if (criteria.departmentId) params.append('department_id', criteria.departmentId);
    if (criteria.limit) params.append('limit', criteria.limit.toString());
    if (criteria.offset) params.append('offset', criteria.offset.toString());

    const response = await this.apiClient.get(`/v1/contacts-clean?${params.toString()}`);
    return {
      items: response.data.items.map((item: any) => this.mapToContact(item)),
      total: response.data.total,
    };
  }

  async save(contact: Contact): Promise<Contact> {
    const response = await this.apiClient.post('/v1/contacts-clean', this.mapToCreateRequest(contact));
    return this.mapToContact(response.data);
  }

  async update(contact: Contact): Promise<Contact> {
    const response = await this.apiClient.patch(`/v1/contacts-clean/${contact.id}`, this.mapToUpdateRequest(contact));
    return this.mapToContact(response.data);
  }

  async delete(id: ContactId): Promise<void> {
    await this.apiClient.delete(`/v1/contacts-clean/${id}`);
  }

  async findByEmail(email: string): Promise<Contact | null> {
    const result = await this.findAll({ searchTerm: email, limit: 1 });
    return result.items.length > 0 ? result.items[0] : null;
  }

  async findByDocument(document: string): Promise<Contact | null> {
    const result = await this.findAll({ searchTerm: document, limit: 1 });
    return result.items.length > 0 ? result.items[0] : null;
  }

  async findByName(name: string): Promise<Contact[]> {
    const result = await this.findAll({ searchTerm: name });
    return result.items;
  }

  async findByUnit(unitId: string): Promise<Contact[]> {
    const result = await this.findAll({ unitId });
    return result.items;
  }

  async findByDepartment(departmentId: string): Promise<Contact[]> {
    const result = await this.findAll({ departmentId });
    return result.items;
  }

  async getStatistics(): Promise<ContactStatistics> {
    const response = await this.apiClient.get('/v1/contacts-clean/statistics');
    return response.data;
  }

  private mapToContact(data: any): Contact {
    return {
      id: data.id,
      fullName: data.full_name,
      contactType: data.contact_type,
      status: data.status,
      document: data.document,
      unitId: data.unit_id,
      departmentId: data.department_id,
      emails: data.emails || [],
      phones: data.phones || [],
      etag: data.etag,
      createdAt: new Date(data.created_at),
      updatedAt: new Date(data.updated_at),
    };
  }

  private mapToCreateRequest(contact: Contact): any {
    return {
      full_name: contact.fullName,
      contact_type: contact.contactType,
      status: contact.status,
      document: contact.document,
      unit_id: contact.unitId,
      department_id: contact.departmentId,
      emails: contact.emails,
      phones: contact.phones,
    };
  }

  private mapToUpdateRequest(contact: Contact): any {
    return {
      full_name: contact.fullName,
      contact_type: contact.contactType,
      status: contact.status,
      document: contact.document,
      unit_id: contact.unitId,
      department_id: contact.departmentId,
      emails: contact.emails,
      phones: contact.phones,
      etag: contact.etag,
    };
  }
}