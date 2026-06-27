use anyhow::{anyhow, Result};
use std::path::{Component, Path};

use super::ValidationError;

pub fn validate_path(path: &str, field_name: &str, must_exist: bool) -> Result<()> {
    if path.is_empty() {
        return Err(anyhow!(ValidationError {
            field: field_name.to_string(),
            message: format!("{} cannot be empty", field_name),
            example: Some("./my-project, /home/user/projects".to_string()),
        }));
    }

    if path.contains('\0') {
        return Err(anyhow!(ValidationError {
            field: field_name.to_string(),
            message: format!("{} contains invalid null character", field_name),
            example: None,
        }));
    }

    let path_obj = Path::new(path);

    if path.len() > 4096 {
        return Err(anyhow!(ValidationError {
            field: field_name.to_string(),
            message: format!("{} is too long (maximum 4096 characters)", field_name),
            example: None,
        }));
    }

    for component in path_obj.components() {
        if let Component::Normal(os_str) = component {
            if let Some(name) = os_str.to_str() {
                if name.starts_with('.')
                    && (name == ".." || name.contains("../") || name.contains("..\\"))
                {
                    return Err(anyhow!(ValidationError {
                        field: field_name.to_string(),
                        message: format!("{} contains unsafe path traversal patterns", field_name),
                        example: Some("./my-project, /home/user/projects".to_string()),
                    }));
                }
            }
        }
    }

    if must_exist && !path_obj.exists() {
        return Err(anyhow!(ValidationError {
            field: field_name.to_string(),
            message: format!("{} does not exist", field_name),
            example: None,
        }));
    }

    Ok(())
}
