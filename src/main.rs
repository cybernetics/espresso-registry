#![allow(unused)]

mod handler;
mod dto;
mod introspect;
mod service;
mod util;
mod context;

use std::{error, result};

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use tracing::{error, info};

async fn not_found() -> actix_web::Result<HttpResponse> {
    let resp = dto::response::generic::DefaultServiceResponse{
        msg: "A handler for the provided URL could not be found".to_string()
    };
    Ok(HttpResponse::NotFound().json(resp))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    // initialize logger
    tracing_subscriber::fmt::init();
    
    info!("Welcome to the Espresso Registry Server");

    // get contexts
    info!("Getting contexts");
    let dap = match context::get_dynamic_absolute_paths() {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to get contexts: {}", e);
            panic!();
        }
    };


    // introspect
    info!("Introspecting the registry @ '{}'", &dap.registry);
    let files = match introspect::init(&dap).await {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to introspect: {}", e);
            panic!();
        }
    };
    println!("{:?}", files);

    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .service(handler::query::search_registry)
        .default_service(web::route().to(not_found))
    })
    
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
