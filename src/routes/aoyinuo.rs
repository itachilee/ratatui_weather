use crate::handlers::{aoyinuo, chat, health, post};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(health::health_check)
            .service(chat::chat)
            .service(post::query)
            .service(post::create_post)
            .service(post::update_post)
            .service(post::delete_post)
            .service(aoyinuo::ipphones),
    );
}
