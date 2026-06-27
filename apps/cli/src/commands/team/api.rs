use anyhow::Result;
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::env;

use crate::utils::create_http_client;

#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub owner: UserProfile,
    pub _count: TeamCounts,
    pub members: Vec<TeamMember>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TeamCounts {
    pub members: u32,
    pub packages: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TeamMember {
    pub id: String,
    #[serde(rename = "teamId")]
    pub team_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub role: String,
    #[serde(rename = "joinedAt")]
    pub joined_at: String,
    pub user: UserProfile,
}

#[derive(Serialize, Deserialize)]
pub struct AddTeamMemberRequest {
    pub username: String,
    pub role: String,
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

pub fn parse_api_response<T: DeserializeOwned>(response_text: &str, action: &str) -> Result<T> {
    match serde_json::from_str::<ApiResponse<T>>(response_text) {
        Ok(api_response) => {
            if api_response.success {
                api_response.data.ok_or_else(|| {
                    anyhow::anyhow!("Server response was successful but contained no data.")
                })
            } else {
                anyhow::bail!(
                    "Server reported an error: {}",
                    api_response
                        .error
                        .unwrap_or_else(|| "Unknown error".to_string())
                );
            }
        }
        Err(_) => {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(response_text) {
                if let Some(error_message) = json.get("error").and_then(|v| v.as_str()) {
                    anyhow::bail!("Failed to {}: {}", action, error_message);
                }
                if let Some(message) = json.get("message").and_then(|v| v.as_str()) {
                    anyhow::bail!("Failed to {}: {}", action, message);
                }
                anyhow::bail!(
                    "Failed to parse the server response. The server sent an unexpected JSON object:\n{}",
                    response_text
                );
            }

            anyhow::bail!(
                "Failed to parse the server response. The server sent the following unexpected response:\n{}",
                response_text
            );
        }
    }
}

pub async fn fetch_teams() -> Result<Vec<Team>> {
    let url = format!("{}/api/teams", get_spore_space_url());
    let client = create_http_client()?;
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to fetch teams list");
    }

    let response_text = response.text().await?;
    parse_api_response(&response_text, "list teams")
}

pub async fn fetch_team(team_name: &str) -> Result<Team> {
    let url = format!("{}/api/teams/{}", get_spore_space_url(), team_name);
    let client = create_http_client()?;
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!(
            "Could not retrieve team information: {}",
            format_api_error(status, &text)
        );
    }

    let response_text = response.text().await?;
    parse_api_response(&response_text, "get team info")
}
