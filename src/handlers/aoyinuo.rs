use crate::constants::constant::Monitor;
use crate::models::{ApiResponse, AppState, ChatRequest, ChatResponse, Message};
use crate::services::deepseek::DeepSeekService;
use actix_web::{get, post, web, HttpResponse};
use chrono::Utc;
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
    let res = monitor.query_ipphones_by_pages(page, page_size);

    HttpResponse::Ok().json(ApiResponse::success(res))
}

#[post("/cameras")]
pub async fn cameras(req: web::Json<BasePageRequest>) -> HttpResponse {
    let page = req.page;
    let page_size = req.page_size;
    let monitor = Monitor;
    let res = monitor.query_cameras_by_pages(page, page_size);

    HttpResponse::Ok().json(ApiResponse::success(res))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BasicInfoResp {
    pub altitude: f64,
    pub well_deep: f64,
    pub safe_production_days: i64,
    pub water_supply_rescue: i32,
    pub pressure_wind: i32,
    pub emergency_evacuation: String,
}
/// 综合情况
#[get("/basic_info")]
pub async fn basic_info() -> HttpResponse {
    let monitor = Monitor;
    let res = monitor.query_security_info();
    let now = Utc::now().naive_utc();
    let duration = now - res.start_date;
    let days = duration.num_days().abs();
    let info: BasicInfoResp = BasicInfoResp {
        altitude: 4050.,
        well_deep: 800.,
        safe_production_days: days,
        water_supply_rescue: 45,
        pressure_wind: 32,
        emergency_evacuation: "已备齐物资".to_string(),
    };

    HttpResponse::Ok().json(ApiResponse::success(info))
}

// #[get("/create_security_info")]
// pub async fn create_security_info() -> HttpResponse {
//     let monitor = Monitor;
//     let res = monitor.insert_security_info();
//     HttpResponse::Ok().json(ApiResponse::success("success"))
// }

#[get("/query_security_info")]
pub async fn query_security_info() -> HttpResponse {
    let monitor = Monitor;
    let res = monitor.query_security_info();
    HttpResponse::Ok().json(ApiResponse::success(res))
}

/// 获取最新一条预警信息
#[get("/query_warning")]
pub async fn query_warning() -> HttpResponse {
    let monitor = Monitor;
    let res = monitor.query_warning();
    HttpResponse::Ok().json(ApiResponse::success(res))
}

/// 设备在线统计
#[get("/count_online_devices")]
pub async fn count_online_devices() -> HttpResponse {
    let monitor = Monitor;
    let res = monitor.count_online_devices();
    HttpResponse::Ok().json(ApiResponse::success(res))
}
