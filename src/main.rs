mod config;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;

use actix_web::{middleware::Logger, web, App, HttpServer};
use elasticsearch::{http::transport::Transport, Elasticsearch};
use futures::lock::Mutex;
use middleware::request_log::RequestLogger;
use models::AppState;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // 获取配置
    let deepseek_api_key = config::init_config();
    let transport = Transport::single_node("http://192.168.0.5:9200").unwrap();
    let client = Elasticsearch::new(transport);
    // 创建应用状态
    let app_state = web::Data::new(AppState {
        deepseek_api_key,
        es_client: Arc::new(Mutex::new(client)),
    });

    println!("Server running at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            // .wrap(Logger::default())
            .wrap(RequestLogger::new("./logs/requests.log".to_string()))
            .app_data(app_state.clone())
            .configure(routes::wechat::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
