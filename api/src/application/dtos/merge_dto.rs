// ============================================================================
// MERGE DTOs - DATA TRANSFER OBJECTS
// ============================================================================
// DTOs para operações de merge de contatos na camada de aplicação
// Seguem o padrão de separação entre camadas da Clean Architecture

use crate::domain::entities::{
    ContactSource, MergeCandidate, MergeDecision, SourceRecord, WebhookReceipt,
};
use crate::domain::value_objects::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// SOURCE RECORD DTOs
// ============================================================================

/// DTO de resposta para registros de origem
/// Contém dados de registros vindos de fontes externas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceRecordResponse {
    /// ID único do registro de origem
    pub id: Uuid,
    /// Nome da fonte de dados
    pub source: String,
    /// Chave única na fonte de origem
    pub source_key: String,
    /// Hash do conteúdo para detecção de mudanças
    pub hash: String,
    /// Dados brutos do registro (JSON)
    pub payload: serde_json::Value,
    /// Data de quando foi obtido da fonte
    pub fetched_at: chrono::DateTime<chrono::Utc>,
}

// ============================================================================
// CONTACT SOURCE DTOs
// ============================================================================

/// DTO de resposta para associações entre contatos e registros de origem
/// Representa a ligação entre um contato e um registro de origem externa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactSourceResponse {
    /// ID do contato associado
    pub contact_id: Uuid,
    /// ID do registro de origem
    pub source_record_id: Uuid,
    /// Nível de confiança da associação (0.0 a 1.0)
    pub confidence: f64,
}

// ============================================================================
// MERGE CANDIDATE DTOs
// ============================================================================

/// DTO de resposta para candidatos a merge
/// Representa pares de contatos que podem ser duplicatas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeCandidateResponse {
    /// ID do primeiro contato
    pub contact_a: Uuid,
    /// ID do segundo contato
    pub contact_b: Uuid,
    /// Score de similaridade (0.0 a 1.0)
    pub score: f64,
    /// Características que levaram à similaridade (JSON)
    pub features: serde_json::Value,
}

/// DTO de resposta para busca de candidatos a merge
/// Contém lista paginada de candidatos e metadados
#[derive(Debug, Clone, Serialize)]
pub struct MergeCandidateSearchResponse {
    /// Lista de candidatos a merge encontrados
    pub items: Vec<MergeCandidateResponse>,
    /// Total de registros disponíveis
    pub total: i64,
}

// ============================================================================
// MERGE DECISION DTOs
// ============================================================================

/// DTO para criação de decisões de merge
/// Contém dados para decidir sobre um merge de contatos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMergeDecisionRequest {
    /// ID do contato que será mantido como principal
    pub primary_contact: Uuid,
    /// ID do contato que será removido (duplicata)
    pub duplicate_contact: Uuid,
    /// Decisão tomada (MERGE, REJECT, MANUAL_REVIEW)
    pub decision: String,
    /// Campos escolhidos para o merge (JSON)
    pub chosen_fields: Option<serde_json::Value>,
}

/// DTO de resposta para decisões de merge
/// Contém dados de uma decisão de merge já tomada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeDecisionResponse {
    /// ID do contato principal
    pub primary_contact: Uuid,
    /// ID do contato duplicata
    pub duplicate_contact: Uuid,
    /// Decisão tomada
    pub decision: String,
    /// Campos escolhidos para o merge
    pub chosen_fields: Option<serde_json::Value>,
    /// ID do usuário que tomou a decisão
    pub decided_by: Option<Uuid>,
    /// Data da decisão
    pub decided_at: chrono::DateTime<chrono::Utc>,
}

// ============================================================================
// WEBHOOK RECEIPT DTOs
// ============================================================================

/// DTO de resposta para recibos de webhook
/// Contém dados de webhooks recebidos para auditoria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookReceiptResponse {
    /// ID único do recibo
    pub id: Uuid,
    /// Nome da fonte que enviou o webhook
    pub source: String,
    /// Nonce único para evitar processamento duplicado
    pub nonce: String,
    /// Data de recebimento do webhook
    pub received_at: chrono::DateTime<chrono::Utc>,
}

// ============================================================================
// CONVERSÕES - DOMAIN ENTITIES TO DTOs
// ============================================================================

/// Implementação de conversão de entidade de domínio para DTO de resposta
/// Permite transformar SourceRecord (domínio) em SourceRecordResponse (aplicação)
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

/// Implementação de conversão de entidade de domínio para DTO de resposta
/// Permite transformar ContactSource (domínio) em ContactSourceResponse (aplicação)
impl From<ContactSource> for ContactSourceResponse {
    fn from(contact_source: ContactSource) -> Self {
        ContactSourceResponse {
            contact_id: contact_source.contact_id.0,
            source_record_id: contact_source.source_record_id.0,
            confidence: contact_source.confidence,
        }
    }
}

/// Implementação de conversão de entidade de domínio para DTO de resposta
/// Permite transformar MergeCandidate (domínio) em MergeCandidateResponse (aplicação)
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

/// Implementação de conversão de entidade de domínio para DTO de resposta
/// Permite transformar MergeDecision (domínio) em MergeDecisionResponse (aplicação)
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

/// Implementação de conversão de entidade de domínio para DTO de resposta
/// Permite transformar WebhookReceipt (domínio) em WebhookReceiptResponse (aplicação)
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
