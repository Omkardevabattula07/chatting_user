use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Actix-web running"
}
#[get("/health")]
async fn health() -> impl Responder {
    "OK"
}

use actix_web::web;

#[get("/greet/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello this is from the greetuser branch, {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(health)
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
