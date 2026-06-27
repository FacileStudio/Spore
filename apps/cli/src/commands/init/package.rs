use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::PathBuf;

use super::super::common::{
    determine_target_directory, prompt_for_confirm_with_help, prompt_for_description_with_help,
    prompt_for_input_with_validation,
};
use super::template::{create_basic_structure, use_package_as_template};
use crate::config::PackageConfig;
use crate::validation::{
    sanitize_input, validate_description, validate_package_name, validate_semver,
    validate_team_name, validate_template_name,
};

pub(super) async fn init_package(
    name: Option<&str>,
    team: Option<&str>,
    version: Option<&str>,
    template: Option<&str>,
    description: Option<&str>,
    path: Option<&str>,
    here: bool,
) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let interactive = name.is_none();

    if interactive {
        println!();
        println!(
            "{}",
            style("📦 Welcome to Spore Package Creator").bold().cyan()
        );
        println!("{}", style("Let's create a new package together!").dim());
        println!();
    }

    let package_name = match name {
        Some(name) => {
            let sanitized = sanitize_input(name);
            validate_package_name(&sanitized)?;
            sanitized
        }
        None => {
            let input = prompt_for_input_with_validation(
                "✨ What's your package name?",
                None,
                Some("Use lowercase letters, numbers, dots, hyphens, underscores"),
                Some(|input: &str| {
                    let sanitized = sanitize_input(input);
                    match validate_package_name(&sanitized) {
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

    let team_name = match team {
        Some(team) => {
            let sanitized = sanitize_input(team);
            if !sanitized.is_empty() {
                validate_team_name(&sanitized)?;
                Some(sanitized)
            } else {
                None
            }
        }
        None if interactive => {
            let team = prompt_for_input_with_validation(
                "👥 Team name (optional, for namespaced packages)",
                None,
                Some("Use lowercase letters, numbers, dots, hyphens, underscores"),
                Some(|input: &str| {
                    let sanitized = sanitize_input(input);
                    if sanitized.is_empty() {
                        Ok(inquire::validator::Validation::Valid)
                    } else {
                        match validate_team_name(&sanitized) {
                            Ok(()) => Ok(inquire::validator::Validation::Valid),
                            Err(error) => Ok(inquire::validator::Validation::Invalid(
                                format!("⚠️  {}", error).into(),
                            )),
                        }
                    }
                }),
            )
            .ok();

            team.and_then(|team| {
                let sanitized = sanitize_input(&team);
                if sanitized.is_empty() {
                    None
                } else {
                    Some(sanitized)
                }
            })
        }
        None => None,
    };

    let package_version = match version {
        Some(version) => {
            let sanitized = sanitize_input(version);
            validate_semver(&sanitized)?;
            sanitized
        }
        None if interactive => {
            let input = prompt_for_input_with_validation(
                "🏷️  Package version",
                Some("1.0.0"),
                Some("Use semantic versioning: MAJOR.MINOR.PATCH"),
                Some(|input: &str| {
                    let sanitized = sanitize_input(input);
                    match validate_semver(&sanitized) {
                        Ok(()) => Ok(inquire::validator::Validation::Valid),
                        Err(error) => Ok(inquire::validator::Validation::Invalid(
                            format!("⚠️  {}", error).into(),
                        )),
                    }
                }),
            )?;
            sanitize_input(&input)
        }
        None => "1.0.0".to_string(),
    };

    let package_description = match description {
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
                "📝 Package description (optional)",
                Some("A new Spore package"),
                Some("Describe what this package does and how others can use it (max 500 chars)"),
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
            Some("Templates provide pre-configured project structure and dependencies"),
        )?,
        None => false,
    };

    let package_template = if use_template {
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
                    style("Examples: @svelte-starter, @react-with-auth, @my-team/template").dim()
                );
                println!();
                let template_input = prompt_for_input_with_validation(
                    "🎨 Template package name",
                    None,
                    Some("Must start with @ (e.g., @my-template or @team/template-name)"),
                    Some(|input: &str| {
                        let sanitized = sanitize_input(input);
                        let template_name = if !sanitized.starts_with('@') {
                            format!("@{}", sanitized)
                        } else {
                            sanitized
                        };
                        match validate_template_name(&template_name) {
                            Ok(()) => Ok(inquire::validator::Validation::Valid),
                            Err(error) => Ok(inquire::validator::Validation::Invalid(
                                format!("⚠️  {}", error).into(),
                            )),
                        }
                    }),
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
            None => determine_target_directory(&current_dir, "packages")?,
        }
    };

    let target_dir = if here || path.is_some() {
        base_target_dir
    } else {
        base_target_dir.join(&package_name)
    };

    if interactive {
        println!();
        println!("{}", style("📋 Package Summary:").bold().green());
        println!(
            "   {} {}",
            style("Name:").dim(),
            style(&package_name).bold()
        );
        if let Some(team) = &team_name {
            println!(
                "   {} @{}/{}",
                style("Namespaced:").dim(),
                team,
                package_name
            );
        }
        println!("   {} {}", style("Version:").dim(), package_version);
        if let Some(description) = &package_description {
            println!("   {} {}", style("Description:").dim(), description);
        }
        println!("   {} {}", style("Template:").dim(), package_template);
        println!(
            "   {} {} ({})",
            style("Location:").dim(),
            target_dir.display(),
            context
        );
        println!();

        if !prompt_for_confirm_with_help(
            "Create this package?",
            Some(true),
            Some("This will create the package directory with configuration and template files"),
        )? {
            println!("{}", style("❌ Package creation cancelled").red());
            return Ok(());
        }
    }

    if !here && target_dir.exists() && !path.map(|path| path == ".").unwrap_or(false) {
        anyhow::bail!("Cannot create package: directory '{}' already exists\n💡 Choose a different package name or path\n💡 Use --here flag to create the package in the current directory\n💡 Or remove the existing directory first", target_dir.display());
    }

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)?;
    }

    let package_yml_path = target_dir.join("package.yml");
    if package_yml_path.exists() {
        anyhow::bail!("Cannot create package: package.yml already exists in directory '{}'\n💡 This directory already contains a Spore package\n💡 Choose a different directory or package name\n💡 Use --here flag if you want to overwrite (not recommended)", target_dir.display());
    }

    let config = PackageConfig {
        name: package_name.clone(),
        team: team_name.clone(),
        version: package_version.clone(),
        description: package_description.clone(),
        author: None,
        license: None,
        repository: None,
        keywords: None,
        tags: None,
        scripts: None,
        dependencies: None,
        dev_dependencies: None,
        optional_dependencies: None,
        peer_dependencies: None,
        exports: None,
        features: None,
        variables: None,
    };

    let yaml_content = serde_yaml::to_string(&config)?;
    fs::write(&package_yml_path, yaml_content).context("Failed to create package.yml")?;

    if !package_template.is_empty() {
        use_package_as_template(
            &package_template,
            &target_dir,
            Some(&package_name),
            Some(&package_version),
            package_description.as_deref(),
        )
        .await?;
    } else {
        create_basic_structure(&target_dir, true)?;
    }

    let display_name = if let Some(team) = &team_name {
        format!("@{}/{}", team, package_name)
    } else {
        package_name.clone()
    };

    println!("✅ Created package: {}", display_name);
    println!("📁 Location: {}", target_dir.display());

    if let Some(team) = &team_name {
        println!("👥 Team: @{}", team);
        println!(
            "💡 Use 'spore publish --team {}' to publish this package",
            team
        );
    } else {
        println!("💡 Use 'spore publish' to publish this package");
    }

    Ok(())
}
