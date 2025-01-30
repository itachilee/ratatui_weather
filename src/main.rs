mod config;
mod handlers;
mod models;
mod routes;
mod services;

use actix_web::{middleware::Logger, web, App, HttpServer};
use models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // 获取配置
    let deepseek_api_key = config::init_config();

    // 创建应用状态
    let app_state = web::Data::new(AppState { deepseek_api_key });

    println!("Server running at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
