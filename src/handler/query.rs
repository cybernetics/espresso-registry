use actix_web::{get, web, Responder, Result};
use crate::dto;

#[get("/search")]
pub async fn search_registry(_term: web::Query<dto::request::GetRegistrySearchQueryParams>) -> Result<impl Responder> {
    let resp = dto::response::GetRegistryResponse {
        group_id: "org.projectlombok".to_string(),
        artifact_id: "lombok".to_string(),
        ref_: "asujdhiu12hu31897d89gf8934yhijsjndlaicjASDIOu897892137uhq".to_string()
    };
    Ok(web::Json(resp))
}