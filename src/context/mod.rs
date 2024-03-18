use std::{env, error, result};

use tracing_subscriber::registry;

use crate::util::error::EspressoError;

pub struct DynamicAbsolutePaths {
    pub registry: String,
}

/// Get the absolute path to the registry
///
/// # Arguments
/// * `cwd`: Reference to a `String` containing the current working directory
///
/// # Returns
/// Propagated errors, where `Ok` is a `String` containing the absolute path to the registry
fn get_registry_path(cwd: &String) -> result::Result<String, Box<dyn error::Error>> {
    let mut registry_path = env::var("ESPRESSO_REGISTRY_PATH")?;
    if registry_path.is_empty() {
        registry_path = cwd.clone() + "/registry";
    }
    Ok(registry_path)
}

/// Get a `DynamicAbsolutePaths` struct
///
/// # Returns
/// Propagated errors, where `Ok` is a `DyanmicAbsolutePaths` struct
pub fn get_dynamic_absolute_paths() -> result::Result<DynamicAbsolutePaths, Box<dyn error::Error>> {
    let cwd = env::current_dir()?.to_string_lossy().into_owned();
    Ok(DynamicAbsolutePaths {
        registry: get_registry_path(&cwd)?,
    })
}
