use anyhow::{anyhow, Result};
use regex::Regex;

use super::{validate_package_name, ValidationError};

pub fn validate_semver(version: &str) -> Result<()> {
    if version.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "version".to_string(),
            message: "Version cannot be empty".to_string(),
            example: Some("1.0.0, 2.1.3, 1.0.0-alpha.1".to_string()),
        }));
    }

    let semver_regex = Regex::new(r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$").unwrap();

    if !semver_regex.is_match(version) {
        return Err(anyhow!(ValidationError {
            field: "version".to_string(),
            message: "Invalid semantic version format".to_string(),
            example: Some("1.0.0, 2.1.3, 1.0.0-alpha.1, 1.0.0+build.1".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_version_constraint(constraint: &str) -> Result<()> {
    if constraint.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "version constraint".to_string(),
            message: "Version constraint cannot be empty".to_string(),
            example: Some("1.0.0, ^1.0.0, ~1.2.0, >=1.0.0, latest".to_string()),
        }));
    }

    if constraint == "latest" {
        return Ok(());
    }

    let constraint_regex = Regex::new(r"^([\^~>=<]*)(.+)$").unwrap();
    if let Some(captures) = constraint_regex.captures(constraint) {
        let version_part = captures.get(2).map_or("", |m| m.as_str());
        validate_semver(version_part)?;
    } else {
        return Err(anyhow!(ValidationError {
            field: "version constraint".to_string(),
            message: "Invalid version constraint format".to_string(),
            example: Some("1.0.0, ^1.0.0, ~1.2.0, >=1.0.0, latest".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_package_spec(spec: &str) -> Result<(String, Option<String>)> {
    if spec.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "package specification".to_string(),
            message: "Package specification cannot be empty".to_string(),
            example: Some("utils, utils@1.0.0, @team/utils@^1.0.0".to_string()),
        }));
    }

    if let Some(at_pos) = spec.rfind('@') {
        if at_pos == 0 {
            if let Some(second_at) = spec[1..].find('@') {
                let name = &spec[..second_at + 1];
                let version = &spec[second_at + 2..];
                validate_package_name(name)?;
                validate_version_constraint(version)?;
                Ok((name.to_string(), Some(version.to_string())))
            } else {
                validate_package_name(spec)?;
                Ok((spec.to_string(), None))
            }
        } else {
            let name = &spec[..at_pos];
            let version = &spec[at_pos + 1..];
            validate_package_name(name)?;
            validate_version_constraint(version)?;
            Ok((name.to_string(), Some(version.to_string())))
        }
    } else {
        validate_package_name(spec)?;
        Ok((spec.to_string(), None))
    }
}
