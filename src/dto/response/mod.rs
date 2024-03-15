use serde::Serialize;

#[derive(Serialize)]
pub struct GetRegistryResponse {
    pub group_id: String,
    pub artifact_id: String,
    /// A Sha512 checksum of the group_id + artifact_id
    #[serde(alias = "ref")]
    pub ref_: String
}