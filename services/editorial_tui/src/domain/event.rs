use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    #[serde(rename = "startsAt")]
    pub starts_at: DateTime<Utc>,
    #[serde(rename = "endsAt")]
    pub ends_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct CreateEventInput {
    #[validate(length(min = 3, max = 255, message = "El nombre del evento debe tener entre 3 y 255 caracteres"))]
    pub name: String,
    #[validate(length(max = 1000, message = "La descripción no puede superar 1000 caracteres"))]
    pub description: Option<String>,
    #[validate(length(min = 3, max = 255, message = "La ubicación debe tener entre 3 y 255 caracteres"))]
    pub location: Option<String>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: Option<DateTime<Utc>>,
}
