mod endpoint;
mod dto;
mod introspect;
mod service;
mod util;

use actix_web::{get, web, App, HttpServer, Responder};
use slog::{info, o, Drain};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello, {}", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let root = slog::Logger::root(drain, o!());
    info!(root, "espresso registry starting up...");

    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
