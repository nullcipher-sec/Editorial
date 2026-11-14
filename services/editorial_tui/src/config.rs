use crate::domain::AreaCode;
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub core_api_url: String,
    pub terminal_area: Option<AreaCode>,
    pub mock_mode: bool,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let core_api_url = env::var("CORE_API_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
        let mock_mode = env::var("MOCK_MODE")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        let terminal_area = env::var("TERMINAL_AREA").ok().and_then(|val| {
            match val.to_uppercase().as_str() {
                "ATTENTION" => Some(AreaCode::Attention),
                "PRODUCTION" => Some(AreaCode::Production),
                "RECRUITMENT" => Some(AreaCode::Recruitment),
                "HR" => Some(AreaCode::Hr),
                "MARKETING" => Some(AreaCode::Marketing),
                _ => None,
            }
        });

        Self {
            core_api_url,
            terminal_area,
            mock_mode,
        }
    }
}
