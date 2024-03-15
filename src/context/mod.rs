use std::{env, error, result};

pub struct DynamicAbsolutePaths {
    pub registry: String
}

pub fn get_dynamic_absolute_paths() -> result::Result<DynamicAbsolutePaths, Box<dyn error::Error>> {
    let cwd = env::current_dir()?;
    let mut cwd_string: String = cwd.to_string_lossy().into_owned();
    Ok(
        DynamicAbsolutePaths {
        registry: (cwd_string.clone() + "/registry").to_string()
        }
    )
}