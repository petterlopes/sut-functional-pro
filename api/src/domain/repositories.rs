use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::*;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::*;

// Contact Repository
#[async_trait]
pub trait ContactRepository: Send + Sync {
    async fn find_by_id(&self, id: &ContactId) -> Result<Option<Contact>, DomainError>;
    async fn find_all(
        &self,
        criteria: &ContactSearchCriteria,
    ) -> Result<ContactSearchResult, DomainError>;
    async fn save(&self, contact: &Contact) -> Result<Contact, DomainError>;
    async fn update(&self, contact: &Contact) -> Result<Contact, DomainError>;
    async fn delete(&self, id: &ContactId) -> Result<(), DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<Contact>, DomainError>;
    async fn find_by_document(&self, document: &str) -> Result<Option<Contact>, DomainError>;
    async fn find_by_name(&self, name: &str) -> Result<Vec<Contact>, DomainError>;
    async fn find_by_unit(&self, unit_id: &OrgUnitId) -> Result<Vec<Contact>, DomainError>;
    async fn find_by_department(
        &self,
        department_id: &DepartmentId,
    ) -> Result<Vec<Contact>, DomainError>;
    async fn count_by_status(&self, status: &ContactStatus) -> Result<i64, DomainError>;
    async fn count_by_type(&self, contact_type: &ContactType) -> Result<i64, DomainError>;
    async fn get_statistics(&self) -> Result<ContactStatistics, DomainError>;
}

#[derive(Debug, Clone)]
pub struct ContactSearchCriteria {
    pub full_name: Option<String>,
    pub contact_type: Option<ContactType>,
    pub status: Option<ContactStatus>,
    pub unit_id: Option<OrgUnitId>,
    pub department_id: Option<DepartmentId>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct ContactSearchResult {
    pub items: Vec<Contact>,
    pub total: i64,
}

#[derive(Debug, Clone)]
pub struct ContactStatistics {
    pub total_contacts: i64,
    pub active_contacts: i64,
    pub inactive_contacts: i64,
    pub persons: i64,
    pub organizations: i64,
    pub departments: i64,
}

// OrgUnit Repository
#[async_trait]
pub trait OrgUnitRepository: Send + Sync {
    async fn find_by_id(&self, id: &OrgUnitId) -> Result<Option<OrgUnit>, DomainError>;
    async fn find_all(
        &self,
        criteria: &OrgUnitSearchCriteria,
    ) -> Result<OrgUnitSearchResult, DomainError>;
    async fn save(&self, org_unit: &OrgUnit) -> Result<OrgUnit, DomainError>;
    async fn update(&self, org_unit: &OrgUnit) -> Result<OrgUnit, DomainError>;
    async fn delete(&self, id: &OrgUnitId) -> Result<(), DomainError>;
    async fn find_by_name(&self, name: &str) -> Result<Vec<OrgUnit>, DomainError>;
    async fn find_children(&self, parent_id: &OrgUnitId) -> Result<Vec<OrgUnit>, DomainError>;
    async fn find_root_units(&self) -> Result<Vec<OrgUnit>, DomainError>;
    async fn get_hierarchy(&self, id: &OrgUnitId) -> Result<Vec<OrgUnit>, DomainError>;
}

#[derive(Debug, Clone)]
pub struct OrgUnitSearchCriteria {
    pub name: Option<String>,
    pub parent_id: Option<OrgUnitId>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct OrgUnitSearchResult {
    pub items: Vec<OrgUnit>,
    pub total: i64,
}

// Department Repository
#[async_trait]
pub trait DepartmentRepository: Send + Sync {
    async fn find_by_id(&self, id: &DepartmentId) -> Result<Option<Department>, DomainError>;
    async fn find_all(
        &self,
        criteria: &DepartmentSearchCriteria,
    ) -> Result<DepartmentSearchResult, DomainError>;
    async fn save(&self, department: &Department) -> Result<Department, DomainError>;
    async fn update(&self, department: &Department) -> Result<Department, DomainError>;
    async fn delete(&self, id: &DepartmentId) -> Result<(), DomainError>;
    async fn find_by_name(&self, name: &str) -> Result<Vec<Department>, DomainError>;
    async fn find_by_unit(&self, unit_id: &OrgUnitId) -> Result<Vec<Department>, DomainError>;
    async fn get_statistics(&self) -> Result<DepartmentStatistics, DomainError>;
}

#[derive(Debug, Clone)]
pub struct DepartmentSearchCriteria {
    pub name: Option<String>,
    pub unit_id: Option<OrgUnitId>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct DepartmentSearchResult {
    pub items: Vec<Department>,
    pub total: i64,
}

#[derive(Debug, Clone)]
pub struct DepartmentStatistics {
    pub total_departments: i64,
    pub departments_by_unit: std::collections::HashMap<OrgUnitId, i64>,
}

// User Repository
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
    async fn find_all(
        &self,
        criteria: &UserSearchCriteria,
    ) -> Result<UserSearchResult, DomainError>;
    async fn save(&self, user: &User) -> Result<User, DomainError>;
    async fn update(&self, user: &User) -> Result<User, DomainError>;
    async fn delete(&self, id: &UserId) -> Result<(), DomainError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_role(&self, role: &str) -> Result<Vec<User>, DomainError>;
}

#[derive(Debug, Clone)]
pub struct UserSearchCriteria {
    pub username: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct UserSearchResult {
    pub items: Vec<User>,
    pub total: i64,
}

// AuditEvent Repository
#[async_trait]
pub trait AuditEventRepository: Send + Sync {
    async fn save(&self, event: &AuditEvent) -> Result<AuditEvent, DomainError>;
    async fn find_by_entity(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<Vec<AuditEvent>, DomainError>;
    async fn find_by_actor(&self, actor_sub: &str) -> Result<Vec<AuditEvent>, DomainError>;
    async fn find_recent(&self, limit: i64) -> Result<Vec<AuditEvent>, DomainError>;
}

// SourceRecord Repository
#[async_trait]
pub trait SourceRecordRepository: Send + Sync {
    async fn find_by_id(&self, id: &SourceRecordId) -> Result<Option<SourceRecord>, DomainError>;
    async fn find_by_source_and_key(
        &self,
        source: &str,
        source_key: &str,
    ) -> Result<Option<SourceRecord>, DomainError>;
    async fn save(&self, record: &SourceRecord) -> Result<SourceRecord, DomainError>;
    async fn delete(&self, id: &SourceRecordId) -> Result<(), DomainError>;
}

// ContactSource Repository
#[async_trait]
pub trait ContactSourceRepository: Send + Sync {
    async fn save(&self, contact_source: &ContactSource) -> Result<ContactSource, DomainError>;
    async fn find_by_contact(
        &self,
        contact_id: &ContactId,
    ) -> Result<Vec<ContactSource>, DomainError>;
    async fn find_by_source_record(
        &self,
        source_record_id: &SourceRecordId,
    ) -> Result<Vec<ContactSource>, DomainError>;
    async fn delete(
        &self,
        contact_id: &ContactId,
        source_record_id: &SourceRecordId,
    ) -> Result<(), DomainError>;
}

// MergeCandidate Repository
#[async_trait]
pub trait MergeCandidateRepository: Send + Sync {
    async fn save(&self, candidate: &MergeCandidate) -> Result<MergeCandidate, DomainError>;
    async fn find_by_contact(
        &self,
        contact_id: &ContactId,
    ) -> Result<Vec<MergeCandidate>, DomainError>;
    async fn find_top_candidates(&self, limit: i64) -> Result<Vec<MergeCandidate>, DomainError>;
    async fn delete(&self, contact_a: &ContactId, contact_b: &ContactId)
        -> Result<(), DomainError>;
}

// MergeDecision Repository
#[async_trait]
pub trait MergeDecisionRepository: Send + Sync {
    async fn save(&self, decision: &MergeDecision) -> Result<MergeDecision, DomainError>;
    async fn find_by_contact(
        &self,
        contact_id: &ContactId,
    ) -> Result<Vec<MergeDecision>, DomainError>;
    async fn find_by_decider(&self, decided_by: &UserId)
        -> Result<Vec<MergeDecision>, DomainError>;
}

// WebhookReceipt Repository
#[async_trait]
pub trait WebhookReceiptRepository: Send + Sync {
    async fn save(&self, receipt: &WebhookReceipt) -> Result<WebhookReceipt, DomainError>;
    async fn exists(&self, source: &str, nonce: &str) -> Result<bool, DomainError>;
}
