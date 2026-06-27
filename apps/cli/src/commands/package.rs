use anyhow::{Context, Result};
use std::fs;

use crate::commands::common::{
    create_spinner, display_error, display_info, display_success, fail_progress, finish_progress,
};
use crate::config::AppConfig;
use crate::dependency::types::PackageSource;
use crate::dependency::DependencyRef;
use crate::project::Project;
use crate::typescript::TypeScriptManager;
use crate::validation::sanitize_input;

pub async fn link_packages(use_symlinks: bool) -> Result<()> {
    let start_time = std::time::Instant::now();
    let current_dir = std::env::current_dir()?;
    let mut project = match Project::find_and_load(&current_dir) {
        Ok(project) => project,
        Err(_) => {
            display_error("Cannot link packages: No spore.yml found in current directory or any parent directory");
            display_info("You must be inside a Spore project to link packages");
            display_info("Run 'spore init <project-name>' to initialize a new Spore project");
            display_info("Or navigate to an existing Spore project directory");
            anyhow::bail!("Not in a Spore project");
        }
    };

    let mode = if use_symlinks { "symlinked" } else { "copied" };
    let spinner = create_spinner(&format!("Linking packages ({} mode)...", mode));

    for app_name in project.get_app_names() {
        let resolution = match project.resolve_dependencies(&app_name, false, None).await {
            Ok(resolution) => resolution,
            Err(e) => {
                fail_progress(&spinner, "Failed to resolve dependencies");
                return Err(e.context(format!(
                    "Failed to resolve dependencies for app '{}'",
                    app_name
                )));
            }
        };

        if let Err(e) = project
            .install_dependencies(&app_name, &resolution, use_symlinks)
            .await
        {
            fail_progress(&spinner, "Failed to install resolved packages");
            return Err(e.context(format!(
                "Failed to install resolved packages for app '{}'",
                app_name
            )));
        }
    }

    spinner.set_message("Setting up TypeScript aliases...".to_string());

    if let Err(e) = TypeScriptManager::new(&project).setup_aliases_for_all_apps() {
        fail_progress(&spinner, "Failed to setup TypeScript aliases");
        return Err(e.context("Failed to setup TypeScript aliases"));
    }

    let duration = start_time.elapsed();
    finish_progress(
        &spinner,
        &format!("All packages {} and TypeScript configured", mode),
    );
    display_success(&format!(
        "Successfully {} all packages and updated TypeScript configurations",
        mode
    ));
    display_info(&format!("Completed in {}ms", duration.as_millis()));
    Ok(())
}

pub async fn add_package(package_spec: &str, auto_link: bool) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let sanitized_spec = sanitize_input(package_spec);
    if sanitized_spec.is_empty() {
        anyhow::bail!("Package specification cannot be empty");
    }

    let dependency_ref = DependencyRef::parse(&sanitized_spec)?;
    let package_name = dependency_ref.name.clone();
    let storage_spec = dependency_ref.storage_spec_with_default("latest");
    let display_name = storage_spec.clone();

    let app_yml_path = current_dir.join("app.yml");
    if !app_yml_path.exists() {
        let project = match Project::find_and_load(&current_dir) {
            Ok(project) => project,
            Err(_) => {
                anyhow::bail!("Cannot install package: Not in a Spore project directory\n💡 You must be inside a Spore project to install packages\n💡 Run 'spore init <project-name>' to initialize a new project\n💡 Or navigate to an existing Spore project directory\n💡 Then navigate to an app directory and run 'spore install' there");
            }
        };

        if !project.apps.is_empty() {
            println!("📋 Available apps:");
            for app_name in project.apps.keys() {
                println!("  • {}", app_name);
            }
            println!("💡 Navigate to an app directory and run 'spore install' there");
        } else {
            println!("💡 Create an app first with 'spore init:app <name>'");
        }
        return Ok(());
    }

    let mut app_config = AppConfig::from_file(&app_yml_path)?;
    app_config.validate_package_name(&package_name)?;

    if matches!(dependency_ref.source, PackageSource::Local) {
        let project = match Project::find_and_load(&current_dir) {
            Ok(project) => project,
            Err(e) => {
                anyhow::bail!("Failed to load project configuration: {}\n💡 Ensure you're in a valid Spore project directory\n💡 Check that spore.yml exists and is properly formatted", e);
            }
        };

        if !project.packages.contains_key(&package_name) {
            let available_packages = project
                .packages
                .keys()
                .map(|s| s.as_str())
                .collect::<Vec<_>>();
            if available_packages.is_empty() {
                anyhow::bail!("Local package '{}' does not exist in this project\n💡 No packages found in this project\n💡 Create a package first with: spore init:package {}", package_name, package_name);
            } else {
                anyhow::bail!("Local package '{}' does not exist in this project\n💡 Available local packages: {}\n💡 Create the package with: spore init:package {}\n💡 Or install from Spore Space with: @team/{}",
                    package_name,
                    available_packages.join(", "),
                    package_name,
                    package_name
                );
            }
        }
    }

    if app_config.packages.is_none() {
        app_config.packages = Some(Vec::new());
    }

    let packages = app_config.packages.as_mut().unwrap();
    if packages.contains(&storage_spec) {
        println!(
            "📦 Package '{}' is already added to app '{}'",
            display_name, app_config.name
        );
        return Ok(());
    }

    packages.retain(|existing_spec| {
        DependencyRef::parse(existing_spec)
            .map(|existing| existing.name != package_name)
            .unwrap_or(true)
    });
    packages.push(storage_spec.clone());

    let yaml_content = serde_yaml::to_string(&app_config)?;
    fs::write(&app_yml_path, yaml_content).context("Failed to update app.yml")?;

    println!(
        "✅ Added package '{}' to app '{}'",
        display_name, app_config.name
    );

    if auto_link {
        println!("🔗 Linking packages...");
        link_packages(false).await?;
    } else {
        println!("💡 Run 'spore link' to apply the changes");
    }

    Ok(())
}
