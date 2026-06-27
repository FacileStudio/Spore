use anyhow::Result;
use std::path::Path;

use crate::config::{AppConfig, PackageConfig};
use crate::project::Project;
use crate::utils;

pub(super) fn load_project_context(current_dir: &Path) -> (Option<Project>, Option<anyhow::Error>) {
    match Project::find_and_load(current_dir) {
        Ok(project) => (Some(project), None),
        Err(error) => match Project::find_project_root(current_dir) {
            Ok(project_root) => {
                let config_path = utils::find_yaml_file(&project_root, "spore")
                    .unwrap_or_else(|| project_root.join("spore.yml"));
                let details = format!("{:#}", error);
                let message = format!(
                    "🔍 Found spore config at: {}\n❌ Failed to load spore config: {}\n💡 This could be due to:\n   • Invalid YAML syntax\n   • Missing required fields\n   • File permission issues\n   • Corrupted file content",
                    config_path.display(),
                    details
                );
                (None, Some(anyhow::anyhow!(message)))
            }
            Err(_) => (None, None),
        },
    }
}

pub(super) fn find_script_in_app(
    app_yml_path: &Path,
    script_name: &str,
    project: Option<&Project>,
) -> Result<Option<String>> {
    if let Some(project) = project {
        if let Some((_, app_config)) = get_project_app_config(project, app_yml_path) {
            return Ok(app_config
                .scripts
                .as_ref()
                .and_then(|scripts| scripts.get(script_name))
                .cloned());
        }
    }

    let app_config = AppConfig::from_file(app_yml_path)?;
    Ok(app_config
        .scripts
        .as_ref()
        .and_then(|scripts| scripts.get(script_name))
        .cloned())
}

pub(super) fn find_script_in_package(
    package_yml_path: &Path,
    script_name: &str,
    project: Option<&Project>,
) -> Result<Option<String>> {
    if let Some(project) = project {
        if let Some((_, package_config)) = get_project_package_config(project, package_yml_path) {
            return Ok(package_config
                .scripts
                .as_ref()
                .and_then(|scripts| scripts.get(script_name))
                .cloned());
        }
    }

    let package_config = PackageConfig::from_file(package_yml_path)?;
    Ok(package_config
        .scripts
        .as_ref()
        .and_then(|scripts| scripts.get(script_name))
        .cloned())
}

pub(super) fn get_project_app_config<'a>(
    project: &'a Project,
    app_yml_path: &Path,
) -> Option<(String, &'a AppConfig)> {
    let app_name = app_yml_path.parent()?.file_name()?.to_str()?;
    project
        .apps
        .get(app_name)
        .map(|config| (app_name.to_string(), config))
}

pub(super) fn get_project_package_config<'a>(
    project: &'a Project,
    package_yml_path: &Path,
) -> Option<(String, &'a PackageConfig)> {
    let package_dir = package_yml_path.parent()?;

    project
        .package_paths
        .iter()
        .find_map(|(name, path)| (path == package_dir).then_some(name.clone()))
        .and_then(|package_name| {
            project
                .packages
                .get(&package_name)
                .map(|config| (package_name, config))
        })
}

pub(super) fn show_available_scripts(current_dir: &Path, project: &Project) -> Result<()> {
    println!("💡 Available scripts:");

    let mut found_any = false;

    if let Some(app_config_path) = utils::find_yaml_file(current_dir, "app") {
        if let Some((app_name, app_config)) = get_project_app_config(project, &app_config_path) {
            if let Some(scripts) = &app_config.scripts {
                if !scripts.is_empty() {
                    println!("  📱 App scripts ({})", app_name);
                    for (name, command) in scripts {
                        println!("    • {} - {}", name, command);
                    }
                    found_any = true;
                }
            }
        } else if let Ok(app_config) = AppConfig::from_file(&app_config_path) {
            if let Some(scripts) = &app_config.scripts {
                if !scripts.is_empty() {
                    println!("  📱 App scripts ({})", app_config.name);
                    for (name, command) in scripts {
                        println!("    • {} - {}", name, command);
                    }
                    found_any = true;
                }
            }
        }
    }

    if let Some(package_config_path) = utils::find_yaml_file(current_dir, "package") {
        if let Some((package_name, package_config)) =
            get_project_package_config(project, &package_config_path)
        {
            if let Some(scripts) = &package_config.scripts {
                if !scripts.is_empty() {
                    println!("  📦 Package scripts ({})", package_name);
                    for (name, command) in scripts {
                        println!("    • {} - {}", name, command);
                    }
                    found_any = true;
                }
            }
        } else if let Ok(package_config) = PackageConfig::from_file(&package_config_path) {
            if let Some(scripts) = &package_config.scripts {
                if !scripts.is_empty() {
                    println!("  📦 Package scripts ({})", package_config.name);
                    for (name, command) in scripts {
                        println!("    • {} - {}", name, command);
                    }
                    found_any = true;
                }
            }
        }
    }

    if let Some(scripts) = &project.config.scripts {
        if !scripts.is_empty() {
            println!("  🏗️  Project scripts ({})", project.config.name);
            for (name, command) in scripts {
                println!("    • {} - {}", name, command);
            }
            found_any = true;
        }
    }

    if !found_any {
        println!("  (No scripts defined in any config files)");
    }

    Ok(())
}
