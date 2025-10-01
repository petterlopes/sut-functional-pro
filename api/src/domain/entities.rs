use std::fmt;
use std::str::FromStr;
use crate::domain::value_objects::*;
use crate::domain::errors::DomainError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Contact Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: ContactId,
    pub full_name: String,
    pub contact_type: ContactType,
    pub status: ContactStatus,
    pub document: Option<String>, // PII document
    pub unit_id: Option<OrgUnitId>,
    pub department_id: Option<DepartmentId>,
    pub emails: Vec<Email>,
    pub phones: Vec<Phone>,
    pub etag: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Contact {
    pub fn new(
        full_name: String,
        contact_type: ContactType,
        status: ContactStatus,
        document: Option<String>,
        unit_id: Option<OrgUnitId>,
        department_id: Option<DepartmentId>,
        emails: Vec<Email>,
        phones: Vec<Phone>,
    ) -> Result<Self, DomainError> {
        if full_name.trim().is_empty() {
            return Err(DomainError::ValidationError("Full name cannot be empty".to_string()));
        }

        Ok(Contact {
            id: ContactId::new(),
            full_name,
            contact_type,
            status,
            document,
            unit_id,
            department_id,
            emails,
            phones,
            etag: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn update_full_name(&mut self, full_name: String) -> Result<(), DomainError> {
        if full_name.trim().is_empty() {
            return Err(DomainError::ValidationError("Full name cannot be empty".to_string()));
        }
        self.full_name = full_name;
        self.etag = Uuid::new_v4().to_string();
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_contact_type(&mut self, contact_type: ContactType) {
        self.contact_type = contact_type;
        self.etag = Uuid::new_v4().to_string();
        self.updated_at = Utc::now();
    }

    pub fn update_status(&mut self, status: ContactStatus) {
        self.status = status;
        self.etag = Uuid::new_v4().to_string();
        self.updated_at = Utc::now();
    }

    pub fn update_document(&mut self, document: Option<String>) {
        self.document = document;
        self.etag = Uuid::new_v4().to_string();
        self.updated_at = Utc::now();
    }

    pub fn update_unit_id(&mut self, unit_id: Option<OrgUnitId>) {
        self.unit_id = unit_id;
        self.etag = Uuid::new_v4().to_string();
        self.updated_at = Utc::now();
    }

    pub fn update_department_id(&mut self, department_id: Option<DepartmentId>) {
        self.department_id = department_id;
        self.etag = Uuid::new_v4().to_string();
        self.updated_at = Utc::now();
    }

    pub fn add_email(&mut self, email: Email) -> Result<(), DomainError> {
        if self.emails.iter().any(|e| e.value == email.value) {
            return Err(DomainError::Conflict(format!("Email {} already exists", email.value)));
        }
        if email.is_primary {
            self.emails.iter_mut().for_each(|e| e.is_primary = false);
        }
        self.emails.push(email);
        self.etag = Uuid::new_v4().to_string();
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn add_phone(&mut self, phone: Phone) -> Result<(), DomainError> {
        if self.phones.iter().any(|p| p.e164 == phone.e164) {
            return Err(DomainError::Conflict(format!("Phone {} already exists", phone.e164)));
        }
        if phone.is_primary {
            self.phones.iter_mut().for_each(|p| p.is_primary = false);
        }
        self.phones.push(phone);
        self.etag = Uuid::new_v4().to_string();
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn remove_email(&mut self, email_value: &str) {
        self.emails.retain(|e| e.value != email_value);
        self.etag = Uuid::new_v4().to_string();
        self.updated_at = Utc::now();
    }

    pub fn remove_phone(&mut self, phone_e164: &str) {
        self.phones.retain(|p| p.e164 != phone_e164);
        self.etag = Uuid::new_v4().to_string();
        self.updated_at = Utc::now();
    }

    pub fn is_active(&self) -> bool {
        self.status == ContactStatus::Active
    }

    pub fn get_primary_email(&self) -> Option<&Email> {
        self.emails.iter().find(|e| e.is_primary)
    }

    pub fn get_primary_phone(&self) -> Option<&Phone> {
        self.phones.iter().find(|p| p.is_primary)
    }
}

// OrgUnit Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgUnit {
    pub id: OrgUnitId,
    pub name: OrgUnitName,
    pub parent_id: Option<OrgUnitId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl OrgUnit {
    pub fn new(name: OrgUnitName, parent_id: Option<OrgUnitId>) -> Self {
        OrgUnit {
            id: OrgUnitId::new(),
            name,
            parent_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn update_name(&mut self, name: OrgUnitName) {
        self.name = name;
        self.updated_at = Utc::now();
    }

    pub fn set_parent(&mut self, parent_id: Option<OrgUnitId>) {
        self.parent_id = parent_id;
        self.updated_at = Utc::now();
    }

    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    pub fn has_children(&self) -> bool {
        // This would be determined by the repository
        false
    }
}

// Department Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    pub id: DepartmentId,
    pub unit_id: OrgUnitId,
    pub name: DepartmentName,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Department {
    pub fn new(unit_id: OrgUnitId, name: DepartmentName) -> Self {
        Department {
            id: DepartmentId::new(),
            unit_id,
            name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn update_name(&mut self, name: DepartmentName) {
        self.name = name;
        self.updated_at = Utc::now();
    }

    pub fn update_unit(&mut self, unit_id: OrgUnitId) {
        self.unit_id = unit_id;
        self.updated_at = Utc::now();
    }
}

// User Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: Username,
    pub email: UserEmail,
    pub password: Password,
    pub roles: Vec<Role>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        username: Username,
        email: UserEmail,
        password: Password,
        roles: Vec<Role>,
    ) -> Self {
        User {
            id: UserId::new(),
            username,
            email,
            password,
            roles,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn update_username(&mut self, username: Username) {
        self.username = username;
        self.updated_at = Utc::now();
    }

    pub fn update_email(&mut self, email: UserEmail) {
        self.email = email;
        self.updated_at = Utc::now();
    }

    pub fn update_password(&mut self, password: Password) {
        self.password = password;
        self.updated_at = Utc::now();
    }

    pub fn add_role(&mut self, role: Role) {
        if !self.roles.iter().any(|r| r.value == role.value) {
            self.roles.push(role);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_role(&mut self, role_value: &str) {
        self.roles.retain(|r| r.value != role_value);
        self.updated_at = Utc::now();
    }

    pub fn has_role(&self, role_value: &str) -> bool {
        self.roles.iter().any(|r| r.value == role_value)
    }

    pub fn has_any_role(&self, role_values: &[&str]) -> bool {
        role_values.iter().any(|&role| self.has_role(role))
    }
}

// AuditEvent Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: AuditEventId,
    pub actor_sub: Option<String>,
    pub action: Action,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub before: Option<serde_json::Value>,
    pub after: Option<serde_json::Value>,
    pub at: DateTime<Utc>,
}

impl AuditEvent {
    pub fn new(
        actor_sub: Option<String>,
        action: Action,
        entity_type: EntityType,
        entity_id: String,
        before: Option<serde_json::Value>,
        after: Option<serde_json::Value>,
    ) -> Self {
        AuditEvent {
            id: AuditEventId::new(0), // Will be set by the database
            actor_sub,
            action,
            entity_type,
            entity_id,
            before,
            after,
            at: Utc::now(),
        }
    }
}

// SourceRecord Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceRecord {
    pub id: SourceRecordId,
    pub source: Source,
    pub source_key: SourceKey,
    pub hash: Hash,
    pub payload: serde_json::Value,
    pub fetched_at: DateTime<Utc>,
}

impl SourceRecord {
    pub fn new(
        source: Source,
        source_key: SourceKey,
        hash: Hash,
        payload: serde_json::Value,
    ) -> Self {
        SourceRecord {
            id: SourceRecordId::new(),
            source,
            source_key,
            hash,
            payload,
            fetched_at: Utc::now(),
        }
    }
}

// ContactSource Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactSource {
    pub contact_id: ContactId,
    pub source_record_id: SourceRecordId,
    pub confidence: f64,
}

impl ContactSource {
    pub fn new(contact_id: ContactId, source_record_id: SourceRecordId, confidence: f64) -> Result<Self, DomainError> {
        if confidence < 0.0 || confidence > 1.0 {
            return Err(DomainError::ValidationError("Confidence must be between 0.0 and 1.0".to_string()));
        }
        Ok(ContactSource {
            contact_id,
            source_record_id,
            confidence,
        })
    }
}

// MergeCandidate Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeCandidate {
    pub contact_a: ContactId,
    pub contact_b: ContactId,
    pub score: f64,
    pub features: serde_json::Value,
}

impl MergeCandidate {
    pub fn new(contact_a: ContactId, contact_b: ContactId, score: f64, features: serde_json::Value) -> Result<Self, DomainError> {
        if score < 0.0 || score > 1.0 {
            return Err(DomainError::ValidationError("Score must be between 0.0 and 1.0".to_string()));
        }
        Ok(MergeCandidate {
            contact_a,
            contact_b,
            score,
            features,
        })
    }
}

// MergeDecision Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeDecision {
    pub primary_contact: ContactId,
    pub duplicate_contact: ContactId,
    pub decision: MergeDecisionType,
    pub chosen_fields: Option<serde_json::Value>,
    pub decided_by: Option<UserId>,
    pub decided_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeDecisionType {
    Merge,
    Reject,
}

impl fmt::Display for MergeDecisionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MergeDecisionType::Merge => write!(f, "MERGE"),
            MergeDecisionType::Reject => write!(f, "REJECT"),
        }
    }
}

impl FromStr for MergeDecisionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "MERGE" => Ok(MergeDecisionType::Merge),
            "REJECT" => Ok(MergeDecisionType::Reject),
            _ => Err(format!("'{}' is not a valid MergeDecisionType", s)),
        }
    }
}

impl MergeDecision {
    pub fn new(
        primary_contact: ContactId,
        duplicate_contact: ContactId,
        decision: MergeDecisionType,
        chosen_fields: Option<serde_json::Value>,
        decided_by: Option<UserId>,
    ) -> Self {
        MergeDecision {
            primary_contact,
            duplicate_contact,
            decision,
            chosen_fields,
            decided_by,
            decided_at: Utc::now(),
        }
    }
}

// WebhookReceipt Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookReceipt {
    pub id: WebhookReceiptId,
    pub source: Source,
    pub nonce: Nonce,
    pub received_at: DateTime<Utc>,
}

impl WebhookReceipt {
    pub fn new(source: Source, nonce: Nonce) -> Self {
        WebhookReceipt {
            id: WebhookReceiptId::new(),
            source,
            nonce,
            received_at: Utc::now(),
        }
    }
}