use anyhow::{anyhow, Result};
use inquire::Confirm;
use regex::Regex;
#[cfg(test)]
use url::Url;

use super::ValidationError;

#[cfg(test)]
pub fn validate_url(url: &str, field_name: &str) -> Result<()> {
    if url.is_empty() {
        return Err(anyhow!(ValidationError {
            field: field_name.to_string(),
            message: format!("{} cannot be empty", field_name),
            example: Some("https://example.com, https://github.com/user/repo".to_string()),
        }));
    }

    match Url::parse(url) {
        Ok(parsed_url) => match parsed_url.scheme() {
            "http" | "https" => Ok(()),
            _ => Err(anyhow!(ValidationError {
                field: field_name.to_string(),
                message: format!("{} must use http or https protocol", field_name),
                example: Some("https://example.com, https://github.com/user/repo".to_string()),
            })),
        },
        Err(_) => Err(anyhow!(ValidationError {
            field: field_name.to_string(),
            message: format!("Invalid {} format", field_name),
            example: Some("https://example.com, https://github.com/user/repo".to_string()),
        })),
    }
}

pub fn validate_script_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "script name".to_string(),
            message: "Script name cannot be empty".to_string(),
            example: Some("dev, build, test, start".to_string()),
        }));
    }

    if name.len() > 50 {
        return Err(anyhow!(ValidationError {
            field: "script name".to_string(),
            message: "Script name cannot exceed 50 characters".to_string(),
            example: None,
        }));
    }

    let valid_regex = Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9._:-]*[a-zA-Z0-9])?$").unwrap();
    if !valid_regex.is_match(name) {
        return Err(anyhow!(ValidationError {
            field: "script name".to_string(),
            message: "Script name must start and end with alphanumeric characters and contain only letters, numbers, dots, hyphens, underscores, or colons".to_string(),
            example: Some("dev, build, test:unit, e2e:ci".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_prerelease_id(preid: &str) -> Result<()> {
    if preid.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "prerelease identifier".to_string(),
            message: "Prerelease identifier cannot be empty".to_string(),
            example: Some("alpha, beta, rc".to_string()),
        }));
    }

    let valid_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
    if !valid_regex.is_match(preid) {
        return Err(anyhow!(ValidationError {
            field: "prerelease identifier".to_string(),
            message: "Prerelease identifier can only contain alphanumeric characters".to_string(),
            example: Some("alpha, beta, rc, dev".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_description(description: &str) -> Result<()> {
    if description.len() > 500 {
        return Err(anyhow!(ValidationError {
            field: "description".to_string(),
            message: "Description cannot exceed 500 characters".to_string(),
            example: None,
        }));
    }

    if description.trim() != description {
        return Err(anyhow!(ValidationError {
            field: "description".to_string(),
            message: "Description cannot start or end with whitespace".to_string(),
            example: Some("A useful package for utilities".to_string()),
        }));
    }

    Ok(())
}

pub fn sanitize_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_control() || *c == '\t' || *c == '\n')
        .collect::<String>()
        .trim()
        .to_string()
}

pub fn confirm_destructive_action(action: &str, target: &str) -> Result<bool> {
    println!("⚠️  WARNING: This is a destructive action!");
    println!("   Action: {}", action);
    println!("   Target: {}", target);
    println!("   This action cannot be undone.");
    println!();

    Ok(Confirm::new(&format!(
        "Are you sure you want to {} '{}'?",
        action, target
    ))
    .with_default(false)
    .prompt()?)
}
