pub mod generic;

use serde::Serialize;

use crate::introspect::Package;

#[derive(Serialize)]
pub struct QueryPackagesResponse {
    pub packages: Vec<Package>
}