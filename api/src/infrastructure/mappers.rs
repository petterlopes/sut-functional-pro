use crate::domain::entities::*;
use crate::domain::value_objects::*;
use crate::domain::errors::DomainError;
use sqlx::FromRow;
use uuid::Uuid;
use std::str::FromStr;

// Contact Database Models
#[derive(Debug, FromRow)]
pub struct ContactRow {
    pub id: Uuid,
    pub full_name: String,
    pub r#type: String, // `type` is a reserved keyword in Rust, so we use `r#type`
    pub status: String,
    pub document: Option<String>,
    pub unit_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub etag: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, FromRow)]
pub struct EmailRow {
    pub contact_id: Uuid,
    pub address: String,
    pub is_primary: bool,
}

#[derive(Debug, FromRow)]
pub struct PhoneRow {
    pub contact_id: Uuid,
    pub e164: String,
    pub extension: Option<String>,
    pub r#type: String, // `type` is a reserved keyword in Rust
    pub is_primary: bool,
}

// OrgUnit Database Models
#[derive(Debug, FromRow)]
pub struct OrgUnitRow {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Department Database Models
#[derive(Debug, FromRow)]
pub struct DepartmentRow {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// User Database Models
#[derive(Debug, FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// AuditEvent Database Models
#[derive(Debug, FromRow)]
pub struct AuditEventRow {
    pub id: i64,
    pub actor_sub: Option<String>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: String,
    pub before: Option<serde_json::Value>,
    pub after: Option<serde_json::Value>,
    pub at: chrono::DateTime<chrono::Utc>,
}

// SourceRecord Database Models
#[derive(Debug, FromRow)]
pub struct SourceRecordRow {
    pub id: Uuid,
    pub source: String,
    pub source_key: String,
    pub hash: String,
    pub payload: serde_json::Value,
    pub fetched_at: chrono::DateTime<chrono::Utc>,
}

// ContactSource Database Models
#[derive(Debug, FromRow)]
pub struct ContactSourceRow {
    pub contact_id: Uuid,
    pub source_record_id: Uuid,
    pub confidence: f64,
}

// MergeCandidate Database Models
#[derive(Debug, FromRow)]
pub struct MergeCandidateRow {
    pub contact_a: Uuid,
    pub contact_b: Uuid,
    pub score: f64,
    pub features: serde_json::Value,
}

// MergeDecision Database Models
#[derive(Debug, FromRow)]
pub struct MergeDecisionRow {
    pub primary_contact: Uuid,
    pub duplicate_contact: Uuid,
    pub decision: String,
    pub chosen_fields: Option<serde_json::Value>,
    pub decided_by: Option<Uuid>,
    pub decided_at: chrono::DateTime<chrono::Utc>,
}

// WebhookReceipt Database Models
#[derive(Debug, FromRow)]
pub struct WebhookReceiptRow {
    pub id: Uuid,
    pub source: String,
    pub nonce: String,
    pub received_at: chrono::DateTime<chrono::Utc>,
}

// Helper function to build a Contact entity from rows
pub fn build_contact_with_relations(
    contact_row: ContactRow,
    email_rows: Vec<EmailRow>,
    phone_rows: Vec<PhoneRow>,
) -> Result<Contact, DomainError> {
    let contact_type = ContactType::from_str(&contact_row.r#type)
        .map_err(|e| DomainError::InternalError(format!("Invalid contact type from DB: {}", e)))?;
    let status = ContactStatus::from_str(&contact_row.status)
        .map_err(|e| DomainError::InternalError(format!("Invalid contact status from DB: {}", e)))?;

    let emails = email_rows
        .into_iter()
        .map(|row| Email {
            value: row.address,
            is_primary: row.is_primary,
        })
        .collect();

    let phones = phone_rows
        .into_iter()
        .map(|row| {
            let phone_type = PhoneType::from_str(&row.r#type)
                .map_err(|e| DomainError::InternalError(format!("Invalid phone type from DB: {}", e)))?;
            Ok(Phone {
                e164: row.e164,
                extension: row.extension,
                phone_type,
                is_primary: row.is_primary,
            })
        })
        .collect::<Result<Vec<Phone>, DomainError>>()?;

    Ok(Contact {
        id: ContactId(contact_row.id),
        full_name: contact_row.full_name,
        contact_type,
        status,
        document: contact_row.document,
        unit_id: contact_row.unit_id.map(OrgUnitId),
        department_id: contact_row.department_id.map(DepartmentId),
        emails,
        phones,
        etag: contact_row.etag,
        created_at: contact_row.created_at,
        updated_at: contact_row.updated_at,
    })
}

// Helper function to build an OrgUnit entity from row
pub fn build_org_unit_from_row(row: OrgUnitRow) -> Result<OrgUnit, DomainError> {
    let name = OrgUnitName::new(row.name)
        .map_err(|e| DomainError::InternalError(format!("Invalid org unit name from DB: {}", e)))?;

    Ok(OrgUnit {
        id: OrgUnitId(row.id),
        name,
        parent_id: row.parent_id.map(OrgUnitId),
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

// Helper function to build a Department entity from row
pub fn build_department_from_row(row: DepartmentRow) -> Result<Department, DomainError> {
    let name = DepartmentName::new(row.name)
        .map_err(|e| DomainError::InternalError(format!("Invalid department name from DB: {}", e)))?;

    Ok(Department {
        id: DepartmentId(row.id),
        unit_id: OrgUnitId(row.unit_id),
        name,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

// Helper function to build a User entity from row
pub fn build_user_from_row(row: UserRow) -> Result<User, DomainError> {
    let username = Username::new(row.username)
        .map_err(|e| DomainError::InternalError(format!("Invalid username from DB: {}", e)))?;
    let email = UserEmail::new(row.email)
        .map_err(|e| DomainError::InternalError(format!("Invalid email from DB: {}", e)))?;
    let password = Password::new(row.password)
        .map_err(|e| DomainError::InternalError(format!("Invalid password from DB: {}", e)))?;

    let roles = row.roles
        .into_iter()
        .map(|role_str| Role::new(role_str))
        .collect::<Result<Vec<Role>, String>>()
        .map_err(|e| DomainError::InternalError(format!("Invalid role from DB: {}", e)))?;

    Ok(User {
        id: UserId(row.id),
        username,
        email,
        password,
        roles,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

// Helper function to build an AuditEvent entity from row
pub fn build_audit_event_from_row(row: AuditEventRow) -> Result<AuditEvent, DomainError> {
    let action = Action::new(row.action)
        .map_err(|e| DomainError::InternalError(format!("Invalid action from DB: {}", e)))?;
    let entity_type = EntityType::new(row.entity_type)
        .map_err(|e| DomainError::InternalError(format!("Invalid entity type from DB: {}", e)))?;

    Ok(AuditEvent {
        id: AuditEventId::new(row.id),
        actor_sub: row.actor_sub,
        action,
        entity_type,
        entity_id: row.entity_id,
        before: row.before,
        after: row.after,
        at: row.at,
    })
}

// Helper function to build a SourceRecord entity from row
pub fn build_source_record_from_row(row: SourceRecordRow) -> Result<SourceRecord, DomainError> {
    let source = Source::new(row.source)
        .map_err(|e| DomainError::InternalError(format!("Invalid source from DB: {}", e)))?;
    let source_key = SourceKey::new(row.source_key)
        .map_err(|e| DomainError::InternalError(format!("Invalid source key from DB: {}", e)))?;
    let hash = Hash::new(row.hash)
        .map_err(|e| DomainError::InternalError(format!("Invalid hash from DB: {}", e)))?;

    Ok(SourceRecord {
        id: SourceRecordId(row.id),
        source,
        source_key,
        hash,
        payload: row.payload,
        fetched_at: row.fetched_at,
    })
}

// Helper function to build a ContactSource entity from row
pub fn build_contact_source_from_row(row: ContactSourceRow) -> Result<ContactSource, DomainError> {
    Ok(ContactSource {
        contact_id: ContactId(row.contact_id),
        source_record_id: SourceRecordId(row.source_record_id),
        confidence: row.confidence,
    })
}

// Helper function to build a MergeCandidate entity from row
pub fn build_merge_candidate_from_row(row: MergeCandidateRow) -> Result<MergeCandidate, DomainError> {
    Ok(MergeCandidate {
        contact_a: ContactId(row.contact_a),
        contact_b: ContactId(row.contact_b),
        score: row.score,
        features: row.features,
    })
}

// Helper function to build a MergeDecision entity from row
pub fn build_merge_decision_from_row(row: MergeDecisionRow) -> Result<MergeDecision, DomainError> {
    let decision = MergeDecisionType::from_str(&row.decision)
        .map_err(|e| DomainError::InternalError(format!("Invalid merge decision type from DB: {}", e)))?;

    Ok(MergeDecision {
        primary_contact: ContactId(row.primary_contact),
        duplicate_contact: ContactId(row.duplicate_contact),
        decision,
        chosen_fields: row.chosen_fields,
        decided_by: row.decided_by.map(UserId),
        decided_at: row.decided_at,
    })
}

// Helper function to build a WebhookReceipt entity from row
pub fn build_webhook_receipt_from_row(row: WebhookReceiptRow) -> Result<WebhookReceipt, DomainError> {
    let source = Source::new(row.source)
        .map_err(|e| DomainError::InternalError(format!("Invalid source from DB: {}", e)))?;
    let nonce = Nonce::new(row.nonce)
        .map_err(|e| DomainError::InternalError(format!("Invalid nonce from DB: {}", e)))?;

    Ok(WebhookReceipt {
        id: WebhookReceiptId(row.id),
        source,
        nonce,
        received_at: row.received_at,
    })
}