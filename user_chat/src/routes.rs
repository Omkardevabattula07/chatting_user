use actix_web::web;
use crate::handlers;




pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/").route(web::get().to(handlers::index))
    );
}