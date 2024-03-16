pub mod generic;

use serde::Serialize;

use crate::introspect::Package;

#[derive(Serialize)]
pub struct GetRegistryResponse {
    pub packages: Vec<Package>
}