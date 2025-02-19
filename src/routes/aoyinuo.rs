use crate::handlers::aoyinuo;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1").service(aoyinuo::ipphones));
}
