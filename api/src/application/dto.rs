use crate::domain::entities::*;
use crate::domain::value_objects::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Contact DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContactRequest {
    pub full_name: String,
    pub contact_type: String,
    pub status: String,
    pub document: Option<String>,
    pub unit_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub emails: Vec<Email>,
    pub phones: Vec<Phone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContactRequest {
    #[serde(skip)]
    pub id: String,
    pub full_name: Option<String>,
    pub contact_type: Option<String>,
    pub status: Option<String>,
    pub document: Option<String>,
    pub unit_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub emails: Option<Vec<Email>>,
    pub phones: Option<Vec<Phone>>,
    pub etag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactResponse {
    pub id: Uuid,
    pub full_name: String,
    pub contact_type: String,
    pub status: String,
    pub document: Option<String>,
    pub unit_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub emails: Vec<Email>,
    pub phones: Vec<Phone>,
    pub etag: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContactSearchRequest {
    pub search_term: Option<String>,
    pub contact_type: Option<String>,
    pub status: Option<String>,
    pub unit_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ContactSearchResponse {
    pub items: Vec<ContactResponse>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ContactStatisticsResponse {
    pub total_contacts: i64,
    pub active_contacts: i64,
    pub inactive_contacts: i64,
    pub persons: i64,
    pub organizations: i64,
    pub departments: i64,
}

// OrgUnit DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrgUnitRequest {
    pub name: String,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrgUnitRequest {
    #[serde(skip)]
    pub id: String,
    pub name: Option<String>,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgUnitResponse {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrgUnitSearchRequest {
    pub search_term: Option<String>,
    pub parent_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrgUnitSearchResponse {
    pub items: Vec<OrgUnitResponse>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrgUnitHierarchyResponse {
    pub items: Vec<OrgUnitResponse>,
    pub children: std::collections::HashMap<Uuid, Vec<OrgUnitResponse>>,
}

// Department DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentRequest {
    pub unit_id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentRequest {
    #[serde(skip)]
    pub id: String,
    pub unit_id: Option<Uuid>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentResponse {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DepartmentSearchRequest {
    pub search_term: Option<String>,
    pub unit_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DepartmentSearchResponse {
    pub items: Vec<DepartmentResponse>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DepartmentStatisticsResponse {
    pub total_departments: i64,
    pub departments_by_unit: std::collections::HashMap<Uuid, i64>,
}

// User DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    #[serde(skip)]
    pub id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub roles: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserSearchRequest {
    pub search_term: Option<String>,
    pub role: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserSearchResponse {
    pub items: Vec<UserResponse>,
    pub total: i64,
}

// AuditEvent DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventResponse {
    pub id: i64,
    pub actor_sub: Option<String>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: String,
    pub before: Option<serde_json::Value>,
    pub after: Option<serde_json::Value>,
    pub at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuditEventSearchRequest {
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub actor_sub: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditEventSearchResponse {
    pub items: Vec<AuditEventResponse>,
    pub total: i64,
}

// SourceRecord DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceRecordResponse {
    pub id: Uuid,
    pub source: String,
    pub source_key: String,
    pub hash: String,
    pub payload: serde_json::Value,
    pub fetched_at: chrono::DateTime<chrono::Utc>,
}

// ContactSource DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactSourceResponse {
    pub contact_id: Uuid,
    pub source_record_id: Uuid,
    pub confidence: f64,
}

// MergeCandidate DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeCandidateResponse {
    pub contact_a: Uuid,
    pub contact_b: Uuid,
    pub score: f64,
    pub features: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct MergeCandidateSearchResponse {
    pub items: Vec<MergeCandidateResponse>,
    pub total: i64,
}

// MergeDecision DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMergeDecisionRequest {
    pub primary_contact: Uuid,
    pub duplicate_contact: Uuid,
    pub decision: String,
    pub chosen_fields: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeDecisionResponse {
    pub primary_contact: Uuid,
    pub duplicate_contact: Uuid,
    pub decision: String,
    pub chosen_fields: Option<serde_json::Value>,
    pub decided_by: Option<Uuid>,
    pub decided_at: chrono::DateTime<chrono::Utc>,
}

// WebhookReceipt DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookReceiptResponse {
    pub id: Uuid,
    pub source: String,
    pub nonce: String,
    pub received_at: chrono::DateTime<chrono::Utc>,
}

// Conversion implementations
impl From<Contact> for ContactResponse {
    fn from(contact: Contact) -> Self {
        ContactResponse {
            id: contact.id.0,
            full_name: contact.full_name,
            contact_type: contact.contact_type.to_string(),
            status: contact.status.to_string(),
            document: contact.document,
            unit_id: contact.unit_id.map(|id| id.0),
            department_id: contact.department_id.map(|id| id.0),
            emails: contact.emails,
            phones: contact.phones,
            etag: contact.etag,
            created_at: contact.created_at,
            updated_at: contact.updated_at,
        }
    }
}

impl From<OrgUnit> for OrgUnitResponse {
    fn from(org_unit: OrgUnit) -> Self {
        OrgUnitResponse {
            id: org_unit.id.0,
            name: org_unit.name.value,
            parent_id: org_unit.parent_id.map(|id| id.0),
            created_at: org_unit.created_at,
            updated_at: org_unit.updated_at,
        }
    }
}

impl From<Department> for DepartmentResponse {
    fn from(department: Department) -> Self {
        DepartmentResponse {
            id: department.id.0,
            unit_id: department.unit_id.0,
            name: department.name.value,
            created_at: department.created_at,
            updated_at: department.updated_at,
        }
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id.0,
            username: user.username.value,
            email: user.email.value,
            roles: user.roles.into_iter().map(|r| r.value).collect(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<AuditEvent> for AuditEventResponse {
    fn from(event: AuditEvent) -> Self {
        AuditEventResponse {
            id: event.id.0,
            actor_sub: event.actor_sub,
            action: event.action.value,
            entity_type: event.entity_type.value,
            entity_id: event.entity_id,
            before: event.before,
            after: event.after,
            at: event.at,
        }
    }
}

impl From<SourceRecord> for SourceRecordResponse {
    fn from(record: SourceRecord) -> Self {
        SourceRecordResponse {
            id: record.id.0,
            source: record.source.value,
            source_key: record.source_key.value,
            hash: record.hash.value,
            payload: record.payload,
            fetched_at: record.fetched_at,
        }
    }
}

impl From<ContactSource> for ContactSourceResponse {
    fn from(contact_source: ContactSource) -> Self {
        ContactSourceResponse {
            contact_id: contact_source.contact_id.0,
            source_record_id: contact_source.source_record_id.0,
            confidence: contact_source.confidence,
        }
    }
}

impl From<MergeCandidate> for MergeCandidateResponse {
    fn from(candidate: MergeCandidate) -> Self {
        MergeCandidateResponse {
            contact_a: candidate.contact_a.0,
            contact_b: candidate.contact_b.0,
            score: candidate.score,
            features: candidate.features,
        }
    }
}

impl From<MergeDecision> for MergeDecisionResponse {
    fn from(decision: MergeDecision) -> Self {
        MergeDecisionResponse {
            primary_contact: decision.primary_contact.0,
            duplicate_contact: decision.duplicate_contact.0,
            decision: decision.decision.to_string(),
            chosen_fields: decision.chosen_fields,
            decided_by: decision.decided_by.map(|id| id.0),
            decided_at: decision.decided_at,
        }
    }
}

impl From<WebhookReceipt> for WebhookReceiptResponse {
    fn from(receipt: WebhookReceipt) -> Self {
        WebhookReceiptResponse {
            id: receipt.id.0,
            source: receipt.source.value,
            nonce: receipt.nonce.value,
            received_at: receipt.received_at,
        }
    }
}