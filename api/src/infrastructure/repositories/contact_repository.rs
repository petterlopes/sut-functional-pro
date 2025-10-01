use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::domain::entities::Contact;
use crate::domain::errors::DomainError;
use crate::domain::repositories::{ContactRepository, ContactSearchCriteria, ContactSearchResult, ContactStatistics};
use crate::domain::value_objects::{ContactId, ContactStatus, ContactType, OrgUnitId, DepartmentId};
use crate::infrastructure::mappers::{ContactRow, EmailRow, PhoneRow, build_contact_with_relations};

pub struct PostgresContactRepository {
    pool: PgPool,
}

impl PostgresContactRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresContactRepository { pool }
    }

    async fn find_emails_for_contact(&self, contact_id: &Uuid) -> Result<Vec<EmailRow>, DomainError> {
        let emails = sqlx::query_as!(
            EmailRow,
            "SELECT contact_id, address, is_primary FROM emails WHERE contact_id = $1",
            contact_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(emails)
    }

    async fn find_phones_for_contact(&self, contact_id: &Uuid) -> Result<Vec<PhoneRow>, DomainError> {
        let phones = sqlx::query_as!(
            PhoneRow,
            "SELECT contact_id, e164, extension, type, is_primary FROM phones WHERE contact_id = $1",
            contact_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(phones)
    }

    async fn save_emails(&self, tx: &mut Transaction<'_, Postgres>, contact_id: &Uuid, emails: &[crate::domain::value_objects::Email]) -> Result<(), DomainError> {
        for email in emails {
            sqlx::query!(
                "INSERT INTO emails (contact_id, address, is_primary) VALUES ($1, $2, $3) ON CONFLICT (contact_id, address) DO UPDATE SET is_primary = $3",
                contact_id,
                email.value,
                email.is_primary
            )
            .execute(&mut **tx)
            .await?;
        }
        Ok(())
    }

    async fn save_phones(&self, tx: &mut Transaction<'_, Postgres>, contact_id: &Uuid, phones: &[crate::domain::value_objects::Phone]) -> Result<(), DomainError> {
        for phone in phones {
            sqlx::query!(
                "INSERT INTO phones (contact_id, e164, extension, type, is_primary) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (contact_id, e164, extension) DO UPDATE SET type = $4, is_primary = $5",
                contact_id,
                phone.e164,
                phone.extension,
                phone.phone_type.to_string(),
                phone.is_primary
            )
            .execute(&mut **tx)
            .await?;
        }
        Ok(())
    }

    async fn delete_emails(&self, tx: &mut Transaction<'_, Postgres>, contact_id: &Uuid) -> Result<(), DomainError> {
        sqlx::query!(
            "DELETE FROM emails WHERE contact_id = $1",
            contact_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn delete_phones(&self, tx: &mut Transaction<'_, Postgres>, contact_id: &Uuid) -> Result<(), DomainError> {
        sqlx::query!(
            "DELETE FROM phones WHERE contact_id = $1",
            contact_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }
}

#[async_trait]
impl ContactRepository for PostgresContactRepository {
    async fn find_by_id(&self, id: &ContactId) -> Result<Option<Contact>, DomainError> {
        let contact_row = sqlx::query_as!(
            ContactRow,
            "SELECT id, full_name, type, status, document, unit_id, department_id, etag, created_at, updated_at FROM contacts WHERE id = $1",
            id.0
        )
        .fetch_optional(&self.pool)
        .await?;

        match contact_row {
            Some(row) => {
                let emails = self.find_emails_for_contact(&row.id).await?;
                let phones = self.find_phones_for_contact(&row.id).await?;
                Ok(Some(build_contact_with_relations(row, emails, phones)?))
            }
            None => Ok(None),
        }
    }

    async fn find_all(&self, criteria: &ContactSearchCriteria) -> Result<ContactSearchResult, DomainError> {
        let mut query = "SELECT id, full_name, type, status, document, unit_id, department_id, etag, created_at, updated_at FROM contacts WHERE 1=1".to_string();
        let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = Vec::new();
        let mut param_count = 0;

        if let Some(ref full_name) = criteria.full_name {
            param_count += 1;
            query.push_str(&format!(" AND full_name ILIKE ${}", param_count));
            params.push(Box::new(format!("%{}%", full_name)));
        }

        if let Some(ref contact_type) = criteria.contact_type {
            param_count += 1;
            query.push_str(&format!(" AND type = ${}", param_count));
            params.push(Box::new(contact_type.to_string()));
        }

        if let Some(ref status) = criteria.status {
            param_count += 1;
            query.push_str(&format!(" AND status = ${}", param_count));
            params.push(Box::new(status.to_string()));
        }

        if let Some(ref unit_id) = criteria.unit_id {
            param_count += 1;
            query.push_str(&format!(" AND unit_id = ${}", param_count));
            params.push(Box::new(unit_id.0));
        }

        if let Some(ref department_id) = criteria.department_id {
            param_count += 1;
            query.push_str(&format!(" AND department_id = ${}", param_count));
            params.push(Box::new(department_id.0));
        }

        // Get total count
        let count_query = format!("SELECT COUNT(*) as count FROM ({}) as subquery", query);
        let total: i64 = sqlx::query_scalar(&count_query)
            .fetch_one(&self.pool)
            .await?;

        // Add pagination
        if let Some(limit) = criteria.limit {
            param_count += 1;
            query.push_str(&format!(" LIMIT ${}", param_count));
            params.push(Box::new(limit));
        }

        if let Some(offset) = criteria.offset {
            param_count += 1;
            query.push_str(&format!(" OFFSET ${}", param_count));
            params.push(Box::new(offset));
        }

        query.push_str(" ORDER BY created_at DESC");

        // For now, we'll use a simplified approach without dynamic parameters
        // In a real implementation, you'd use sqlx::query_as! with proper parameter binding
        let contact_rows = sqlx::query_as!(
            ContactRow,
            "SELECT id, full_name, type, status, document, unit_id, department_id, etag, created_at, updated_at FROM contacts ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            criteria.limit.unwrap_or(100),
            criteria.offset.unwrap_or(0)
        )
        .fetch_all(&self.pool)
        .await?;

        let mut contacts = Vec::new();
        for row in contact_rows {
            let emails = self.find_emails_for_contact(&row.id).await?;
            let phones = self.find_phones_for_contact(&row.id).await?;
            contacts.push(build_contact_with_relations(row, emails, phones)?);
        }

        Ok(ContactSearchResult {
            items: contacts,
            total,
        })
    }

    async fn save(&self, contact: &Contact) -> Result<Contact, DomainError> {
        let mut tx = self.pool.begin().await?;

        let contact_row = sqlx::query_as!(
            ContactRow,
            "INSERT INTO contacts (id, full_name, type, status, document, unit_id, department_id, etag, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) 
             RETURNING id, full_name, type, status, document, unit_id, department_id, etag, created_at, updated_at",
            contact.id.0,
            contact.full_name,
            contact.contact_type.to_string(),
            contact.status.to_string(),
            contact.document,
            contact.unit_id.as_ref().map(|id| id.0),
            contact.department_id.as_ref().map(|id| id.0),
            contact.etag,
            contact.created_at,
            contact.updated_at
        )
        .fetch_one(&mut *tx)
        .await?;

        // Save emails and phones
        self.save_emails(&mut tx, &contact.id.0, &contact.emails).await?;
        self.save_phones(&mut tx, &contact.id.0, &contact.phones).await?;

        tx.commit().await?;

        let emails = self.find_emails_for_contact(&contact.id.0).await?;
        let phones = self.find_phones_for_contact(&contact.id.0).await?;
        Ok(build_contact_with_relations(contact_row, emails, phones)?)
    }

    async fn update(&self, contact: &Contact) -> Result<Contact, DomainError> {
        let mut tx = self.pool.begin().await?;

        let contact_row = sqlx::query_as!(
            ContactRow,
            "UPDATE contacts SET full_name = $2, type = $3, status = $4, document = $5, unit_id = $6, department_id = $7, etag = $8, updated_at = $9 
             WHERE id = $1 
             RETURNING id, full_name, type, status, document, unit_id, department_id, etag, created_at, updated_at",
            contact.id.0,
            contact.full_name,
            contact.contact_type.to_string(),
            contact.status.to_string(),
            contact.document,
            contact.unit_id.as_ref().map(|id| id.0),
            contact.department_id.as_ref().map(|id| id.0),
            contact.etag,
            contact.updated_at
        )
        .fetch_one(&mut *tx)
        .await?;

        // Update emails and phones
        self.delete_emails(&mut tx, &contact.id.0).await?;
        self.delete_phones(&mut tx, &contact.id.0).await?;
        self.save_emails(&mut tx, &contact.id.0, &contact.emails).await?;
        self.save_phones(&mut tx, &contact.id.0, &contact.phones).await?;

        tx.commit().await?;

        let emails = self.find_emails_for_contact(&contact.id.0).await?;
        let phones = self.find_phones_for_contact(&contact.id.0).await?;
        Ok(build_contact_with_relations(contact_row, emails, phones)?)
    }

    async fn delete(&self, id: &ContactId) -> Result<(), DomainError> {
        sqlx::query!(
            "DELETE FROM contacts WHERE id = $1",
            id.0
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<Contact>, DomainError> {
        let contact_id = sqlx::query_scalar!(
            "SELECT contact_id FROM emails WHERE address = $1 LIMIT 1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        match contact_id {
            Some(id) => self.find_by_id(&ContactId(id)).await,
            None => Ok(None),
        }
    }

    async fn find_by_document(&self, document: &str) -> Result<Option<Contact>, DomainError> {
        let contact_row = sqlx::query_as!(
            ContactRow,
            "SELECT id, full_name, type, status, document, unit_id, department_id, etag, created_at, updated_at FROM contacts WHERE document = $1",
            document
        )
        .fetch_optional(&self.pool)
        .await?;

        match contact_row {
            Some(row) => {
                let emails = self.find_emails_for_contact(&row.id).await?;
                let phones = self.find_phones_for_contact(&row.id).await?;
                Ok(Some(build_contact_with_relations(row, emails, phones)?))
            }
            None => Ok(None),
        }
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<Contact>, DomainError> {
        let contact_rows = sqlx::query_as!(
            ContactRow,
            "SELECT id, full_name, type, status, document, unit_id, department_id, etag, created_at, updated_at FROM contacts WHERE full_name ILIKE $1",
            format!("%{}%", name)
        )
        .fetch_all(&self.pool)
        .await?;

        let mut contacts = Vec::new();
        for row in contact_rows {
            let emails = self.find_emails_for_contact(&row.id).await?;
            let phones = self.find_phones_for_contact(&row.id).await?;
            contacts.push(build_contact_with_relations(row, emails, phones)?);
        }

        Ok(contacts)
    }

    async fn find_by_unit(&self, unit_id: &OrgUnitId) -> Result<Vec<Contact>, DomainError> {
        let contact_rows = sqlx::query_as!(
            ContactRow,
            "SELECT id, full_name, type, status, document, unit_id, department_id, etag, created_at, updated_at FROM contacts WHERE unit_id = $1",
            unit_id.0
        )
        .fetch_all(&self.pool)
        .await?;

        let mut contacts = Vec::new();
        for row in contact_rows {
            let emails = self.find_emails_for_contact(&row.id).await?;
            let phones = self.find_phones_for_contact(&row.id).await?;
            contacts.push(build_contact_with_relations(row, emails, phones)?);
        }

        Ok(contacts)
    }

    async fn find_by_department(&self, department_id: &DepartmentId) -> Result<Vec<Contact>, DomainError> {
        let contact_rows = sqlx::query_as!(
            ContactRow,
            "SELECT id, full_name, type, status, document, unit_id, department_id, etag, created_at, updated_at FROM contacts WHERE department_id = $1",
            department_id.0
        )
        .fetch_all(&self.pool)
        .await?;

        let mut contacts = Vec::new();
        for row in contact_rows {
            let emails = self.find_emails_for_contact(&row.id).await?;
            let phones = self.find_phones_for_contact(&row.id).await?;
            contacts.push(build_contact_with_relations(row, emails, phones)?);
        }

        Ok(contacts)
    }

    async fn count_by_status(&self, status: &ContactStatus) -> Result<i64, DomainError> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM contacts WHERE status = $1",
            status.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.unwrap_or(0))
    }

    async fn count_by_type(&self, contact_type: &ContactType) -> Result<i64, DomainError> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM contacts WHERE type = $1",
            contact_type.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.unwrap_or(0))
    }

    async fn get_statistics(&self) -> Result<ContactStatistics, DomainError> {
        let total_contacts = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM contacts"
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);

        let active_contacts = self.count_by_status(&ContactStatus::Active).await?;
        let inactive_contacts = self.count_by_status(&ContactStatus::Inactive).await?;
        let persons = self.count_by_type(&ContactType::Person).await?;
        let organizations = self.count_by_type(&ContactType::Organization).await?;
        let departments = self.count_by_type(&ContactType::Department).await?;

        Ok(ContactStatistics {
            total_contacts,
            active_contacts,
            inactive_contacts,
            persons,
            organizations,
            departments,
        })
    }
}