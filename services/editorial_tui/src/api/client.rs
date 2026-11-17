use super::{ApiError, EditorialApi};
use crate::auth::Session;
use crate::domain::{
    Article, ArticleStatus, ArticleSummary, Candidate, CaseStatus, Client, CreateArticleInput,
    CreateCandidateInput, CreateCaseInput, CreateClientInput, CreateEventInput,
    CreateJobOpeningInput, Employee, ErrorBody, Event, JobOpening, PageResponse, SupportCase,
};
use reqwest::{header, Client as HttpClient, StatusCode};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct HttpApiClient {
    client: HttpClient,
    base_url: String,
    token: Arc<RwLock<Option<String>>>,
}

impl HttpApiClient {
    pub fn new(base_url: String) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );

        let client = HttpClient::builder()
            .timeout(Duration::from_secs(5))
            .default_headers(headers)
            .build()
            .unwrap_or_else(|_| HttpClient::new());

        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            token: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn set_token(&self, token: Option<String>) {
        let mut w = self.token.write().await;
        *w = token;
    }

    async fn auth_header(&self) -> Option<String> {
        let r = self.token.read().await;
        r.as_ref().map(|t| format!("Bearer {}", t))
    }

    async fn handle_error(resp: reqwest::Response) -> ApiError {
        let status = resp.status();
        let body_msg = if let Ok(err) = resp.json::<ErrorBody>().await {
            err.message
        } else {
            String::new()
        };

        match status {
            StatusCode::UNAUTHORIZED => ApiError::Unauthorized,
            StatusCode::FORBIDDEN => ApiError::Forbidden,
            StatusCode::NOT_FOUND => ApiError::NotFound,
            StatusCode::NOT_IMPLEMENTED => {
                if body_msg.is_empty() {
                    ApiError::NotImplemented("Operación no implementada aún en el Core".to_string())
                } else {
                    ApiError::NotImplemented(body_msg)
                }
            }
            _ if status.is_client_error() => {
                if body_msg.is_empty() {
                    ApiError::Validation(format!("Petición inválida del cliente: {}", status))
                } else {
                    ApiError::Validation(body_msg)
                }
            }
            _ if status.is_server_error() => {
                if body_msg.is_empty() {
                    ApiError::Network(format!("Error interno del servidor: {}", status))
                } else {
                    ApiError::Network(body_msg)
                }
            }
            _ => ApiError::Network(format!("Estado inesperado: {}", status)),
        }
    }
}

impl EditorialApi for HttpApiClient {
    async fn login(&self, username: &str, password: &str) -> Result<Session, ApiError> {
        let url = format!("{}/api/auth/login", self.base_url);
        let res = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "username": username,
                "password": password
            }))
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    let session: Session = response
                        .json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))?;
                    if let Some(token) = session.token.as_ref() {
                        self.set_token(Some(token.value.clone())).await;
                    }
                    Ok(session)
                } else {
                    Err(Self::handle_error(response).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn list_clients(&self, page: u32, size: u32) -> Result<PageResponse<Client>, ApiError> {
        let url = format!(
            "{}/api/internal/attention/clients?page={}&size={}",
            self.base_url, page, size
        );
        let mut req = self.client.get(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn create_client(&self, input: CreateClientInput) -> Result<Client, ApiError> {
        let url = format!("{}/api/internal/attention/clients", self.base_url);
        let mut req = self.client.post(&url).json(&input);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn list_cases(
        &self,
        page: u32,
        size: u32,
    ) -> Result<PageResponse<SupportCase>, ApiError> {
        let url = format!(
            "{}/api/internal/attention/cases?page={}&size={}",
            self.base_url, page, size
        );
        let mut req = self.client.get(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn delete_client(&self, id: Uuid) -> Result<(), ApiError> {
        let url = format!("{}/api/internal/attention/clients/{}", self.base_url, id);
        let mut req = self.client.delete(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn create_case(&self, input: CreateCaseInput) -> Result<SupportCase, ApiError> {
        let url = format!("{}/api/internal/attention/cases", self.base_url);
        let mut req = self.client.post(&url).json(&input);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn update_case_status(
        &self,
        id: Uuid,
        status: CaseStatus,
    ) -> Result<SupportCase, ApiError> {
        let url = format!("{}/api/internal/attention/cases/{}", self.base_url, id);
        let mut req = self
            .client
            .patch(&url)
            .json(&serde_json::json!({ "status": status.as_str() }));
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn delete_case(&self, id: Uuid) -> Result<(), ApiError> {
        let url = format!("{}/api/internal/attention/cases/{}", self.base_url, id);
        let mut req = self.client.delete(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn list_articles(
        &self,
        page: u32,
        size: u32,
    ) -> Result<PageResponse<ArticleSummary>, ApiError> {
        let url = format!(
            "{}/api/internal/content/articles?page={}&size={}",
            self.base_url, page, size
        );
        let mut req = self.client.get(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn get_article(&self, id: Uuid) -> Result<Article, ApiError> {
        let url = format!("{}/api/internal/content/articles/{}", self.base_url, id);
        let mut req = self.client.get(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }
        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn create_article(&self, input: CreateArticleInput) -> Result<Article, ApiError> {
        let url = format!("{}/api/internal/content/articles", self.base_url);
        let mut req = self.client.post(&url).json(&input);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn update_article_status(
        &self,
        id: Uuid,
        status: ArticleStatus,
    ) -> Result<Article, ApiError> {
        let url = if status == ArticleStatus::Published {
            format!(
                "{}/api/internal/content/articles/{}/publish",
                self.base_url, id
            )
        } else {
            format!(
                "{}/api/internal/content/articles/{}/review",
                self.base_url, id
            )
        };

        let mut req = self.client.patch(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn delete_article(&self, id: Uuid) -> Result<(), ApiError> {
        let url = format!("{}/api/internal/content/articles/{}", self.base_url, id);
        let mut req = self.client.delete(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn list_employees(
        &self,
        page: u32,
        size: u32,
    ) -> Result<PageResponse<Employee>, ApiError> {
        let url = format!(
            "{}/api/internal/hr/employees?page={}&size={}",
            self.base_url, page, size
        );
        let mut req = self.client.get(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn list_job_openings(
        &self,
        page: u32,
        size: u32,
    ) -> Result<PageResponse<JobOpening>, ApiError> {
        let url = format!(
            "{}/api/internal/hr/job-openings?page={}&size={}",
            self.base_url, page, size
        );
        let mut req = self.client.get(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn create_job_opening(
        &self,
        input: CreateJobOpeningInput,
    ) -> Result<JobOpening, ApiError> {
        let url = format!("{}/api/internal/hr/job-openings", self.base_url);
        let mut req = self.client.post(&url).json(&input);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn delete_job_opening(&self, id: Uuid) -> Result<(), ApiError> {
        let url = format!("{}/api/internal/hr/job-openings/{}", self.base_url, id);
        let mut req = self.client.delete(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn list_candidates(
        &self,
        page: u32,
        size: u32,
    ) -> Result<PageResponse<Candidate>, ApiError> {
        let url = format!(
            "{}/api/internal/recruitment/candidates?page={}&size={}",
            self.base_url, page, size
        );
        let mut req = self.client.get(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn create_candidate(&self, input: CreateCandidateInput) -> Result<Candidate, ApiError> {
        let url = format!("{}/api/internal/recruitment/candidates", self.base_url);
        let mut req = self.client.post(&url).json(&input);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn accept_candidate(&self, candidate_id: Uuid) -> Result<Employee, ApiError> {
        let url = format!(
            "{}/api/internal/hr/candidates/{}/accept",
            self.base_url, candidate_id
        );
        let mut req = self.client.post(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn delete_employee(&self, employee_id: Uuid) -> Result<(), ApiError> {
        let url = format!("{}/api/internal/hr/employees/{}", self.base_url, employee_id);
        let mut req = self.client.delete(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn delete_candidate(&self, candidate_id: Uuid) -> Result<(), ApiError> {
        let url = format!("{}/api/internal/hr/candidates/{}", self.base_url, candidate_id);
        let mut req = self.client.delete(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn list_events(&self, page: u32, size: u32) -> Result<PageResponse<Event>, ApiError> {
        let url = format!(
            "{}/api/internal/events?page={}&size={}",
            self.base_url, page, size
        );
        let mut req = self.client.get(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn create_event(&self, input: CreateEventInput) -> Result<Event, ApiError> {
        let url = format!("{}/api/internal/events", self.base_url);
        let mut req = self.client.post(&url).json(&input);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }

    async fn delete_event(&self, id: Uuid) -> Result<(), ApiError> {
        let url = format!("{}/api/internal/events/{}", self.base_url, id);
        let mut req = self.client.delete(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(Self::handle_error(resp).await)
                }
            }
            Err(e) => Err(ApiError::Network(format!(
                "No se pudo conectar con el Core: {}",
                e
            ))),
        }
    }
}
