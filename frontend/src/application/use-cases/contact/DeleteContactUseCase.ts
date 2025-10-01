import { IContactRepository } from '../../../domain/repositories/IContactRepository';
import { ContactId } from '../../../domain';

export class DeleteContactUseCase {
  constructor(private contactRepository: IContactRepository) {}

  async execute(id: ContactId): Promise<void> {
    // Check if contact exists
    const existingContact = await this.contactRepository.findById(id);
    if (!existingContact) {
      throw new Error('Contact not found');
    }

    await this.contactRepository.delete(id);
  }
}