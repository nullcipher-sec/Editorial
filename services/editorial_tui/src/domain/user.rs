use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AreaCode {
    Attention,
    Production,
    Recruitment,
    Hr,
    Marketing,
}

impl AreaCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            AreaCode::Attention => "ATTENTION",
            AreaCode::Production => "PRODUCTION",
            AreaCode::Recruitment => "RECRUITMENT",
            AreaCode::Hr => "HR",
            AreaCode::Marketing => "MARKETING",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AreaCode::Attention => "Atención al Cliente",
            AreaCode::Production => "Contenido y Producción",
            AreaCode::Recruitment => "Reclutamiento",
            AreaCode::Hr => "Recursos Humanos",
            AreaCode::Marketing => "Marketing",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUser {
    pub id: Uuid,
    pub ldap_uid: String,
    pub display_name: String,
    pub area: AreaCode,
}
