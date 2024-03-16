use std::ops::Deref;

use actix_web::{get, web, Responder, Result};
use crate::{dto, introspect, IntrospectedPackages};

#[get("/search")]
pub async fn search_registry(q: web::Query<dto::request::GetRegistrySearchQueryParams>, introspected_packages: web::Data<IntrospectedPackages>) -> Result<impl Responder> {
    let packages: std::sync::MutexGuard<'_, Vec<crate::introspect::Package>> = introspected_packages.packages.lock().expect("failed to lock packages, this is indicative of something very wrong");
    let matches = introspect::query(q.q.clone(), packages.clone());
    let resp = dto::response::GetRegistryResponse {
        packages: matches
    };
    Ok(web::Json(resp))
}