use anyhow::Result;
use std::fs;

use crate::config::PackageConfig;
use crate::utils;
use crate::validation::{
    confirm_destructive_action, sanitize_input, validate_package_name, validate_semver,
};

use super::api::{
    check_version_exists, delete_remote_package, publish_metadata, require_auth_token,
    upload_tarball, PublishPackageRequest,
};
use super::archive::create_package_tarball;
use super::versioning::validate_publish_version;

pub async fn publish_package(team: Option<&str>, description: Option<&str>) -> Result<()> {
    let token = require_auth_token()?;
    let current_dir = std::env::current_dir()?;
    let package_config_path = utils::find_yaml_file(&current_dir, "package").ok_or_else(|| {
        anyhow::anyhow!(
            "No package.yml or package.yaml found. Run this command from a package directory."
        )
    })?;

    let package_config = PackageConfig::from_file(&package_config_path)?;
    validate_publish_version(&package_config.version)?;

    if let Err(e) =
        check_version_exists(&package_config.name, &package_config.version, &token).await
    {
        println!("⚠️  Warning: Could not verify version uniqueness: {}", e);
        println!("💡 Continuing with publish attempt...");
    }

    let tarball_path = format!("{}-{}.tar.gz", package_config.name, package_config.version);
    create_package_tarball(&tarball_path)?;

    let metadata = PublishPackageRequest {
        name: package_config.name.clone(),
        version: package_config.version,
        description: description
            .map(|s| s.to_string())
            .or(package_config.description.clone()),
        team_name: team.map(|s| s.to_string()).or(package_config.team),
        tags: Some(package_config.tags.unwrap_or_default()),
    };

    let publish_result = async {
        let client = publish_metadata(&metadata, &token).await?;
        upload_tarball(&client, &metadata, &tarball_path, &token).await
    }
    .await;

    let _ = fs::remove_file(&tarball_path);
    publish_result?;

    println!(
        "📦 Successfully published {} v{}",
        metadata.name, metadata.version
    );
    if let Some(team_name) = &metadata.team_name {
        println!("   Team: {}", team_name);
    }

    Ok(())
}

pub async fn delete_package(name: &str, version: &str) -> Result<()> {
    let token = require_auth_token()?;
    let sanitized_name = sanitize_input(name);
    let sanitized_version = sanitize_input(version);
    validate_package_name(&sanitized_name)?;
    validate_semver(&sanitized_version)?;

    let package_spec = format!("{}@{}", sanitized_name, sanitized_version);
    if !confirm_destructive_action("delete package", &package_spec)? {
        println!("❌ Delete operation cancelled");
        return Ok(());
    }

    delete_remote_package(&sanitized_name, &sanitized_version, &token).await?;
    println!("🗑️  Successfully deleted {} v{}", name, version);
    Ok(())
}
