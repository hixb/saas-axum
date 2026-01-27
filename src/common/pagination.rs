use serde::Deserialize;

/// Pagination query parameters
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u64,

    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    20
}

impl PaginationParams {
    /// Calculate offset for database query
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.page_size
    }

    /// Get limit for database query
    pub fn limit (&self) -> u64 {
        self.page_size
    }

    /// Validate pagination parameters
    pub fn validate(&self) -> Result<(), String> {
        if self.page < 1 {
            return Err("Page must be greater than 0".to_string())
        }

        if self.page_size < 1 || self.page_size > 100 {
            return Err("Page size must be between 1 and 100".to_string())
        }

        Ok(())
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20
        }
    }
}