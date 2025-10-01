import { IContactRepository } from '../../../domain/repositories/IContactRepository';
import { Contact, ContactSearchCriteria, ContactSearchResult, ContactId } from '../../../domain';

export class GetContactsUseCase {
  constructor(private contactRepository: IContactRepository) {}

  async execute(criteria: ContactSearchCriteria): Promise<ContactSearchResult> {
    return await this.contactRepository.findAll(criteria);
  }

  async executeById(id: ContactId): Promise<Contact> {
    const contact = await this.contactRepository.findById(id);
    if (!contact) {
      throw new Error('Contact not found');
    }
    return contact;
  }
}