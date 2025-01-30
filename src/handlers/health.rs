use crate::models::ApiResponse;
use actix_web::{get, HttpResponse};

#[get("/health")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::success("Service is healthy"))
}
