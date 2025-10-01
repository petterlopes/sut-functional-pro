import { IContactRepository } from '../../../domain/repositories/IContactRepository';
import { ContactStatistics } from '../../../domain';

export class GetContactStatisticsUseCase {
  constructor(private contactRepository: IContactRepository) {}

  async execute(): Promise<ContactStatistics> {
    return await this.contactRepository.getStatistics();
  }
}