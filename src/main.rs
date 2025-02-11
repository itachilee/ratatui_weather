mod config;
mod db;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;

use actix_web::{middleware::Logger, web, App, HttpServer};

use middleware::request_log::RequestLogger;
use models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // 创建应用状态
    let app_state = web::Data::new(AppState::new());

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
