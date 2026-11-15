use super::{ApiError, EditorialApi, MockApiClient};
use crate::auth::Session;
use crate::domain::{
    Article, ArticleStatus, ArticleSummary, Candidate, CaseStatus, Client,
    CreateArticleInput, CreateCandidateInput, CreateCaseInput, CreateClientInput,
    CreateEventInput, CreateJobOpeningInput, Employee, Event, JobOpening, PageResponse,
    SupportCase,
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
    fallback: MockApiClient,
}

impl HttpApiClient {
    pub fn new(base_url: String) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/json"));

        let client = HttpClient::builder()
            .timeout(Duration::from_secs(5))
            .default_headers(headers)
            .build()
            .unwrap_or_else(|_| HttpClient::new());

        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            token: Arc::new(RwLock::new(None)),
            fallback: MockApiClient::new(),
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

    async fn handle_status(status: StatusCode) -> Result<(), ApiError> {
        match status {
            StatusCode::UNAUTHORIZED => Err(ApiError::Unauthorized),
            StatusCode::FORBIDDEN => Err(ApiError::Forbidden),
            StatusCode::NOT_FOUND => Err(ApiError::NotFound),
            StatusCode::NOT_IMPLEMENTED => Err(ApiError::NotImplemented(
                "Endpoint no implementado aún en el Core (Fase 1)".to_string(),
            )),
            _ if status.is_server_error() => Err(ApiError::Network(format!(
                "Error interno del servidor: {}",
                status
            ))),
            _ if status.is_client_error() => Err(ApiError::Validation(format!(
                "Petición inválida del cliente: {}",
                status
            ))),
            _ => Ok(()),
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
                let status = response.status();
                if status.is_success() {
                    let session: Session = response
                        .json()
                        .await
                        .map_err(|e| ApiError::Serialization(e.to_string()))?;
                    if let Some(token) = session.token.as_ref() {
                        self.set_token(Some(token.value.clone())).await;
                    }
                    Ok(session)
                } else {
                    self.fallback.login(username, password).await
                }
            }
            Err(_) => self.fallback.login(username, password).await,
        }
    }

    async fn list_clients(&self, page: u32, size: u32) -> Result<PageResponse<Client>, ApiError> {
        let url = format!("{}/api/internal/attention/clients?page={}&size={}", self.base_url, page, size);
        let mut req = self.client.get(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    resp.json().await.map_err(|e| ApiError::Serialization(e.to_string()))
                } else if status == StatusCode::NOT_IMPLEMENTED {
                    self.fallback.list_clients(page, size).await
                } else {
                    Self::handle_status(status).await?;
                    Err(ApiError::Network(format!("Estado inesperado: {}", status)))
                }
            }
            Err(_) => self.fallback.list_clients(page, size).await,
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
                let status = resp.status();
                if status.is_success() {
                    resp.json().await.map_err(|e| ApiError::Serialization(e.to_string()))
                } else if status == StatusCode::NOT_IMPLEMENTED {
                    self.fallback.create_client(input).await
                } else {
                    Self::handle_status(status).await?;
                    Err(ApiError::Network(format!("Estado inesperado: {}", status)))
                }
            }
            Err(_) => self.fallback.create_client(input).await,
        }
    }

    async fn list_cases(&self, page: u32, size: u32) -> Result<PageResponse<SupportCase>, ApiError> {
        let url = format!("{}/api/internal/attention/cases?page={}&size={}", self.base_url, page, size);
        let mut req = self.client.get(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    resp.json().await.map_err(|e| ApiError::Serialization(e.to_string()))
                } else if status == StatusCode::NOT_IMPLEMENTED {
                    self.fallback.list_cases(page, size).await
                } else {
                    Self::handle_status(status).await?;
                    Err(ApiError::Network(format!("Estado inesperado: {}", status)))
                }
            }
            Err(_) => self.fallback.list_cases(page, size).await,
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
                let status = resp.status();
                if status.is_success() {
                    resp.json().await.map_err(|e| ApiError::Serialization(e.to_string()))
                } else if status == StatusCode::NOT_IMPLEMENTED {
                    self.fallback.create_case(input).await
                } else {
                    Self::handle_status(status).await?;
                    Err(ApiError::Network(format!("Estado inesperado: {}", status)))
                }
            }
            Err(_) => self.fallback.create_case(input).await,
        }
    }

    async fn update_case_status(&self, id: Uuid, status: CaseStatus) -> Result<SupportCase, ApiError> {
        let url = format!("{}/api/internal/attention/cases/{}", self.base_url, id);
        let mut req = self.client.patch(&url).json(&serde_json::json!({ "status": status.as_str() }));
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                let s = resp.status();
                if s.is_success() {
                    resp.json().await.map_err(|e| ApiError::Serialization(e.to_string()))
                } else if s == StatusCode::NOT_IMPLEMENTED || s == StatusCode::NOT_FOUND {
                    self.fallback.update_case_status(id, status).await
                } else {
                    Self::handle_status(s).await?;
                    Err(ApiError::Network(format!("Estado inesperado: {}", s)))
                }
            }
            Err(_) => self.fallback.update_case_status(id, status).await,
        }
    }

    async fn list_articles(&self, page: u32, size: u32) -> Result<PageResponse<ArticleSummary>, ApiError> {
        let url = format!("{}/api/internal/content/articles?page={}&size={}", self.base_url, page, size);
        let mut req = self.client.get(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    resp.json().await.map_err(|e| ApiError::Serialization(e.to_string()))
                } else if status == StatusCode::NOT_IMPLEMENTED {
                    self.fallback.list_articles(page, size).await
                } else {
                    Self::handle_status(status).await?;
                    Err(ApiError::Network(format!("Estado inesperado: {}", status)))
                }
            }
            Err(_) => self.fallback.list_articles(page, size).await,
        }
    }

    async fn get_article(&self, id: Uuid) -> Result<Article, ApiError> {
        let url = format!("{}/api/public/articles/{}", self.base_url, id);
        match self.client.get(&url).send().await {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    resp.json().await.map_err(|e| ApiError::Serialization(e.to_string()))
                } else {
                    self.fallback.get_article(id).await
                }
            }
            Err(_) => self.fallback.get_article(id).await,
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
                let status = resp.status();
                if status.is_success() {
                    resp.json().await.map_err(|e| ApiError::Serialization(e.to_string()))
                } else if status == StatusCode::NOT_IMPLEMENTED {
                    self.fallback.create_article(input).await
                } else {
                    Self::handle_status(status).await?;
                    Err(ApiError::Network(format!("Estado inesperado: {}", status)))
                }
            }
            Err(_) => self.fallback.create_article(input).await,
        }
    }

    async fn update_article_status(&self, id: Uuid, status: ArticleStatus) -> Result<Article, ApiError> {
        let url = if status == ArticleStatus::Published {
            format!("{}/api/internal/content/articles/{}/publish", self.base_url, id)
        } else {
            format!("{}/api/internal/content/articles/{}", self.base_url, id)
        };

        let mut req = self.client.patch(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                let s = resp.status();
                if s.is_success() {
                    resp.json().await.map_err(|e| ApiError::Serialization(e.to_string()))
                } else if s == StatusCode::NOT_IMPLEMENTED || s == StatusCode::NOT_FOUND {
                    self.fallback.update_article_status(id, status).await
                } else {
                    Self::handle_status(s).await?;
                    Err(ApiError::Network(format!("Estado inesperado: {}", s)))
                }
            }
            Err(_) => self.fallback.update_article_status(id, status).await,
        }
    }

    async fn list_employees(&self, page: u32, size: u32) -> Result<PageResponse<Employee>, ApiError> {
        let url = format!("{}/api/internal/hr/employees?page={}&size={}", self.base_url, page, size);
        let mut req = self.client.get(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    resp.json().await.map_err(|e| ApiError::Serialization(e.to_string()))
                } else if status == StatusCode::NOT_IMPLEMENTED {
                    self.fallback.list_employees(page, size).await
                } else {
                    Self::handle_status(status).await?;
                    Err(ApiError::Network(format!("Estado inesperado: {}", status)))
                }
            }
            Err(_) => self.fallback.list_employees(page, size).await,
        }
    }

    async fn list_job_openings(&self, page: u32, size: u32) -> Result<PageResponse<JobOpening>, ApiError> {
        let url = format!("{}/api/internal/hr/job-openings?page={}&size={}", self.base_url, page, size);
        let mut req = self.client.get(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    resp.json().await.map_err(|e| ApiError::Serialization(e.to_string()))
                } else if status == StatusCode::NOT_IMPLEMENTED {
                    self.fallback.list_job_openings(page, size).await
                } else {
                    Self::handle_status(status).await?;
                    Err(ApiError::Network(format!("Estado inesperado: {}", status)))
                }
            }
            Err(_) => self.fallback.list_job_openings(page, size).await,
        }
    }

    async fn create_job_opening(&self, input: CreateJobOpeningInput) -> Result<JobOpening, ApiError> {
        let url = format!("{}/api/internal/hr/job-openings", self.base_url);
        let mut req = self.client.post(&url).json(&input);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    resp.json().await.map_err(|e| ApiError::Serialization(e.to_string()))
                } else if status == StatusCode::NOT_IMPLEMENTED {
                    self.fallback.create_job_opening(input).await
                } else {
                    Self::handle_status(status).await?;
                    Err(ApiError::Network(format!("Estado inesperado: {}", status)))
                }
            }
            Err(_) => self.fallback.create_job_opening(input).await,
        }
    }

    async fn list_candidates(&self, page: u32, size: u32) -> Result<PageResponse<Candidate>, ApiError> {
        let url = format!("{}/api/internal/recruitment/candidates?page={}&size={}", self.base_url, page, size);
        let mut req = self.client.get(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    resp.json().await.map_err(|e| ApiError::Serialization(e.to_string()))
                } else if status == StatusCode::NOT_IMPLEMENTED {
                    self.fallback.list_candidates(page, size).await
                } else {
                    Self::handle_status(status).await?;
                    Err(ApiError::Network(format!("Estado inesperado: {}", status)))
                }
            }
            Err(_) => self.fallback.list_candidates(page, size).await,
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
                let status = resp.status();
                if status.is_success() {
                    resp.json().await.map_err(|e| ApiError::Serialization(e.to_string()))
                } else if status == StatusCode::NOT_IMPLEMENTED {
                    self.fallback.create_candidate(input).await
                } else {
                    Self::handle_status(status).await?;
                    Err(ApiError::Network(format!("Estado inesperado: {}", status)))
                }
            }
            Err(_) => self.fallback.create_candidate(input).await,
        }
    }

    async fn list_events(&self, page: u32, size: u32) -> Result<PageResponse<Event>, ApiError> {
        let url = format!("{}/api/internal/events?page={}&size={}", self.base_url, page, size);
        let mut req = self.client.get(&url);
        if let Some(h) = self.auth_header().await {
            req = req.header(header::AUTHORIZATION, h);
        }

        match req.send().await {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    resp.json().await.map_err(|e| ApiError::Serialization(e.to_string()))
                } else if status == StatusCode::NOT_IMPLEMENTED {
                    self.fallback.list_events(page, size).await
                } else {
                    Self::handle_status(status).await?;
                    Err(ApiError::Network(format!("Estado inesperado: {}", status)))
                }
            }
            Err(_) => self.fallback.list_events(page, size).await,
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
                let status = resp.status();
                if status.is_success() {
                    resp.json().await.map_err(|e| ApiError::Serialization(e.to_string()))
                } else if status == StatusCode::NOT_IMPLEMENTED {
                    self.fallback.create_event(input).await
                } else {
                    Self::handle_status(status).await?;
                    Err(ApiError::Network(format!("Estado inesperado: {}", status)))
                }
            }
            Err(_) => self.fallback.create_event(input).await,
        }
    }
}
