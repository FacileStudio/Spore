use anyhow::{anyhow, Result};
use regex::Regex;

use super::ValidationError;

pub fn validate_package_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "package name".to_string(),
            message: "Package name cannot be empty".to_string(),
            example: Some("my-package, utils, @team/utils".to_string()),
        }));
    }

    if name.len() > 214 {
        return Err(anyhow!(ValidationError {
            field: "package name".to_string(),
            message: "Package name cannot exceed 214 characters".to_string(),
            example: None,
        }));
    }

    if name.starts_with('.') || name.starts_with('_') {
        return Err(anyhow!(ValidationError {
            field: "package name".to_string(),
            message: "Package name cannot start with '.' or '_'".to_string(),
            example: Some("my-package, utils".to_string()),
        }));
    }

    if name.contains("..") {
        return Err(anyhow!(ValidationError {
            field: "package name".to_string(),
            message: "Package name cannot contain consecutive dots".to_string(),
            example: Some("my-package, utils".to_string()),
        }));
    }

    let scoped_regex = Regex::new(
        r"^@[a-zA-Z0-9]([a-zA-Z0-9._-]*[a-zA-Z0-9])?/[a-zA-Z0-9]([a-zA-Z0-9._-]*[a-zA-Z0-9])?$",
    )
    .unwrap();
    let unscoped_regex = Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9._-]*[a-zA-Z0-9])?$").unwrap();

    if name.starts_with('@') {
        if !scoped_regex.is_match(name) {
            return Err(anyhow!(ValidationError {
                field: "scoped package name".to_string(),
                message: "Invalid scoped package format. Must be @scope/name with alphanumeric characters, dots, hyphens, or underscores".to_string(),
                example: Some("@team/utils, @my-org/ui-components".to_string()),
            }));
        }
    } else if !unscoped_regex.is_match(name) {
        return Err(anyhow!(ValidationError {
            field: "package name".to_string(),
            message: "Invalid package name format. Must contain only alphanumeric characters, dots, hyphens, or underscores".to_string(),
            example: Some("my-package, utils, ui-components".to_string()),
        }));
    }

    let reserved_names = [
        "node_modules",
        "favicon.ico",
        ".git",
        ".gitignore",
        ".npmignore",
        "package.json",
        "package-lock.json",
        "spore.yml",
        "package.yml",
        "app.yml",
    ];

    if reserved_names.contains(&name) {
        return Err(anyhow!(ValidationError {
            field: "package name".to_string(),
            message: format!("'{}' is a reserved name", name),
            example: Some("my-package, utils".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_app_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "app name".to_string(),
            message: "App name cannot be empty".to_string(),
            example: Some("my-app, web, mobile".to_string()),
        }));
    }

    if name.len() > 100 {
        return Err(anyhow!(ValidationError {
            field: "app name".to_string(),
            message: "App name cannot exceed 100 characters".to_string(),
            example: None,
        }));
    }

    let valid_regex = Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9._-]*[a-zA-Z0-9])?$").unwrap();
    if !valid_regex.is_match(name) {
        return Err(anyhow!(ValidationError {
            field: "app name".to_string(),
            message: "App name must start and end with alphanumeric characters and contain only letters, numbers, dots, hyphens, or underscores".to_string(),
            example: Some("my-app, web-dashboard, mobile_app".to_string()),
        }));
    }

    if name.starts_with('.') || name.starts_with('_') || name.starts_with('-') {
        return Err(anyhow!(ValidationError {
            field: "app name".to_string(),
            message: "App name cannot start with '.', '_', or '-'".to_string(),
            example: Some("my-app, web, mobile".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_project_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "project name".to_string(),
            message: "Project name cannot be empty".to_string(),
            example: Some("my-project, monorepo".to_string()),
        }));
    }

    if name.len() > 100 {
        return Err(anyhow!(ValidationError {
            field: "project name".to_string(),
            message: "Project name cannot exceed 100 characters".to_string(),
            example: None,
        }));
    }

    let valid_regex = Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9._\s-]*[a-zA-Z0-9])?$").unwrap();
    if !valid_regex.is_match(name) {
        return Err(anyhow!(ValidationError {
            field: "project name".to_string(),
            message: "Project name must start and end with alphanumeric characters and contain only letters, numbers, spaces, dots, hyphens, or underscores".to_string(),
            example: Some("my-project, Web Dashboard, mobile_app".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_team_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "team name".to_string(),
            message: "Team name cannot be empty".to_string(),
            example: Some("my-team, frontend, devops".to_string()),
        }));
    }

    if name.len() > 50 {
        return Err(anyhow!(ValidationError {
            field: "team name".to_string(),
            message: "Team name cannot exceed 50 characters".to_string(),
            example: None,
        }));
    }

    let valid_regex = Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9._-]*[a-zA-Z0-9])?$").unwrap();
    if !valid_regex.is_match(name) {
        return Err(anyhow!(ValidationError {
            field: "team name".to_string(),
            message: "Team name must start and end with alphanumeric characters and contain only letters, numbers, dots, hyphens, or underscores".to_string(),
            example: Some("my-team, frontend-team, dev_ops".to_string()),
        }));
    }

    let reserved_names = ["admin", "system", "root", "api", "www", "spore"];
    if reserved_names.contains(&name.to_lowercase().as_str()) {
        return Err(anyhow!(ValidationError {
            field: "team name".to_string(),
            message: format!("'{}' is a reserved team name", name),
            example: Some("my-team, frontend, devops".to_string()),
        }));
    }

    Ok(())
}

pub fn validate_template_name(template: &str) -> Result<()> {
    if template.is_empty() {
        return Err(anyhow!(ValidationError {
            field: "template".to_string(),
            message: "Template name cannot be empty".to_string(),
            example: Some("@svelte-starter, @team/template".to_string()),
        }));
    }

    if !template.starts_with('@') {
        return Err(anyhow!(ValidationError {
            field: "template".to_string(),
            message: "Template name must start with '@' to indicate it's from Spore Space"
                .to_string(),
            example: Some("@svelte-starter, @team/template".to_string()),
        }));
    }

    validate_package_name(template)
}
