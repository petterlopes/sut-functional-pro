import { IContactRepository } from '../../../domain/repositories/IContactRepository';
import { Contact, UpdateContactRequest, ContactId } from '../../../domain';
import { ValueObjectFactory } from '../../../domain/value-objects';

export class UpdateContactUseCase {
  constructor(private contactRepository: IContactRepository) {}

  async execute(request: UpdateContactRequest): Promise<Contact> {
    // Find existing contact
    const existingContact = await this.contactRepository.findById(request.id);
    if (!existingContact) {
      throw new Error('Contact not found');
    }

    // Check ETag for optimistic concurrency control
    if (existingContact.etag !== request.etag) {
      throw new Error('Contact was modified by another user');
    }

    // Update fields if provided
    const updatedContact: Contact = {
      ...existingContact,
      fullName: request.fullName ?? existingContact.fullName,
      contactType: request.contactType ?? existingContact.contactType,
      status: request.status ?? existingContact.status,
      document: request.document ?? existingContact.document,
      unitId: request.unitId ?? existingContact.unitId,
      departmentId: request.departmentId ?? existingContact.departmentId,
      emails: request.emails?.map(email => 
        ValueObjectFactory.createEmail(email.value, email.isPrimary)
      ) ?? existingContact.emails,
      phones: request.phones?.map(phone => 
        ValueObjectFactory.createPhone(phone.e164, phone.phoneType, phone.isPrimary, phone.extension)
      ) ?? existingContact.phones,
      etag: crypto.randomUUID(),
      updatedAt: new Date(),
    };

    return await this.contactRepository.update(updatedContact);
  }
}