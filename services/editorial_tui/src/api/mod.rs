pub mod client;
pub mod error;
pub mod mock;

pub use error::ApiError;
pub use mock::MockApiClient;
pub use client::HttpApiClient;

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
