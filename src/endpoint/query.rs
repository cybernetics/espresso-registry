use actix_web::{get, Responder};

#[get("/q")]
pub fn query(term: String) -> impl Responder {
    
}