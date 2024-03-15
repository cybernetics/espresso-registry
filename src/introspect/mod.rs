/// Represents a package.
pub struct Package {
    pub group_id: String,
    pub artifact_id: String,
    pub source_repository: String,
    pub versions: Vec<String>,
}

/// Represents the supported package types. This will dictate how they're applied at compile time.
pub enum Flags {
    annotation_processor,
}

/// Represents a specific release/version of the package.
pub struct PackageVersion {
    pub version: String,
    pub types: Vec<Flags>
}