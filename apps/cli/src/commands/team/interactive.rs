use anyhow::Result;
use inquire::{Select, Text};

use super::super::common::is_interactive;
use super::api::{fetch_teams, Team};

pub async fn select_team_interactively(prompt_message: &str) -> Result<String> {
    if !is_interactive() {
        anyhow::bail!("Interactive mode required but running in non-interactive environment");
    }

    let teams = fetch_teams().await?;
    if teams.is_empty() {
        anyhow::bail!("No teams found. Create a team first with 'spore team create'");
    }

    let team_options: Vec<String> = teams.iter().map(team_option_label).collect();

    println!("🔍 Available teams:");
    let selection = Select::new(prompt_message, team_options.clone())
        .with_vim_mode(true)
        .with_help_message("Use arrow keys or j/k to navigate, Enter to select, Esc to cancel")
        .prompt();

    match selection {
        Ok(selected_text) => {
            let selected_index = team_options
                .iter()
                .position(|opt| opt == &selected_text)
                .unwrap_or(0);
            Ok(teams[selected_index].name.clone())
        }
        Err(_) => anyhow::bail!("Team selection cancelled"),
    }
}

pub fn prompt_for_username(prompt_message: &str) -> Result<String> {
    let text_prompt = Text::new(prompt_message).with_validator(|input: &str| {
        if input.trim().is_empty() {
            Ok(inquire::validator::Validation::Invalid(
                "Username is required".into(),
            ))
        } else if !input
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            Ok(inquire::validator::Validation::Invalid(
                "Username can only contain letters, numbers, hyphens, and underscores".into(),
            ))
        } else {
            Ok(inquire::validator::Validation::Valid)
        }
    });

    Ok(text_prompt.prompt()?)
}

pub fn prompt_for_role() -> String {
    let role_options = ["member".to_string(), "admin".to_string()];
    let role_descriptions = vec![
        "member - Can view team packages and participate in team activities",
        "admin - Can manage team members and settings",
    ];

    println!();
    println!("🎭 Select role for the new team member:");
    let selection = Select::new("Select role", role_descriptions.clone())
        .with_vim_mode(true)
        .with_help_message("Use arrow keys or j/k to navigate, Enter to select")
        .prompt();

    match selection {
        Ok(selected_desc) => {
            let selected_index = role_descriptions
                .iter()
                .position(|desc| desc == &selected_desc)
                .unwrap_or(0);
            role_options[selected_index].clone()
        }
        Err(_) => {
            println!("❌ Role selection cancelled, using default role: member");
            "member".to_string()
        }
    }
}

fn team_option_label(team: &Team) -> String {
    if let Some(desc) = &team.description {
        format!("{} - {}", team.name, desc)
    } else {
        team.name.clone()
    }
}
