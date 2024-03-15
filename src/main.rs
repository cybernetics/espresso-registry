#![allow(unused)]

mod handler;
mod dto;
mod introspect;
mod service;
mod util;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

async fn not_found() -> actix_web::Result<HttpResponse> {
    let resp = dto::response::generic::DefaultServiceResponse{
        msg: "A handler for the provided URL could not be found".to_string()
    };
    Ok(HttpResponse::NotFound().json(resp))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let log = util::log::create_logger();
    slog::info!(log, "Starting the Espresso Registry Server");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(log.clone()))
        .service(handler::query::search_registry)
        .default_service(web::route().to(not_found))
    })
    
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
