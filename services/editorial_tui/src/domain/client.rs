use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CaseStatus {
    Open,
    InProgress,
    Closed,
}

impl CaseStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            CaseStatus::Open => "OPEN",
            CaseStatus::InProgress => "IN_PROGRESS",
            CaseStatus::Closed => "CLOSED",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            CaseStatus::Open => "Abierto",
            CaseStatus::InProgress => "En Progreso",
            CaseStatus::Closed => "Cerrado",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            CaseStatus::Open => CaseStatus::InProgress,
            CaseStatus::InProgress => CaseStatus::Closed,
            CaseStatus::Closed => CaseStatus::Open,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub id: Uuid,
    #[serde(rename = "fullName")]
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportCase {
    pub id: Uuid,
    #[serde(rename = "clientId")]
    pub client_id: Uuid,
    pub subject: String,
    pub description: Option<String>,
    pub status: CaseStatus,
    #[serde(rename = "assignedTo")]
    pub assigned_to: Option<Uuid>,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct CreateClientInput {
    #[validate(length(min = 3, max = 255, message = "El nombre debe tener entre 3 y 255 caracteres"))]
    pub full_name: String,
    #[validate(email(message = "El formato de correo no es válido"))]
    pub email: Option<String>,
    #[validate(length(min = 7, max = 32, message = "El teléfono debe tener entre 7 y 32 caracteres"))]
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct CreateCaseInput {
    pub client_id: Uuid,
    #[validate(length(min = 5, max = 255, message = "El asunto debe tener entre 5 y 255 caracteres"))]
    pub subject: String,
    #[validate(length(min = 10, max = 2000, message = "La descripción debe tener entre 10 y 2000 caracteres"))]
    pub description: String,
}
