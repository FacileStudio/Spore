use anyhow::Result;

use crate::commands::common::{display_error, display_info};
use crate::utils;
use crate::validation::{sanitize_input, validate_script_name};

use super::execute::execute_script;
use super::lookup::{
    find_script_in_app, find_script_in_package, load_project_context, show_available_scripts,
};

pub(super) async fn run_script(script_name: &str) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let sanitized_script_name = sanitize_input(script_name);
    validate_script_name(&sanitized_script_name)?;
    let (project, project_error) = load_project_context(&current_dir);

    if let Some(app_config_path) = utils::find_yaml_file(&current_dir, "app") {
        if let Some(script_command) =
            find_script_in_app(&app_config_path, script_name, project.as_ref())?
        {
            return execute_script(script_name, &script_command, &current_dir, "app").await;
        }
    }

    if let Some(package_config_path) = utils::find_yaml_file(&current_dir, "package") {
        if let Some(script_command) =
            find_script_in_package(&package_config_path, script_name, project.as_ref())?
        {
            return execute_script(script_name, &script_command, &current_dir, "package").await;
        }
    }

    let project = match project {
        Some(project) => project,
        None => {
            if let Some(error) = project_error {
                return Err(error);
            }
            anyhow::bail!(
                "❌ No spore.yml/yaml, app.yml/yaml, or package.yml/yaml found\n💡 Run from a directory containing one of these config files"
            );
        }
    };

    if let Some(scripts) = &project.config.scripts {
        if let Some(script_command) = scripts.get(script_name) {
            return execute_script(script_name, script_command, &project.root, "project").await;
        }
    }

    display_error(&format!("Script '{}' not found", script_name));
    display_info("Available scripts:");
    show_available_scripts(&current_dir, &project)?;

    anyhow::bail!("Script '{}' not found", script_name)
}
