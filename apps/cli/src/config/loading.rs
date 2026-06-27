use anyhow::Context;
use std::path::Path;

use super::{parse_yaml_error_with_context, AppConfig, ConfigType, SporeConfig, PackageConfig};

fn read_config_file(path: &Path, prefix: &str) -> anyhow::Result<(String, String)> {
    let file_extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("yml")
        .to_string();
    let content = std::fs::read_to_string(path).with_context(|| {
        format!(
            "Failed to read {}.{} file: {}",
            prefix,
            file_extension,
            path.display()
        )
    })?;

    Ok((file_extension, content))
}

impl SporeConfig {
    #[allow(dead_code)]
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let (file_extension, content) = read_config_file(path, "spore")?;

        if content.is_empty() {
            anyhow::bail!("Configuration file spore.{} is empty at '{}'\n💡 A valid spore.{} file should contain at least a 'name' field\n💡 Example:\n   name: my-project\n   description: My awesome project",
                file_extension, path.display(), file_extension);
        }

        let config: SporeConfig = match serde_yaml::from_str(&content) {
            Ok(config) => config,
            Err(error) => {
                let details = parse_yaml_error_with_context(&error, ConfigType::Spore);
                anyhow::bail!(
                    "Failed to parse spore.{} file: {}: {}",
                    file_extension,
                    path.display(),
                    details
                );
            }
        };

        config.validate()?;
        Ok(config)
    }
}

impl PackageConfig {
    #[allow(dead_code)]
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let (file_extension, content) = read_config_file(path, "package")?;

        if content.is_empty() {
            anyhow::bail!("Package configuration file package.{} is empty at '{}'\n💡 A valid package.{} file should contain at least 'name' and 'version' fields\n💡 Example:\n   name: my-package\n   version: 1.0.0\n   description: My awesome package",
                file_extension, path.display(), file_extension);
        }

        let config: PackageConfig = match serde_yaml::from_str(&content) {
            Ok(config) => config,
            Err(error) => {
                let details = parse_yaml_error_with_context(&error, ConfigType::Package);
                anyhow::bail!(
                    "Failed to parse package.{} file: {}: {}",
                    file_extension,
                    path.display(),
                    details
                );
            }
        };

        config.validate()?;
        Ok(config)
    }
}

impl AppConfig {
    #[allow(dead_code)]
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let (file_extension, content) = read_config_file(path, "app")?;

        if content.is_empty() {
            anyhow::bail!("App configuration file app.{} is empty at '{}'\n💡 A valid app.{} file should contain at least a 'name' field\n💡 Example:\n   name: my-app\n   description: My awesome app\n   packages:\n     - utils",
                file_extension, path.display(), file_extension);
        }

        let config: AppConfig = match serde_yaml::from_str(&content) {
            Ok(config) => config,
            Err(error) => {
                let details = parse_yaml_error_with_context(&error, ConfigType::App);
                anyhow::bail!(
                    "Failed to parse app.{} file: {}: {}",
                    file_extension,
                    path.display(),
                    details
                );
            }
        };

        config.validate()?;
        Ok(config)
    }
}
