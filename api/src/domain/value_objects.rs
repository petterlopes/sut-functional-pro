use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

// Common Value Objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub Uuid);

impl EntityId {
    pub fn new() -> Self {
        EntityId(Uuid::new_v4())
    }

    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Uuid::from_str(s).map(EntityId)
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Contact Value Objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContactId(pub Uuid);

impl ContactId {
    pub fn new() -> Self {
        ContactId(Uuid::new_v4())
    }

    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Uuid::from_str(s).map(ContactId)
    }
}

impl Default for ContactId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ContactId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email {
    pub value: String,
    pub is_primary: bool,
}

impl Email {
    pub fn new(value: String, is_primary: bool) -> Result<Self, String> {
        if !value.contains('@') {
            return Err("Invalid email format".to_string());
        }
        Ok(Email { value, is_primary })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phone {
    pub e164: String,
    pub extension: Option<String>,
    pub phone_type: PhoneType,
    pub is_primary: bool,
}

impl Phone {
    pub fn new(
        e164: String,
        extension: Option<String>,
        phone_type: PhoneType,
        is_primary: bool,
    ) -> Result<Self, String> {
        if e164.is_empty() {
            return Err("E164 phone number cannot be empty".to_string());
        }
        Ok(Phone {
            e164,
            extension,
            phone_type,
            is_primary,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhoneType {
    Mobile,
    Work,
    Home,
    Other,
}

impl fmt::Display for PhoneType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PhoneType::Mobile => write!(f, "MOBILE"),
            PhoneType::Work => write!(f, "WORK"),
            PhoneType::Home => write!(f, "HOME"),
            PhoneType::Other => write!(f, "OTHER"),
        }
    }
}

impl FromStr for PhoneType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "MOBILE" => Ok(PhoneType::Mobile),
            "WORK" => Ok(PhoneType::Work),
            "HOME" => Ok(PhoneType::Home),
            "OTHER" => Ok(PhoneType::Other),
            _ => Err(format!("'{}' is not a valid PhoneType", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContactType {
    Person,
    Organization,
    Department,
}

impl fmt::Display for ContactType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContactType::Person => write!(f, "PERSON"),
            ContactType::Organization => write!(f, "ORGANIZATION"),
            ContactType::Department => write!(f, "DEPARTMENT"),
        }
    }
}

impl FromStr for ContactType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PERSON" => Ok(ContactType::Person),
            "ORGANIZATION" => Ok(ContactType::Organization),
            "DEPARTMENT" => Ok(ContactType::Department),
            _ => Err(format!("'{}' is not a valid ContactType", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContactStatus {
    Active,
    Inactive,
    Pending,
}

impl fmt::Display for ContactStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContactStatus::Active => write!(f, "ACTIVE"),
            ContactStatus::Inactive => write!(f, "INACTIVE"),
            ContactStatus::Pending => write!(f, "PENDING"),
        }
    }
}

impl FromStr for ContactStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ACTIVE" => Ok(ContactStatus::Active),
            "INACTIVE" => Ok(ContactStatus::Inactive),
            "PENDING" => Ok(ContactStatus::Pending),
            _ => Err(format!("'{}' is not a valid ContactStatus", s)),
        }
    }
}

// OrgUnit Value Objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrgUnitId(pub Uuid);

impl OrgUnitId {
    pub fn new() -> Self {
        OrgUnitId(Uuid::new_v4())
    }

    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Uuid::from_str(s).map(OrgUnitId)
    }
}

impl Default for OrgUnitId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for OrgUnitId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrgUnitName {
    pub value: String,
}

impl OrgUnitName {
    pub fn new(value: String) -> Result<Self, String> {
        if value.trim().is_empty() {
            return Err("OrgUnit name cannot be empty".to_string());
        }
        Ok(OrgUnitName { value })
    }
}

// Department Value Objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DepartmentId(pub Uuid);

impl DepartmentId {
    pub fn new() -> Self {
        DepartmentId(Uuid::new_v4())
    }

    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Uuid::from_str(s).map(DepartmentId)
    }
}

impl Default for DepartmentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DepartmentId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepartmentName {
    pub value: String,
}

impl DepartmentName {
    pub fn new(value: String) -> Result<Self, String> {
        if value.trim().is_empty() {
            return Err("Department name cannot be empty".to_string());
        }
        Ok(DepartmentName { value })
    }
}

// User Value Objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        UserId(Uuid::new_v4())
    }

    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Uuid::from_str(s).map(UserId)
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Username {
    pub value: String,
}

impl Username {
    pub fn new(value: String) -> Result<Self, String> {
        if value.trim().is_empty() {
            return Err("Username cannot be empty".to_string());
        }
        if value.len() < 3 {
            return Err("Username must be at least 3 characters long".to_string());
        }
        Ok(Username { value })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserEmail {
    pub value: String,
}

impl UserEmail {
    pub fn new(value: String) -> Result<Self, String> {
        if !value.contains('@') {
            return Err("Invalid email format".to_string());
        }
        Ok(UserEmail { value })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Password {
    pub value: String,
}

impl Password {
    pub fn new(value: String) -> Result<Self, String> {
        if value.len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }
        Ok(Password { value })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Role {
    pub value: String,
}

impl Role {
    pub fn new(value: String) -> Result<Self, String> {
        if value.trim().is_empty() {
            return Err("Role cannot be empty".to_string());
        }
        Ok(Role { value })
    }
}

// Audit Value Objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AuditEventId(pub i64);

impl AuditEventId {
    pub fn new(value: i64) -> Self {
        AuditEventId(value)
    }
}

impl fmt::Display for AuditEventId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Action {
    pub value: String,
}

impl Action {
    pub fn new(value: String) -> Result<Self, String> {
        if value.trim().is_empty() {
            return Err("Action cannot be empty".to_string());
        }
        Ok(Action { value })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityType {
    pub value: String,
}

impl EntityType {
    pub fn new(value: String) -> Result<Self, String> {
        if value.trim().is_empty() {
            return Err("Entity type cannot be empty".to_string());
        }
        Ok(EntityType { value })
    }
}

// Source Record Value Objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceRecordId(pub Uuid);

impl SourceRecordId {
    pub fn new() -> Self {
        SourceRecordId(Uuid::new_v4())
    }

    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Uuid::from_str(s).map(SourceRecordId)
    }
}

impl Default for SourceRecordId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SourceRecordId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    pub value: String,
}

impl Source {
    pub fn new(value: String) -> Result<Self, String> {
        if value.trim().is_empty() {
            return Err("Source cannot be empty".to_string());
        }
        Ok(Source { value })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceKey {
    pub value: String,
}

impl SourceKey {
    pub fn new(value: String) -> Result<Self, String> {
        if value.trim().is_empty() {
            return Err("Source key cannot be empty".to_string());
        }
        Ok(SourceKey { value })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hash {
    pub value: String,
}

impl Hash {
    pub fn new(value: String) -> Result<Self, String> {
        if value.trim().is_empty() {
            return Err("Hash cannot be empty".to_string());
        }
        Ok(Hash { value })
    }
}

// Webhook Value Objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WebhookReceiptId(pub Uuid);

impl WebhookReceiptId {
    pub fn new() -> Self {
        WebhookReceiptId(Uuid::new_v4())
    }

    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Uuid::from_str(s).map(WebhookReceiptId)
    }
}

impl Default for WebhookReceiptId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for WebhookReceiptId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Nonce {
    pub value: String,
}

impl Nonce {
    pub fn new(value: String) -> Result<Self, String> {
        if value.trim().is_empty() {
            return Err("Nonce cannot be empty".to_string());
        }
        Ok(Nonce { value })
    }
}

// Common Value Object Errors
#[derive(Debug, Error)]
pub enum ValueObjectError {
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    #[error("Empty value not allowed")]
    EmptyValue,
    #[error("Value too short: minimum {0} characters required")]
    TooShort(usize),
    #[error("Value too long: maximum {0} characters allowed")]
    TooLong(usize),
}
