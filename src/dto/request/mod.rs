use serde::Deserialize;

/// Represents the query params for the `GET /search?q=` endpoint
#[derive(Deserialize)]
pub struct GetRegistrySearchQueryParams {
    pub q: String
}