use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::PathBuf;

use super::super::common::{
    determine_target_directory, prompt_for_confirm_with_help, prompt_for_description_with_help,
    prompt_for_input_with_validation,
};
use super::template::{create_basic_structure, use_package_as_template};
use crate::config::AppConfig;
use crate::validation::{
    sanitize_input, validate_app_name, validate_description, validate_template_name,
};

pub(super) async fn init_app(
    name: Option<&str>,
    template: Option<&str>,
    description: Option<&str>,
    path: Option<&str>,
    here: bool,
) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let interactive = name.is_none();

    if interactive {
        println!();
        println!("{}", style("🚀 Welcome to Spore App Creator").bold().cyan());
        println!("{}", style("Let's create a new app together!").dim());
        println!();
    }

    let app_name = match name {
        Some(name) => {
            let sanitized = sanitize_input(name);
            validate_app_name(&sanitized)?;
            sanitized
        }
        None => {
            let input = prompt_for_input_with_validation(
                "✨ What's your app name?",
                None,
                Some("Use lowercase letters, numbers, dots, hyphens, underscores"),
                Some(|input: &str| {
                    let sanitized = sanitize_input(input);
                    match validate_app_name(&sanitized) {
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

    let app_description = match description {
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
                "📝 App description (optional)",
                Some("A new Spore app"),
                Some("Describe the app's purpose and target users (max 500 chars)"),
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

    let use_template = match template {
        Some(_) => true,
        None if interactive => prompt_for_confirm_with_help(
            "📦 Do you want to use a template from Spore Space?",
            Some(false),
            Some("Templates provide pre-configured app structure and framework setup"),
        )?,
        None => false,
    };

    let app_template = if use_template {
        match template {
            Some(template) => {
                let template_name = if !template.starts_with('@') {
                    format!("@{}", template)
                } else {
                    template.to_string()
                };
                validate_template_name(&template_name)?;
                template_name
            }
            None if interactive => {
                println!();
                println!(
                    "{}",
                    style("Enter a package name to use as template.").dim()
                );
                println!(
                    "{}",
                    style(
                        "Examples: @svelte-app-starter, @nextjs-with-auth, @my-team/app-template"
                    )
                    .dim()
                );
                println!();
                let template_input = prompt_for_input_with_validation(
                    "🎨 Template package name",
                    None,
                    Some("Must start with @ (e.g., @nextjs-starter or @team/app-template)"),
                    None,
                )?;
                if !template_input.starts_with('@') {
                    format!("@{}", template_input)
                } else {
                    template_input
                }
            }
            None => return Ok(()),
        }
    } else {
        String::new()
    };

    let (base_target_dir, context, _) = if here {
        (
            current_dir.clone(),
            "in current directory".to_string(),
            false,
        )
    } else {
        match path {
            Some(path) => (
                PathBuf::from(path),
                format!("in specified path: {}", path),
                false,
            ),
            None => determine_target_directory(&current_dir, "apps")?,
        }
    };

    let target_dir = if here || path.is_some() {
        base_target_dir
    } else {
        base_target_dir.join(&app_name)
    };

    if interactive {
        println!();
        println!("{}", style("📋 App Summary:").bold().green());
        println!("   {} {}", style("Name:").dim(), style(&app_name).bold());
        if let Some(description) = &app_description {
            println!("   {} {}", style("Description:").dim(), description);
        }
        println!("   {} {}", style("Template:").dim(), app_template);
        println!(
            "   {} {} ({})",
            style("Location:").dim(),
            target_dir.display(),
            context
        );
        println!();

        if !prompt_for_confirm_with_help(
            "Create this app?",
            Some(true),
            Some("This will create the app directory with configuration and template files"),
        )? {
            println!("{}", style("❌ App creation cancelled").red());
            return Ok(());
        }
    }

    if !here && target_dir.exists() && !path.map(|path| path == ".").unwrap_or(false) {
        anyhow::bail!("Cannot create app: directory '{}' already exists\n💡 Choose a different app name or path\n💡 Use --here flag to create the app in the current directory\n💡 Or remove the existing directory first", target_dir.display());
    }

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)?;
    }

    let app_yml_path = target_dir.join("app.yml");
    if app_yml_path.exists() {
        anyhow::bail!("Cannot create app: app.yml already exists in directory '{}'\n💡 This directory already contains a Spore app\n💡 Choose a different directory or app name\n💡 Use --here flag if you want to overwrite (not recommended)", target_dir.display());
    }

    let config = AppConfig {
        name: app_name.clone(),
        description: app_description.clone(),
        ts_alias: None,
        packages: None,
        scripts: None,
        variables: None,
    };

    let yaml_content = serde_yaml::to_string(&config)?;
    fs::write(&app_yml_path, yaml_content).context("Failed to create app.yml")?;

    if !app_template.is_empty() {
        use_package_as_template(
            &app_template,
            &target_dir,
            Some(&app_name),
            Some("1.0.0"),
            app_description.as_deref(),
        )
        .await?;
    } else {
        create_basic_structure(&target_dir, false)?;
    }

    println!("✅ Created app: {}", app_name);
    println!("📁 Location: {}", target_dir.display());
    println!("💡 Use 'spore install <package>' to add dependencies");
    println!("💡 Use 'spore link' to install and link packages");

    Ok(())
}
