use actix_web::web;
use crate::handlers;




pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(handlers::index))
	.route("/login",web::get().to(handlers::login))
	.route("/register",web::get().to(handlers::register));
}
