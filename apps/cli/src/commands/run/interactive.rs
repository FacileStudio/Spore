use anyhow::Result;
use console::style;
use inquire::Select;
use std::io::IsTerminal;

use crate::commands::common::{display_error, display_info};
use crate::config::{AppConfig, PackageConfig};
use crate::utils;

use super::execute::execute_script;
use super::lookup::{get_project_app_config, get_project_package_config, load_project_context};

fn is_interactive() -> bool {
    std::io::stdin().is_terminal()
}

pub(super) async fn run_script_interactive() -> Result<()> {
    let current_dir = std::env::current_dir()?;

    if !is_interactive() {
        display_error("Interactive mode requires a terminal");
        display_info("Use 'spore run <script_name>' to run a specific script");
        anyhow::bail!("Interactive mode requires a terminal");
    }

    let mut all_scripts = Vec::new();
    let (project, project_error) = load_project_context(&current_dir);

    if let Some(app_config_path) = utils::find_yaml_file(&current_dir, "app") {
        if let Some((app_name, app_config)) = project
            .as_ref()
            .and_then(|project| get_project_app_config(project, &app_config_path))
        {
            if let Some(scripts) = &app_config.scripts {
                for (name, command) in scripts {
                    all_scripts.push((
                        format!("{} {}", name, style(format!("({})", app_name)).dim()),
                        name.clone(),
                        command.clone(),
                        "app".to_string(),
                    ));
                }
            }
        } else if let Ok(app_config) = AppConfig::from_file(&app_config_path) {
            if let Some(scripts) = &app_config.scripts {
                for (name, command) in scripts {
                    all_scripts.push((
                        format!("{} {}", name, style(format!("({})", app_config.name)).dim()),
                        name.clone(),
                        command.clone(),
                        "app".to_string(),
                    ));
                }
            }
        }
    }

    if let Some(package_config_path) = utils::find_yaml_file(&current_dir, "package") {
        if let Some((package_name, package_config)) = project
            .as_ref()
            .and_then(|project| get_project_package_config(project, &package_config_path))
        {
            if let Some(scripts) = &package_config.scripts {
                for (name, command) in scripts {
                    all_scripts.push((
                        format!("{} {}", name, style(format!("({})", package_name)).dim()),
                        name.clone(),
                        command.clone(),
                        "package".to_string(),
                    ));
                }
            }
        } else if let Ok(package_config) = PackageConfig::from_file(&package_config_path) {
            if let Some(scripts) = &package_config.scripts {
                for (name, command) in scripts {
                    all_scripts.push((
                        format!(
                            "{} {}",
                            name,
                            style(format!("({})", package_config.name)).dim()
                        ),
                        name.clone(),
                        command.clone(),
                        "package".to_string(),
                    ));
                }
            }
        }
    }

    if let Some(project) = &project {
        if let Some(scripts) = &project.config.scripts {
            for (name, command) in scripts {
                all_scripts.push((
                    format!(
                        "{} {}",
                        name,
                        style(format!("({})", project.config.name)).dim()
                    ),
                    name.clone(),
                    command.clone(),
                    "project".to_string(),
                ));
            }
        }
    }

    if all_scripts.is_empty() {
        let has_app_config = utils::find_yaml_file(&current_dir, "app").is_some();
        let has_package_config = utils::find_yaml_file(&current_dir, "package").is_some();
        let has_project = project.is_some();

        if !has_app_config && !has_package_config && !has_project {
            if let Some(error) = project_error {
                return Err(error);
            }
            display_error("No spore.yml/yaml, app.yml/yaml, or package.yml/yaml found");
            display_info("Run from a directory containing one of these config files");
            anyhow::bail!("No spore.yml/yaml, app.yml/yaml, or package.yml/yaml found");
        } else {
            display_error("No scripts found");
            display_info("Add scripts to spore.yml/yaml, app.yml/yaml, or package.yml/yaml");
            println!("\n{}", style("Example:").bold());
            println!("scripts:");
            println!("  build: \"npm run build\"");
            println!("  test: \"npm test\"");
            println!("  dev: \"npm run dev\"");
            anyhow::bail!("No scripts found");
        }
    }

    println!("{}", style("🚀 Select a script to run:").bold().cyan());

    let script_options: Vec<String> = all_scripts
        .iter()
        .map(|(display_name, _, _, _)| display_name.clone())
        .collect();

    let selection = Select::new("Select a script:", script_options.clone())
        .with_vim_mode(true)
        .with_help_message("Use arrow keys or j/k to navigate, Enter to select, Esc to cancel\nPress 'q' to quit without running any script")
        .prompt();

    match selection {
        Ok(selected_text) => {
            let selected_index = script_options
                .iter()
                .position(|option| option == &selected_text)
                .unwrap_or(0);
            let (_, script_name, script_command, context) = &all_scripts[selected_index];

            println!("✨ Running script: {}", style(script_name).green().bold());
            println!("📍 Command: {}", style(script_command).dim());
            println!("🔧 Context: {}", context);
            println!();

            let working_dir = if let Some(project) = &project {
                if context == "project" {
                    &project.root
                } else {
                    &current_dir
                }
            } else {
                &current_dir
            };

            execute_script(script_name, script_command, working_dir, context).await?;
        }
        Err(inquire::InquireError::OperationCanceled) => {
            display_info("Script selection cancelled");
        }
        Err(error) => {
            let message = format!("Selection error: {}", error);
            display_error(&message);
            return Err(anyhow::anyhow!(message));
        }
    }

    Ok(())
}
