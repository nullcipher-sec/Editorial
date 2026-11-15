pub mod client;
pub mod error;
pub mod mock;

pub use client::HttpApiClient;
pub use error::ApiError;
pub use mock::MockApiClient;

use crate::auth::Session;
use crate::domain::*;
use uuid::Uuid;

pub trait EditorialApi: Send + Sync {
    fn login(
        &self,
        username: &str,
        password: &str,
    ) -> impl std::future::Future<Output = Result<Session, ApiError>> + Send;

    fn list_clients(
        &self,
        page: u32,
        size: u32,
    ) -> impl std::future::Future<Output = Result<PageResponse<Client>, ApiError>> + Send;

    fn create_client(
        &self,
        input: CreateClientInput,
    ) -> impl std::future::Future<Output = Result<Client, ApiError>> + Send;

    fn list_cases(
        &self,
        page: u32,
        size: u32,
    ) -> impl std::future::Future<Output = Result<PageResponse<SupportCase>, ApiError>> + Send;

    fn create_case(
        &self,
        input: CreateCaseInput,
    ) -> impl std::future::Future<Output = Result<SupportCase, ApiError>> + Send;

    fn update_case_status(
        &self,
        id: Uuid,
        status: CaseStatus,
    ) -> impl std::future::Future<Output = Result<SupportCase, ApiError>> + Send;

    fn list_articles(
        &self,
        page: u32,
        size: u32,
    ) -> impl std::future::Future<Output = Result<PageResponse<ArticleSummary>, ApiError>> + Send;

    fn get_article(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<Article, ApiError>> + Send;

    fn create_article(
        &self,
        input: CreateArticleInput,
    ) -> impl std::future::Future<Output = Result<Article, ApiError>> + Send;

    fn update_article_status(
        &self,
        id: Uuid,
        status: ArticleStatus,
    ) -> impl std::future::Future<Output = Result<Article, ApiError>> + Send;

    fn list_employees(
        &self,
        page: u32,
        size: u32,
    ) -> impl std::future::Future<Output = Result<PageResponse<Employee>, ApiError>> + Send;

    fn list_job_openings(
        &self,
        page: u32,
        size: u32,
    ) -> impl std::future::Future<Output = Result<PageResponse<JobOpening>, ApiError>> + Send;

    fn create_job_opening(
        &self,
        input: CreateJobOpeningInput,
    ) -> impl std::future::Future<Output = Result<JobOpening, ApiError>> + Send;

    fn list_candidates(
        &self,
        page: u32,
        size: u32,
    ) -> impl std::future::Future<Output = Result<PageResponse<Candidate>, ApiError>> + Send;

    fn create_candidate(
        &self,
        input: CreateCandidateInput,
    ) -> impl std::future::Future<Output = Result<Candidate, ApiError>> + Send;

    fn list_events(
        &self,
        page: u32,
        size: u32,
    ) -> impl std::future::Future<Output = Result<PageResponse<Event>, ApiError>> + Send;

    fn create_event(
        &self,
        input: CreateEventInput,
    ) -> impl std::future::Future<Output = Result<Event, ApiError>> + Send;
}

#[derive(Clone)]
pub enum ApiBackend {
    Http(HttpApiClient),
    Mock(MockApiClient),
}

impl EditorialApi for ApiBackend {
    async fn login(&self, username: &str, password: &str) -> Result<Session, ApiError> {
        match self {
            ApiBackend::Http(c) => c.login(username, password).await,
            ApiBackend::Mock(m) => m.login(username, password).await,
        }
    }

    async fn list_clients(&self, page: u32, size: u32) -> Result<PageResponse<Client>, ApiError> {
        match self {
            ApiBackend::Http(c) => c.list_clients(page, size).await,
            ApiBackend::Mock(m) => m.list_clients(page, size).await,
        }
    }

    async fn create_client(&self, input: CreateClientInput) -> Result<Client, ApiError> {
        match self {
            ApiBackend::Http(c) => c.create_client(input).await,
            ApiBackend::Mock(m) => m.create_client(input).await,
        }
    }

    async fn list_cases(&self, page: u32, size: u32) -> Result<PageResponse<SupportCase>, ApiError> {
        match self {
            ApiBackend::Http(c) => c.list_cases(page, size).await,
            ApiBackend::Mock(m) => m.list_cases(page, size).await,
        }
    }

    async fn create_case(&self, input: CreateCaseInput) -> Result<SupportCase, ApiError> {
        match self {
            ApiBackend::Http(c) => c.create_case(input).await,
            ApiBackend::Mock(m) => m.create_case(input).await,
        }
    }

    async fn update_case_status(&self, id: Uuid, status: CaseStatus) -> Result<SupportCase, ApiError> {
        match self {
            ApiBackend::Http(c) => c.update_case_status(id, status).await,
            ApiBackend::Mock(m) => m.update_case_status(id, status).await,
        }
    }

    async fn list_articles(&self, page: u32, size: u32) -> Result<PageResponse<ArticleSummary>, ApiError> {
        match self {
            ApiBackend::Http(c) => c.list_articles(page, size).await,
            ApiBackend::Mock(m) => m.list_articles(page, size).await,
        }
    }

    async fn get_article(&self, id: Uuid) -> Result<Article, ApiError> {
        match self {
            ApiBackend::Http(c) => c.get_article(id).await,
            ApiBackend::Mock(m) => m.get_article(id).await,
        }
    }

    async fn create_article(&self, input: CreateArticleInput) -> Result<Article, ApiError> {
        match self {
            ApiBackend::Http(c) => c.create_article(input).await,
            ApiBackend::Mock(m) => m.create_article(input).await,
        }
    }

    async fn update_article_status(&self, id: Uuid, status: ArticleStatus) -> Result<Article, ApiError> {
        match self {
            ApiBackend::Http(c) => c.update_article_status(id, status).await,
            ApiBackend::Mock(m) => m.update_article_status(id, status).await,
        }
    }

    async fn list_employees(&self, page: u32, size: u32) -> Result<PageResponse<Employee>, ApiError> {
        match self {
            ApiBackend::Http(c) => c.list_employees(page, size).await,
            ApiBackend::Mock(m) => m.list_employees(page, size).await,
        }
    }

    async fn list_job_openings(&self, page: u32, size: u32) -> Result<PageResponse<JobOpening>, ApiError> {
        match self {
            ApiBackend::Http(c) => c.list_job_openings(page, size).await,
            ApiBackend::Mock(m) => m.list_job_openings(page, size).await,
        }
    }

    async fn create_job_opening(&self, input: CreateJobOpeningInput) -> Result<JobOpening, ApiError> {
        match self {
            ApiBackend::Http(c) => c.create_job_opening(input).await,
            ApiBackend::Mock(m) => m.create_job_opening(input).await,
        }
    }

    async fn list_candidates(&self, page: u32, size: u32) -> Result<PageResponse<Candidate>, ApiError> {
        match self {
            ApiBackend::Http(c) => c.list_candidates(page, size).await,
            ApiBackend::Mock(m) => m.list_candidates(page, size).await,
        }
    }

    async fn create_candidate(&self, input: CreateCandidateInput) -> Result<Candidate, ApiError> {
        match self {
            ApiBackend::Http(c) => c.create_candidate(input).await,
            ApiBackend::Mock(m) => m.create_candidate(input).await,
        }
    }

    async fn list_events(&self, page: u32, size: u32) -> Result<PageResponse<Event>, ApiError> {
        match self {
            ApiBackend::Http(c) => c.list_events(page, size).await,
            ApiBackend::Mock(m) => m.list_events(page, size).await,
        }
    }

    async fn create_event(&self, input: CreateEventInput) -> Result<Event, ApiError> {
        match self {
            ApiBackend::Http(c) => c.create_event(input).await,
            ApiBackend::Mock(m) => m.create_event(input).await,
        }
    }
}
