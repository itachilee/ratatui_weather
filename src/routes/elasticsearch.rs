use crate::handlers::{chat, elasticsearch, health};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(elasticsearch::create_document)
            .service(elasticsearch::insert_document)
            .service(elasticsearch::search_document)
            .service(elasticsearch::update_document),
    );
}
