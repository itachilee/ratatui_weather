use actix_web::web;

pub mod aoyinuo;
pub mod elasticsearch;

pub fn config(cfg: &mut web::ServiceConfig) {
    aoyinuo::config(cfg);
}
