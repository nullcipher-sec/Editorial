use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    pub content: Vec<T>,
    pub page: u32,
    pub size: u32,
    #[serde(rename = "totalElements")]
    pub total_elements: u64,
    #[serde(rename = "totalPages")]
    pub total_pages: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorBody {
    pub code: i32,
    pub message: String,
}
