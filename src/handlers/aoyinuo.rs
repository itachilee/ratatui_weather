use crate::constants::constant::Monitor;
use crate::models::{ApiResponse, AppState, ChatRequest, ChatResponse, Message};
use crate::services::deepseek::DeepSeekService;
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct BasePageRequest {
    pub page: i64,
    pub page_size: i64,
}

#[post("/ipphones")]
pub async fn ipphones(req: web::Json<BasePageRequest>) -> HttpResponse {
    let page = req.page;
    let page_size = req.page_size;
    let monitor = Monitor;
    let ipphones = monitor.query_ipphones_by_pages(page, page_size);

    HttpResponse::Ok().json(ApiResponse::success(ipphones))
}
