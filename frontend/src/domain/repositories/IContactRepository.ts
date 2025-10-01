import { Contact, ContactSearchCriteria, ContactSearchResult, ContactStatistics } from '../entities/Contact';
import { ContactId } from '../value-objects';

export interface IContactRepository {
  findById(id: ContactId): Promise<Contact | null>;
  findAll(criteria: ContactSearchCriteria): Promise<ContactSearchResult>;
  save(contact: Contact): Promise<Contact>;
  update(contact: Contact): Promise<Contact>;
  delete(id: ContactId): Promise<void>;
  findByEmail(email: string): Promise<Contact | null>;
  findByDocument(document: string): Promise<Contact | null>;
  findByName(name: string): Promise<Contact[]>;
  findByUnit(unitId: string): Promise<Contact[]>;
  findByDepartment(departmentId: string): Promise<Contact[]>;
  getStatistics(): Promise<ContactStatistics>;
}