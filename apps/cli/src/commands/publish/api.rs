use anyhow::Result;
use reqwest::{multipart, StatusCode};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

use crate::utils::create_http_client;

#[derive(Serialize, Deserialize)]
pub struct PublishPackageRequest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(rename = "teamName")]
    pub team_name: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub fn get_spore_space_url() -> String {
    env::var("SPORE_SPACE_URL")
        .unwrap_or_else(|_| "https://spore-space-production.up.railway.app".to_string())
}

fn get_auth_token() -> Result<Option<String>> {
    match env::var("SPORE_TOKEN") {
        Ok(token) if !token.trim().is_empty() => Ok(Some(token)),
        _ => Ok(None),
    }
}

pub fn require_auth_token() -> Result<String> {
    get_auth_token()?.ok_or_else(|| {
        anyhow::anyhow!("Authentication required. Set SPORE_TOKEN environment variable.")
    })
}

pub fn format_api_error(status: StatusCode, response_text: &str) -> String {
    let error_message =
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(response_text) {
            json_value
                .get("error")
                .or_else(|| json_value.get("message"))
                .and_then(|v| v.as_str())
                .unwrap_or(response_text)
                .to_string()
        } else {
            response_text.to_string()
        };

    match status.as_u16() {
        400 => format!("❌ {}", error_message),
        401 => format!("🔐 Authentication failed: {}", error_message),
        403 => format!("🚫 Permission denied: {}", error_message),
        404 => format!("🔍 Not found: {}", error_message),
        409 => format!("⚠️  Conflict: {}", error_message),
        500 => format!("💥 Server error: {}", error_message),
        502..=504 => format!("🌐 Service temporarily unavailable: {}", error_message),
        _ => format!("❌ Error ({}): {}", status.as_u16(), error_message),
    }
}

pub async fn check_version_exists(package_name: &str, version: &str, token: &str) -> Result<()> {
    let base_url = get_spore_space_url();
    let url = format!(
        "{}/api/packages/{}/{}/exists",
        base_url, package_name, version
    );

    let client = create_http_client()?;
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    match response.status().as_u16() {
        200 => anyhow::bail!(
            "Version '{}' of package '{}' already exists. \n💡 Use 'spore version bump major|minor|patch' to create a new version.",
            version,
            package_name
        ),
        404 => {
            println!("✅ Version {} is available for publishing", version);
            Ok(())
        }
        _ => anyhow::bail!("Could not verify version existence (server error)"),
    }
}

pub async fn publish_metadata(
    metadata: &PublishPackageRequest,
    token: &str,
) -> Result<reqwest::Client> {
    let base_url = get_spore_space_url();
    let client = create_http_client()?;
    let metadata_url = format!("{}/api/packages", base_url);
    let metadata_response = client
        .post(&metadata_url)
        .header("Authorization", format!("Bearer {}", token))
        .json(metadata)
        .send()
        .await?;

    if !metadata_response.status().is_success() {
        let status = metadata_response.status();
        let text = metadata_response.text().await.unwrap_or_default();
        anyhow::bail!(
            "Failed to publish package: {}",
            format_api_error(status, &text)
        );
    }

    Ok(client)
}

pub async fn upload_tarball(
    client: &reqwest::Client,
    metadata: &PublishPackageRequest,
    tarball_path: &str,
    token: &str,
) -> Result<()> {
    let base_url = get_spore_space_url();
    let upload_url = format!("{}/api/packages/upload", base_url);
    let file_content = fs::read(tarball_path)?;
    let file_part = multipart::Part::bytes(file_content)
        .file_name(tarball_path.to_string())
        .mime_str("application/gzip")?;

    let form = multipart::Form::new()
        .part("file", file_part)
        .text("packageName", metadata.name.clone())
        .text("version", metadata.version.clone());

    let upload_response = client
        .post(&upload_url)
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send()
        .await?;

    if !upload_response.status().is_success() {
        let status = upload_response.status();
        let text = upload_response.text().await.unwrap_or_default();
        anyhow::bail!(
            "Failed to upload package files: {}",
            format_api_error(status, &text)
        );
    }

    Ok(())
}

pub async fn delete_remote_package(name: &str, version: &str, token: &str) -> Result<()> {
    let base_url = get_spore_space_url();
    let url = format!("{}/api/packages/{}/{}", base_url, name, version);

    let client = create_http_client()?;
    let response = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    if response.status().is_success() {
        return Ok(());
    }

    let status = response.status();
    let text = response.text().await.unwrap_or_default();
    anyhow::bail!(
        "Failed to delete package: {}",
        format_api_error(status, &text)
    );
}
