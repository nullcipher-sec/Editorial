use super::user::AreaCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: Uuid,
    #[serde(rename = "employeeCode")]
    pub employee_code: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    pub email: String,
    pub department: AreaCode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobOpening {
    pub id: Uuid,
    pub title: String,
    pub area: AreaCode,
    pub open: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub id: Uuid,
    #[serde(rename = "jobOpeningId")]
    pub job_opening_id: Uuid,
    #[serde(rename = "fullName")]
    pub full_name: String,
    pub stage: String,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateJobOpeningInput {
    #[validate(length(min = 5, max = 255, message = "El título debe tener entre 5 y 255 caracteres"))]
    pub title: String,
    pub area: AreaCode,
    #[validate(length(min = 10, max = 2000, message = "La descripción debe tener entre 10 y 2000 caracteres"))]
    pub description: String,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCandidateInput {
    pub job_opening_id: Uuid,
    #[validate(length(min = 3, max = 255, message = "El nombre completo debe tener entre 3 y 255 caracteres"))]
    pub full_name: String,
    #[validate(email(message = "El formato de correo no es válido"))]
    pub email: String,
    #[validate(length(max = 1000, message = "El resumen del CV no puede superar 1000 caracteres"))]
    pub resume_summary: Option<String>,
}
