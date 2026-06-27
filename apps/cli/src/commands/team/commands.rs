use crate::utils::create_http_client;
use crate::validation::{sanitize_input, validate_team_name};
use anyhow::Result;
use console;

use super::super::common::{
    create_spinner, display_info, display_success, fail_progress, finish_progress, is_interactive,
    prompt_for_description_with_help, prompt_for_input_with_validation,
};
use super::api::{
    fetch_team, fetch_teams, format_api_error, get_spore_space_url, parse_api_response,
    require_auth_token, AddTeamMemberRequest, CreateTeamRequest, Team,
};
use super::interactive::{prompt_for_role, prompt_for_username, select_team_interactively};

pub async fn create_team(name: Option<&str>, description: Option<&str>) -> Result<()> {
    let token = require_auth_token()?;
    let team_name = resolve_team_name_input(name)?;
    let team_description = resolve_team_description(description);
    let request = CreateTeamRequest {
        name: team_name.clone(),
        description: team_description.clone(),
    };

    let spinner = create_spinner(&format!("Creating team '{}'...", team_name));
    let url = format!("{}/api/teams", get_spore_space_url());

    let client = create_http_client()?;
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&request)
        .send()
        .await;

    let response = match response {
        Ok(resp) => resp,
        Err(e) => {
            fail_progress(&spinner, "Failed to create team");
            return Err(anyhow::anyhow!("Network error: {}", e));
        }
    };

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        fail_progress(&spinner, "Failed to create team");
        anyhow::bail!("{}", format_api_error(status, &text));
    }

    let response_text = response.text().await?;
    let team: Team = match parse_api_response(&response_text, "create team") {
        Ok(team) => team,
        Err(err) => {
            fail_progress(&spinner, "Failed to parse server response");
            return Err(err);
        }
    };

    finish_progress(
        &spinner,
        &format!("Team '{}' created successfully", team.name),
    );
    display_success(&format!("Created team: {}", team.name));
    if let Some(desc) = team.description {
        println!("   {} {}", console::style("Description:").dim(), desc);
    }
    display_info(&format!("Team ID: {}", team.id));
    Ok(())
}

pub async fn list_teams() -> Result<()> {
    let teams = fetch_teams().await?;
    if teams.is_empty() {
        println!("No teams found.");
        return Ok(());
    }

    println!("👥 Teams:");
    for team in teams {
        println!(
            "  • {} - {}",
            team.name,
            team.description.unwrap_or("No description".to_string())
        );
    }

    Ok(())
}

pub async fn team_info(name: Option<&str>) -> Result<()> {
    let team_name = resolve_team_name_argument(
        name,
        "🔍 Select a team to view information:",
        "Select team",
        "Team name is required. Use: spore team info <name>",
    )
    .await?;

    let team = fetch_team(&team_name).await?;
    println!("👥 Team: {}", team.name);
    if let Some(desc) = team.description {
        println!("   Description: {}", desc);
    }
    println!("   Created: {}", team.created_at);
    println!("   Members:");
    for member in team.members {
        println!("     • {} ({})", member.user.username, member.role);
    }

    Ok(())
}

pub async fn add_team_member(team: Option<&str>, username: Option<&str>, role: &str) -> Result<()> {
    let token = require_auth_token()?;
    let team_name = resolve_team_name_argument(
        team,
        "➕ Adding member to team",
        "Select team to add member to",
        "Team name is required. Use: spore team add-member <team> <username>",
    )
    .await?;
    let username_str = resolve_username(username, "👤 Username to add to team")?;
    let selected_role = if is_interactive() && team.is_none() && username.is_none() {
        prompt_for_role()
    } else {
        role.to_string()
    };

    let request = AddTeamMemberRequest {
        username: username_str.clone(),
        role: selected_role.clone(),
    };

    let encoded_team = urlencoding::encode(&team_name);
    let url = format!(
        "{}/api/teams/{}/members",
        get_spore_space_url(),
        encoded_team
    );

    let client = create_http_client()?;
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!(
            "Could not add team member: {}",
            format_api_error(status, &text)
        );
    }

    println!(
        "✅ Added {} to team {} as {}",
        username_str, team_name, selected_role
    );
    Ok(())
}

pub async fn remove_team_member(team: Option<&str>, username: Option<&str>) -> Result<()> {
    let token = require_auth_token()?;
    let team_name = resolve_team_name_argument(
        team,
        "➖ Removing member from team",
        "Select team to remove member from",
        "Team name is required. Use: spore team remove-member <team> <username>",
    )
    .await?;
    let username_str = resolve_username(username, "👤 Username to remove from team")?;

    let encoded_team = urlencoding::encode(&team_name);
    let encoded_username = urlencoding::encode(&username_str);
    let url = format!(
        "{}/api/teams/{}/members/{}",
        get_spore_space_url(),
        encoded_team,
        encoded_username
    );

    let client = create_http_client()?;
    let response = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!(
            "Could not remove team member: {}",
            format_api_error(status, &text)
        );
    }

    println!("✅ Removed {} from team {}", username_str, team_name);
    Ok(())
}

fn resolve_team_name_input(name: Option<&str>) -> Result<String> {
    match name {
        Some(n) => {
            let sanitized = sanitize_input(n);
            validate_team_name(&sanitized)?;
            Ok(sanitized)
        }
        None => {
            if !is_interactive() {
                anyhow::bail!("Team name is required. Use: spore team create <name>");
            }

            println!("🎉 Creating a new team!");
            println!();
            let input = prompt_for_input_with_validation(
                "💼 Team name",
                None,
                Some("Use lowercase letters, numbers, dots, hyphens, underscores"),
                Some(|input: &str| {
                    let sanitized = sanitize_input(input);
                    match validate_team_name(&sanitized) {
                        Ok(()) => Ok(inquire::validator::Validation::Valid),
                        Err(e) => Ok(inquire::validator::Validation::Invalid(
                            format!("⚠️  {}", e).into(),
                        )),
                    }
                }),
            )?;
            Ok(sanitize_input(&input))
        }
    }
}

fn resolve_team_description(description: Option<&str>) -> Option<String> {
    match description {
        Some(d) => Some(d.to_string()),
        None if is_interactive() => {
            match prompt_for_description_with_help(
                "📝 Team description (optional)",
                Some(""),
                Some("Describe your team's purpose and responsibilities"),
            ) {
                Ok(desc) if !desc.trim().is_empty() => Some(desc),
                _ => None,
            }
        }
        None => None,
    }
}

async fn resolve_team_name_argument(
    team: Option<&str>,
    intro: &str,
    prompt: &str,
    missing_message: &str,
) -> Result<String> {
    match team {
        Some(team) => Ok(team.to_string()),
        None if is_interactive() => {
            println!("{}", intro);
            println!();
            select_team_interactively(prompt).await
        }
        None => anyhow::bail!("{}", missing_message),
    }
}

fn resolve_username(username: Option<&str>, prompt: &str) -> Result<String> {
    match username {
        Some(username) => Ok(username.to_string()),
        None if is_interactive() => prompt_for_username(prompt),
        None => anyhow::bail!(
            "Username is required. Use the team member command with a username argument"
        ),
    }
}
