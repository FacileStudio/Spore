use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::PathBuf;

use super::super::common::{
    prompt_for_confirm_with_help, prompt_for_description_with_help,
    prompt_for_input_with_validation,
};
use crate::config::{SporeConfig, TsAlias};
use crate::validation::{
    sanitize_input, validate_description, validate_path, validate_project_name,
};

pub(super) fn init_project(
    name: Option<&str>,
    path: Option<&str>,
    description: Option<&str>,
) -> Result<()> {
    let interactive = name.is_none() || path.is_none();

    println!();
    println!(
        "{}",
        style("🚀 Welcome to Spore Project Initializer")
            .bold()
            .cyan()
    );
    println!(
        "{}",
        style("Let's create a new Spore project together!").dim()
    );
    println!();

    let project_name = match name {
        Some(name) => {
            let sanitized = sanitize_input(name);
            validate_project_name(&sanitized)?;
            sanitized
        }
        None => {
            let input = prompt_for_input_with_validation(
                "✨ What's your project name?",
                None,
                Some("Use letters, numbers, spaces, dots, hyphens, or underscores"),
                Some(|input: &str| {
                    let sanitized = sanitize_input(input);
                    match validate_project_name(&sanitized) {
                        Ok(()) => Ok(inquire::validator::Validation::Valid),
                        Err(error) => Ok(inquire::validator::Validation::Invalid(
                            format!("⚠️  {}", error).into(),
                        )),
                    }
                }),
            )?;
            sanitize_input(&input)
        }
    };

    let project_description = match description {
        Some(description) => {
            let sanitized = sanitize_input(description);
            if !sanitized.is_empty() {
                validate_description(&sanitized)?;
                Some(sanitized)
            } else {
                None
            }
        }
        None if interactive => {
            let desc = prompt_for_description_with_help(
                "📝 Project description (optional)",
                Some("A new Spore project"),
                Some("Briefly describe your project's purpose or goals (max 500 chars)"),
            )?;
            let sanitized = sanitize_input(&desc);
            if !sanitized.is_empty() {
                validate_description(&sanitized)?;
                Some(sanitized)
            } else {
                None
            }
        }
        None => None,
    };

    let target_dir = match path {
        Some(path) => {
            validate_path(path, "target path", false)?;
            PathBuf::from(path)
        }
        None if interactive => {
            println!("💡 Path options:");
            println!(
                "   {} Default location: {}",
                style("•").dim(),
                style(".").cyan()
            );
            println!(
                "   {} Current directory: {}",
                style("•").dim(),
                style(".").cyan()
            );
            println!(
                "   {} Custom path: {}",
                style("•").dim(),
                style("./my-custom-path").dim()
            );
            println!();
            let path_str = prompt_for_input_with_validation(
                "📁 Where should we create the project?",
                Some("."),
                Some("Press Enter for current directory, '.' for current directory, or specify custom path"),
                Some(|input: &str| {
                    let trimmed = input.trim();
                    if trimmed.is_empty() {
                        return Ok(inquire::validator::Validation::Valid);
                    }

                    match validate_path(trimmed, "target path", false) {
                        Ok(()) => Ok(inquire::validator::Validation::Valid),
                        Err(error) => Ok(inquire::validator::Validation::Invalid(
                            format!("⚠️  {}", error).into(),
                        )),
                    }
                }),
            )?
            .trim()
            .to_string();

            if path_str.is_empty() || path_str == "." {
                std::env::current_dir()?
            } else {
                PathBuf::from(path_str)
            }
        }
        None => std::env::current_dir()?,
    };

    if interactive {
        println!();
        println!("{}", style("📋 Project Summary:").bold().green());
        println!(
            "   {} {}",
            style("Name:").dim(),
            style(&project_name).bold()
        );
        if let Some(description) = &project_description {
            println!("   {} {}", style("Description:").dim(), description);
        }
        let location_description = if target_dir == std::env::current_dir()? {
            format!("{} (current directory)", target_dir.display())
        } else if target_dir.file_name().and_then(|name| name.to_str()) == Some(&project_name) {
            format!("{} (new folder)", target_dir.display())
        } else {
            target_dir.display().to_string()
        };
        println!("   {} {}", style("Location:").dim(), location_description);
        println!();

        if !prompt_for_confirm_with_help(
            "Create this project?",
            Some(true),
            Some("This will create the directory structure and configuration files"),
        )? {
            println!("{}", style("❌ Project creation cancelled").red());
            return Ok(());
        }
    }

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)?;
    }

    let spore_yml_path = target_dir.join("spore.yml");
    if spore_yml_path.exists() {
        anyhow::bail!("Cannot initialize project: spore.yml already exists in directory '{}'\n💡 This directory is already a Spore project\n💡 Use 'spore init:package' or 'spore init:app' to add components to this project\n💡 Or choose a different directory for your new project", target_dir.display());
    }

    let config = SporeConfig {
        name: project_name.clone(),
        description: project_description,
        ts_alias: Some(TsAlias::Boolean(false)),
        apps: None,
        scripts: None,
        variables: None,
    };

    let yaml_content = serde_yaml::to_string(&config)?;
    fs::write(spore_yml_path, yaml_content).context("Failed to create spore.yml")?;
    fs::create_dir_all(target_dir.join("packages"))?;
    fs::create_dir_all(target_dir.join("apps"))?;

    println!("✅ Initialized new Spore project: {}", project_name);
    println!("📁 Created directories: packages/, apps/");
    println!("💡 Use 'spore init:package <name>' to create packages");
    println!("💡 Use 'spore init:app <name>' to create apps");

    Ok(())
}
