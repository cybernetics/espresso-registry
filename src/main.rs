#![allow(unused)]

mod handler;
mod dto;
mod introspect;
mod service;
mod util;
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello, {}", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let log = util::log::create_logger();
    slog::info!(log, "Starting the Espresso Registry Server");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(log.clone()))
        .service(handler::query::search_registry)
    })
    
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
