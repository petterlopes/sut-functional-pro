use std::str::FromStr;
use crate::domain::entities::Contact;
use crate::domain::repositories::{ContactRepository, ContactSearchCriteria};
use crate::domain::value_objects::*;
use crate::domain::errors::DomainError;
use crate::application::dto::*;

pub struct CreateContactUseCase<'a> {
    contact_repository: &'a dyn ContactRepository,
}

impl<'a> CreateContactUseCase<'a> {
    pub fn new(contact_repository: &'a dyn ContactRepository) -> Self {
        CreateContactUseCase { contact_repository }
    }

    pub async fn execute(&self, request: CreateContactRequest) -> Result<ContactResponse, DomainError> {
        let contact_type = ContactType::from_str(&request.contact_type)
            .map_err(|e| DomainError::ValidationError(e))?;
        let status = ContactStatus::from_str(&request.status)
            .map_err(|e| DomainError::ValidationError(e))?;

        let unit_id = request.unit_id.map(OrgUnitId);
        let department_id = request.department_id.map(DepartmentId);

        let contact = Contact::new(
            request.full_name,
            contact_type,
            status,
            request.document,
            unit_id,
            department_id,
            request.emails,
            request.phones,
        )?;

        let saved_contact = self.contact_repository.save(&contact).await?;
        Ok(saved_contact.into())
    }
}

pub struct UpdateContactUseCase<'a> {
    contact_repository: &'a dyn ContactRepository,
}

impl<'a> UpdateContactUseCase<'a> {
    pub fn new(contact_repository: &'a dyn ContactRepository) -> Self {
        UpdateContactUseCase { contact_repository }
    }

    pub async fn execute(&self, request: UpdateContactRequest) -> Result<ContactResponse, DomainError> {
        let contact_id = ContactId::from_string(&request.id)
            .map_err(|e| DomainError::ValidationError(format!("Invalid contact ID: {}", e)))?;

        let mut contact = self.contact_repository.find_by_id(&contact_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Contact with ID {} not found", request.id)))?;

        // Check ETag for optimistic concurrency control
        if contact.etag != request.etag {
            return Err(DomainError::Conflict("ETag mismatch - contact was modified by another user".to_string()));
        }

        if let Some(full_name) = request.full_name {
            contact.update_full_name(full_name)?;
        }

        if let Some(contact_type_str) = request.contact_type {
            let contact_type = ContactType::from_str(&contact_type_str)
                .map_err(|e| DomainError::ValidationError(e))?;
            contact.update_contact_type(contact_type);
        }

        if let Some(status_str) = request.status {
            let status = ContactStatus::from_str(&status_str)
                .map_err(|e| DomainError::ValidationError(e))?;
            contact.update_status(status);
        }

        if let Some(document) = request.document {
            contact.update_document(Some(document));
        }

        if let Some(unit_id) = request.unit_id {
            contact.update_unit_id(Some(OrgUnitId(unit_id)));
        }

        if let Some(department_id) = request.department_id {
            contact.update_department_id(Some(DepartmentId(department_id)));
        }

        if let Some(emails) = request.emails {
            contact.emails = emails;
        }

        if let Some(phones) = request.phones {
            contact.phones = phones;
        }

        let updated_contact = self.contact_repository.update(&contact).await?;
        Ok(updated_contact.into())
    }
}

pub struct DeleteContactUseCase<'a> {
    contact_repository: &'a dyn ContactRepository,
}

impl<'a> DeleteContactUseCase<'a> {
    pub fn new(contact_repository: &'a dyn ContactRepository) -> Self {
        DeleteContactUseCase { contact_repository }
    }

    pub async fn execute(&self, id: &str) -> Result<(), DomainError> {
        let contact_id = ContactId::from_string(id)
            .map_err(|e| DomainError::ValidationError(format!("Invalid contact ID: {}", e)))?;

        // Check if contact exists
        self.contact_repository.find_by_id(&contact_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Contact with ID {} not found", id)))?;

        self.contact_repository.delete(&contact_id).await?;
        Ok(())
    }
}

pub struct GetContactsUseCase<'a> {
    contact_repository: &'a dyn ContactRepository,
}

impl<'a> GetContactsUseCase<'a> {
    pub fn new(contact_repository: &'a dyn ContactRepository) -> Self {
        GetContactsUseCase { contact_repository }
    }

    pub async fn execute(&self, request: ContactSearchRequest) -> Result<ContactSearchResponse, DomainError> {
        let contact_type = if let Some(ct) = request.contact_type {
            Some(ContactType::from_str(&ct)
                .map_err(|e| DomainError::ValidationError(e))?)
        } else {
            None
        };

        let status = if let Some(s) = request.status {
            Some(ContactStatus::from_str(&s)
                .map_err(|e| DomainError::ValidationError(e))?)
        } else {
            None
        };

        let unit_id = request.unit_id.map(OrgUnitId);
        let department_id = request.department_id.map(DepartmentId);

        let criteria = ContactSearchCriteria {
            full_name: request.search_term,
            contact_type,
            status,
            unit_id,
            department_id,
            limit: request.limit,
            offset: request.offset,
        };

        let result = self.contact_repository.find_all(&criteria).await?;
        let items = result.items.into_iter().map(|contact| contact.into()).collect();

        Ok(ContactSearchResponse {
            items,
            total: result.total,
        })
    }

    pub async fn execute_by_id(&self, id: &ContactId) -> Result<ContactResponse, DomainError> {
        let contact = self.contact_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Contact with ID {} not found", id)))?;
        Ok(contact.into())
    }
}

pub struct GetContactStatisticsUseCase<'a> {
    contact_repository: &'a dyn ContactRepository,
}

impl<'a> GetContactStatisticsUseCase<'a> {
    pub fn new(contact_repository: &'a dyn ContactRepository) -> Self {
        GetContactStatisticsUseCase { contact_repository }
    }

    pub async fn execute(&self) -> Result<ContactStatisticsResponse, DomainError> {
        let stats = self.contact_repository.get_statistics().await?;
        Ok(ContactStatisticsResponse {
            total_contacts: stats.total_contacts,
            active_contacts: stats.active_contacts,
            inactive_contacts: stats.inactive_contacts,
            persons: stats.persons,
            organizations: stats.organizations,
            departments: stats.departments,
        })
    }
}