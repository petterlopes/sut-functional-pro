import { ContactId, Email, Phone, ContactType, ContactStatus } from '../value-objects';

export interface Contact {
  id: ContactId;
  fullName: string;
  contactType: ContactType;
  status: ContactStatus;
  document?: string;
  unitId?: string;
  departmentId?: string;
  emails: Email[];
  phones: Phone[];
  etag: string;
  createdAt: Date;
  updatedAt: Date;
}

export interface CreateContactRequest {
  fullName: string;
  contactType: ContactType;
  status: ContactStatus;
  document?: string;
  unitId?: string;
  departmentId?: string;
  emails: Email[];
  phones: Phone[];
}

export interface UpdateContactRequest {
  id: string;
  fullName?: string;
  contactType?: ContactType;
  status?: ContactStatus;
  document?: string;
  unitId?: string;
  departmentId?: string;
  emails?: Email[];
  phones?: Phone[];
  etag: string;
}

export interface ContactSearchCriteria {
  searchTerm?: string;
  contactType?: ContactType;
  status?: ContactStatus;
  unitId?: string;
  departmentId?: string;
  limit?: number;
  offset?: number;
}

export interface ContactSearchResult {
  items: Contact[];
  total: number;
}

export interface ContactStatistics {
  totalContacts: number;
  activeContacts: number;
  inactiveContacts: number;
  persons: number;
  organizations: number;
  departments: number;
}