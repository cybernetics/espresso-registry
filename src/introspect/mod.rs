use std::{error, fs, result};

use serde::{Deserialize, Serialize};

use crate::{context::DynamicAbsolutePaths, util};


/// Represents a package.
#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub group_id: String,
    pub artifact_id: String,
    pub ref_: String,
    pub source_repository: String,
    pub versions: Vec<String>,
}

/// Represents the supported package types. This will dictate how they're applied at compile time.
#[derive(Serialize, Deserialize, Debug)]
pub enum Flags {
    AnnotationProcessor,
}

/// Represents a specific release/version of the package.
#[derive(Serialize, Deserialize, Debug)]
pub struct PackageVersion {
    pub version: String,
    pub types: Vec<Flags>
}

/// Parse the JSON of a package in the registry.
/// 
/// # Arguments
/// * `content`: The raw JSON string
/// 
/// # Returns
/// `result::Result`, `Ok` is a `Package` struct and `Err` is propagated errors from underlying calls.
fn parse(content: String) -> result::Result<Package, Box<dyn error::Error>> {
    let package: Package = serde_json::from_str(&content)?;
    Ok(package)
}

/// Introspect the registry for packages
/// 
/// # Returns
/// `result::Result`, `Ok` is a `Vec` of `Package` structs and `Err` is propagated errors from underlying calls.
pub async fn init(dap: &DynamicAbsolutePaths) -> result::Result<Vec<Package>, Box<dyn error::Error>> {
    let paths = util::directory::read_files_recursively(dap.registry.clone()).await?;

    // iterate over the paths, parse them
    let mut packages: Vec<Package> = vec![];
    for p in paths {
        let content = String::from_utf8(fs::read(p)?)?;
        packages.push(parse(content)?);
    }

    Ok(packages)

}