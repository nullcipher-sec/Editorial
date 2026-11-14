pub mod api;
pub mod auth;
pub mod config;
pub mod domain;

use anyhow::Result;
use api::{EditorialApi, MockApiClient};
use config::AppConfig;
use domain::CreateClientInput;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::from_env();
    println!("Sultana del Lago Editores - TUI Base Inicializada");
    println!("Configuración: API URL = {}", config.core_api_url);

    let api = MockApiClient::new();
    let session = api.login("atencion.agente", "password").await?;
    println!("Sesión iniciada con éxito para: {} ({:?})", session.user.display_name, session.user.area);

    let clients = api.list_clients(0, 10).await?;
    println!("Clientes cargados en memoria: {}", clients.total_elements);

    let valid_client = api.create_client(CreateClientInput {
        full_name: "Librería Nueva Esparta".to_string(),
        email: Some("contacto@librerianuevaesparta.com".to_string()),
        phone: Some("+58-295-2612233".to_string()),
    }).await?;
    println!("Nuevo cliente registrado: {} ({})", valid_client.full_name, valid_client.id);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::*;
    use validator::Validate;

    #[tokio::test]
    async fn test_mock_api_login_and_areas() {
        let api = MockApiClient::new();
        let session = api.login("prod.editor", "secret").await.unwrap();
        assert_eq!(session.user.area, AreaCode::Production);
        assert!(session.token.is_some());
    }

    #[tokio::test]
    async fn test_validation_rejects_invalid_inputs() {
        let invalid_client = CreateClientInput {
            full_name: "AB".to_string(),
            email: Some("correo_invalido_sin_arroba".to_string()),
            phone: None,
        };
        assert!(invalid_client.validate().is_err());

        let valid_client = CreateClientInput {
            full_name: "Librería Bolívar C.A.".to_string(),
            email: Some("ventas@libreriabolivar.com".to_string()),
            phone: Some("+58-285-6321144".to_string()),
        };
        assert!(valid_client.validate().is_ok());
    }

    #[tokio::test]
    async fn test_case_status_transition() {
        let api = MockApiClient::new();
        let cases = api.list_cases(0, 10).await.unwrap();
        assert!(!cases.content.is_empty());
        let first_case = &cases.content[0];
        let updated = api.update_case_status(first_case.id, CaseStatus::Closed).await.unwrap();
        assert_eq!(updated.status, CaseStatus::Closed);
    }

    #[tokio::test]
    async fn test_event_date_validation() {
        let api = MockApiClient::new();
        let now = chrono::Utc::now();
        let invalid_event = CreateEventInput {
            name: "Feria del Libro".to_string(),
            description: None,
            location: Some("Maracaibo".to_string()),
            starts_at: now,
            ends_at: Some(now - chrono::Duration::hours(2)),
        };
        let res = api.create_event(invalid_event).await;
        assert!(res.is_err());
    }
}
