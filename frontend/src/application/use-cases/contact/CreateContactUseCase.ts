import { IContactRepository } from '../../../domain/repositories/IContactRepository';
import { Contact, CreateContactRequest, ContactId, ContactType, ContactStatus } from '../../../domain';
import { ValueObjectFactory } from '../../../domain/value-objects';

export class CreateContactUseCase {
  constructor(private contactRepository: IContactRepository) {}

  async execute(request: CreateContactRequest): Promise<Contact> {
    // Validate input
    if (!request.fullName?.trim()) {
      throw new Error('Full name is required');
    }

    // Create value objects
    const contactType = request.contactType || ContactType.PERSON;
    const status = request.status || ContactStatus.ACTIVE;

    // Create emails with validation
    const emails = request.emails?.map(email => 
      ValueObjectFactory.createEmail(email.value, email.isPrimary)
    ) || [];

    // Create phones with validation
    const phones = request.phones?.map(phone => 
      ValueObjectFactory.createPhone(phone.e164, phone.phoneType, phone.isPrimary, phone.extension)
    ) || [];

    // Create contact entity
    const contact: Contact = {
      id: crypto.randomUUID(),
      fullName: request.fullName,
      contactType,
      status,
      document: request.document,
      unitId: request.unitId,
      departmentId: request.departmentId,
      emails,
      phones,
      etag: crypto.randomUUID(),
      createdAt: new Date(),
      updatedAt: new Date(),
    };

    return await this.contactRepository.save(contact);
  }
}