use crate::config;
use chrono::{NaiveDateTime, Utc};
use elasticsearch::{http::transport::Transport, Elasticsearch};
use futures::lock::Mutex;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug)]
pub struct AppState {
    pub deepseek_api_key: String,
    pub es_client: Arc<Mutex<Elasticsearch>>,
    // 这里使用连接池
    pub pg_db: Arc<PgPool>,
}
impl AppState {
    pub fn new() -> Self {
        let app_state = Self {
            deepseek_api_key: config::init_config(),
            es_client: Arc::new(Mutex::new(establish_esconnection())),
            pg_db: Arc::new(establish_pgconnection()),
        };
        app_state
    }
}

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;
pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub fn establish_pgconnection() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub fn establish_esconnection() -> Elasticsearch {
    dotenv().ok();
    let es_url = env::var("ELASTICSEARCH_HOST").expect("ELASTICSEARCH_HOST must be set");
    let transport = Transport::single_node("http://192.168.0.5:9200").unwrap();
    let client = Elasticsearch::new(transport);
    client
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub extras: Option<String>,
    pub message: String,
    pub success: bool,
    pub result: Option<T>,
    pub time: NaiveDateTime,
    #[serde(rename = "type")]
    type_: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            extras: None,
            success: true,
            result: Some(data),
            time: Utc::now().naive_local(),
            message: "".to_string(),
            type_: "success".into(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            code: 200,
            extras: None,
            success: false,
            result: None,
            time: Utc::now().naive_local(),
            message: message.into(),
            type_: "error".into(),
        }
    }
}

#[derive(Serialize)]
pub struct PaginatedResult<T> {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub total_pages: i64,
    pub items: Vec<T>,
    pub has_prev_page: bool,
    pub has_next_page: bool,
}
