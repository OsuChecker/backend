use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaginationView {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "type")]
    pub view_type: String,
    pub first: String,
    pub last: String,
    pub previous: Option<String>,
    pub next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaginationParams {
    #[validate(range(min = 1))]
    pub page: Option<i64>,
    #[validate(range(min = 1, max = 50))]
    pub per_page: Option<i64>,
}

impl PaginationParams {
    pub fn get_page(&self) -> i64 {
        self.page.unwrap_or(1)
    }

    pub fn get_per_page(&self) -> i64 {
        self.per_page.unwrap_or(20).min(50)
    }
}

impl PaginationView {
    pub fn new(base_url: &str, page: i64, per_page: i64, total_pages: i64) -> Self {
        let first = format!("{}?page=1&per_page={}", base_url, per_page);
        let last = format!("{}?page={}&per_page={}", base_url, total_pages, per_page);
        let previous = if page > 1 {
            Some(format!("{}?page={}&per_page={}", base_url, page - 1, per_page))
        } else {
            None
        };
        let next = if page < total_pages {
            Some(format!("{}?page={}&per_page={}", base_url, page + 1, per_page))
        } else {
            None
        };

        Self {
            id: format!("{}?page={}&per_page={}", base_url, page, per_page),
            view_type: "PartialCollectionView".to_string(),
            first,
            last,
            previous,
            next,
        }
    }
} 