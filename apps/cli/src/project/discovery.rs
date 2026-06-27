use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use crate::config::{AppConfig, SporeConfig, PackageConfig};
use crate::utils;
use crate::variables::{VariableContext, VariableInterpolation};

use super::Project;

impl Project {
    pub fn find_and_load(start_dir: &Path) -> Result<Project> {
        let project_root = Self::find_project_root(start_dir)?;
        let config_path = utils::find_yaml_file(&project_root, "spore").ok_or_else(|| {
            anyhow::anyhow!("No spore.yml or spore.yaml file found in {:?}", project_root)
        })?;
        let mut config = SporeConfig::from_file(&config_path)
            .with_context(|| format!("Failed to load config from {:?}", config_path))?;

        let variable_context =
            VariableContext::new(&config.name, &project_root).with_project_variables(&config);

        config
            .interpolate_variables(&variable_context)
            .context("Failed to interpolate variables in project config")?;

        let mut project = Project {
            root: project_root,
            config,
            packages: std::collections::HashMap::new(),
            package_paths: std::collections::HashMap::new(),
            apps: std::collections::HashMap::new(),
            dependency_resolver: None,
            variable_context,
        };

        project.load_packages()?;
        project.load_apps()?;
        Ok(project)
    }

    pub fn find_project_root(start_dir: &Path) -> Result<PathBuf> {
        let mut current = start_dir.to_path_buf();

        loop {
            if utils::yaml_config_exists(&current, "spore") {
                return match current.canonicalize() {
                    Ok(canonical) => Ok(canonical),
                    Err(_) => Ok(current),
                };
            }

            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => anyhow::bail!("No spore.yml or spore.yaml file found in directory tree starting from '{}'\n💡 You are not inside a Spore project\n💡 Run 'spore init <project-name>' to initialize a new project\n💡 Or navigate to an existing Spore project directory", start_dir.display()),
            }
        }
    }

    fn load_packages(&mut self) -> Result<()> {
        let packages_dir = self.root.join("packages");
        if !packages_dir.exists() {
            return Ok(());
        }

        let entries: Vec<_> = std::fs::read_dir(&packages_dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_dir() {
                    utils::find_yaml_file(&path, "package").map(|config_path| (path, config_path))
                } else {
                    None
                }
            })
            .collect();

        for (path, package_config_path) in entries {
            let mut package_config = PackageConfig::from_file(&package_config_path)
                .with_context(|| format!("Failed to load {:?}", package_config_path))?;

            let package_name = package_config.name.clone();
            if let Some(existing_path) = self.package_paths.get(&package_name) {
                anyhow::bail!(
                    "Duplicate local package name '{}' found in '{}' and '{}'\n💡 Local package names must be unique across the project\n💡 Rename one package or update its package.yml name",
                    package_name,
                    existing_path.display(),
                    path.display()
                );
            }

            let package_context = self
                .variable_context
                .clone()
                .with_package_variables(&package_config);

            package_config
                .interpolate_variables(&package_context)
                .with_context(|| {
                    format!(
                        "Failed to interpolate variables in package config for '{}'",
                        package_name
                    )
                })?;

            self.package_paths.insert(package_name.clone(), path);
            self.packages.insert(package_name, package_config);
        }

        Ok(())
    }

    fn load_apps(&mut self) -> Result<()> {
        let apps_dir = self.root.join("apps");
        if !apps_dir.exists() {
            return Ok(());
        }

        let entries: Result<Vec<_>, _> = std::fs::read_dir(&apps_dir)?
            .map(|entry| -> Result<_, std::io::Error> {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    if let Some(config_path) = utils::find_yaml_file(&path, "app") {
                        Ok(Some((path, config_path)))
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            })
            .filter_map(|result| match result {
                Ok(Some(pair)) => Some(Ok(pair)),
                Ok(None) => None,
                Err(e) => Some(Err(e)),
            })
            .collect();

        for (path, app_config_path) in entries? {
            let mut app_config = AppConfig::from_file(&app_config_path)
                .with_context(|| format!("Failed to load {:?}", app_config_path))?;

            let app_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| anyhow::anyhow!("Invalid app directory name"))?
                .to_string();

            let app_context = self
                .variable_context
                .clone()
                .with_app_variables(&app_config);
            app_config
                .interpolate_variables(&app_context)
                .with_context(|| {
                    format!(
                        "Failed to interpolate variables in app config for '{}'",
                        app_name
                    )
                })?;

            self.apps.insert(app_name, app_config);
        }

        Ok(())
    }
}
