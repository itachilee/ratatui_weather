use crate::handlers::{chat, health, wechat};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(health::health_check)
            .service(chat::chat), // .service(wechat::verify_server)
                                  // .service(wechat::handle_message),
    );
}
