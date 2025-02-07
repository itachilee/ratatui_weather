use elasticsearch::Elasticsearch;
use futures::lock::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug)]
pub struct AppState {
    pub deepseek_api_key: String,
    pub es_client: Arc<Mutex<Elasticsearch>>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}
