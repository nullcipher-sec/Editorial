use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ArticleStatus {
    Draft,
    Review,
    Published,
}

impl ArticleStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ArticleStatus::Draft => "DRAFT",
            ArticleStatus::Review => "REVIEW",
            ArticleStatus::Published => "PUBLISHED",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ArticleStatus::Draft => "Borrador",
            ArticleStatus::Review => "En Revisión",
            ArticleStatus::Published => "Publicado",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleSummary {
    pub id: Uuid,
    pub title: String,
    pub status: ArticleStatus,
    #[serde(rename = "publishedAt")]
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub summary: Option<String>,
    pub body: String,
    pub status: ArticleStatus,
    #[serde(rename = "authorId")]
    pub author_id: Option<Uuid>,
    #[serde(rename = "publishedAt")]
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateArticleInput {
    #[validate(length(min = 5, max = 255, message = "El título debe tener entre 5 y 255 caracteres"))]
    pub title: String,
    #[validate(length(max = 500, message = "El resumen no puede exceder 500 caracteres"))]
    pub summary: Option<String>,
    #[validate(length(min = 20, message = "El cuerpo debe tener al menos 20 caracteres"))]
    pub body: String,
}
