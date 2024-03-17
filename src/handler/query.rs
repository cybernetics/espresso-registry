use std::ops::Deref;

use actix_web::{get, web, Responder, Result};
use crate::{dto, introspect, IntrospectedPackages};

#[get("/search")]
pub async fn search_registry<'a>(q: web::Query<dto::request::GetRegistrySearchQueryParams>, introspected_packages: web::Data<IntrospectedPackages<'a>>) -> Result<impl Responder> {
    let packages = introspected_packages.packages.as_ref();
    let matches = introspect::query(q.q.clone(), packages);
    let resp = dto::response::QueryPackagesResponse {
        packages: matches
    };
    Ok(web::Json(resp))
}