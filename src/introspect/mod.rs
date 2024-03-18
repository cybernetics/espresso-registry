use std::{collections::HashMap, error, fs, result, vec};

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use tracing::info;

use crate::{context::DynamicAbsolutePaths, util};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub metadata: PackageMetadata,
    pub group_id: String,
    pub artifact_id: String,
    #[serde(rename="ref")]
    pub ref_: String,
}

/// Represents a package's metadata.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageMetadata {
    pub source_repository: String,
    pub versions: Vec<PackageVersion>,
}

/// Represents a specific release/version of the package.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageVersion {
    pub version: String,
    pub flags: Vec<Flags>,
    pub vulnerabilities: HashMap<String, String>,
    pub artifact_url: String,
    pub sha512sum: String,
}

/// Represents the supported package types. This will dictate how they're applied at compile time.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Flags {
    #[serde(rename = "annotation_processor")]
    AnnotationProcessor,
}

/// Parse the JSON of a package in the registry.
///
/// # Arguments
/// * `content`: The raw JSON string
///
/// # Returns
/// `result::Result`, `Ok` is a `Package` struct and `Err` is propagated errors from underlying calls.
fn parse(content: String) -> result::Result<PackageMetadata, Box<dyn error::Error>> {
    let package: PackageMetadata = serde_json::from_str(&content)?;
    Ok(package)
}

async fn get_artifact_gid_from_path(p: &String) -> String {
    let split_path: Vec<&str> = p.split("/").collect();
    split_path.get(split_path.len() - 2).unwrap().to_string()
}

async fn get_artifact_aid_from_path(p: &String) -> String{ 
    let split_path: Vec<&str> = p.split("/").collect();
    let artifact_name = split_path.get(split_path.len() - 1).unwrap().to_string();
    let clean_artifact_name = match artifact_name.strip_suffix(".json") {
        Some(v) => v,
        None => &artifact_name
    };
    clean_artifact_name.to_string()
}

/// Introspect the registry for packages
///
/// # Returns
/// `result::Result`, `Ok` is a `Vec` of `Package` structs and `Err` is propagated errors from underlying calls.
pub async fn init(
    dap: &DynamicAbsolutePaths,
) -> result::Result<Vec<Package>, Box<dyn error::Error>> {
    let paths = util::directory::read_files_recursively(dap.registry.clone()).await?;

    // iterate over the paths, parse them
    let mut packages: Vec<Package> = vec![];
    for (index, p) in paths.iter().enumerate() {
        if !p.ends_with(".json") {
            return Err(util::error::EspressoError::nib(
                format!("'{}' is not a .json file", p).as_str(),
            ));
        }

        // get our group id and artifact id
        let gid = get_artifact_gid_from_path(p).await;
        let aid = get_artifact_aid_from_path(p).await;

        // generate our ref
        let mut hasher = Sha256::new();
        hasher.update(gid.clone() + ":" + &aid);
        let ref_ = hex::encode(hasher.finalize().to_vec());

        // todo comments
        let content = String::from_utf8(fs::read(p)?)?;
        let package_metadata = parse(content)?;
        packages.push(Package {
            artifact_id: aid.to_string(),
            group_id: gid.to_string(),
            metadata: package_metadata,
            ref_
        });

        info!(
            "[{}/{}] Found '{}:{}'",
            index + 1,
            paths.len(),
            gid,
            aid
        );
    }

    Ok(packages)
}

/// Query for some packages
/// 
/// # Arguments
/// * `q`: The search term
/// * `packages`: A `Vec` containing all introspected packages
/// 
/// # Returns
/// A `Vec<Package>`, containing any packages that meet the query term.
pub fn query(q: String, packages: &Vec<Package>) -> Vec<Package> {
    let mut matches: Vec<Package> = vec![];
    let l_q = q.to_lowercase();

    // iterate over the package, find some matches
    for p in packages {
        let l_artifact_id = p.artifact_id.to_lowercase();
        let l_group_id = p.group_id.to_lowercase();

        // if artifact id contains the search term
        if l_artifact_id.contains(&l_q) || l_group_id.contains(&l_q) || p.ref_ == l_q {
            matches.push(p.clone());
        }
    } 

    matches
}