use anyhow::Context;
use semver::{Version, VersionReq};

use super::{AppConfig, SporeConfig, PackageConfig};

#[allow(dead_code)]
fn validate_safe_name_common(name: &str, context: &str) -> anyhow::Result<()> {
    if name.contains("..") || name.contains('/') || name.contains('\\') {
        anyhow::bail!("{} contains invalid path characters: '{}'", context, name);
    }

    if name.contains('\0') || name.starts_with('.') || name.starts_with('-') {
        anyhow::bail!("{} contains unsafe characters: '{}'", context, name);
    }

    if name.len() > 100 {
        anyhow::bail!("{} is too long (max 100 characters): '{}'", context, name);
    }

    Ok(())
}

fn validate_dependency_name_common(name: &str) -> anyhow::Result<()> {
    if name.is_empty() {
        anyhow::bail!("Dependency name cannot be empty");
    }

    if let Some(scoped) = name.strip_prefix('@') {
        let (scope, package) = scoped
            .split_once('/')
            .ok_or_else(|| anyhow::anyhow!("Invalid scoped package name '{}'", name))?;

        if scope.is_empty() || package.is_empty() {
            anyhow::bail!("Invalid scoped package name '{}'", name);
        }

        if scope.contains("..")
            || scope.contains('\\')
            || scope.contains('\0')
            || package.contains("..")
            || package.contains('\\')
            || package.contains('\0')
        {
            anyhow::bail!("Dependency name contains unsafe characters: '{}'", name);
        }

        if scope.starts_with('.') || package.starts_with('.') || package.starts_with('-') {
            anyhow::bail!("Dependency name contains unsafe characters: '{}'", name);
        }
    }

    if !name.starts_with('@') {
        validate_safe_name_common(name, "Dependency name")?;
    }

    Ok(())
}

fn validate_dependency_spec_common(spec: &str) -> anyhow::Result<()> {
    if spec.trim().is_empty() {
        anyhow::bail!("Dependency specification cannot be empty");
    }

    let (name, version) = if let Some(stripped) = spec.strip_prefix('@') {
        if let Some(second_at) = stripped.rfind('@') {
            let split_at = second_at + 1;
            (&spec[..split_at], Some(&spec[split_at + 1..]))
        } else {
            (spec, None)
        }
    } else if let Some(at_pos) = spec.rfind('@') {
        (&spec[..at_pos], Some(&spec[at_pos + 1..]))
    } else {
        (spec, None)
    };

    validate_dependency_name_common(name)?;

    if let Some(version_req) = version {
        if version_req.is_empty() {
            anyhow::bail!("Dependency version cannot be empty in '{}'", spec);
        }

        let normalized = if version_req == "latest" {
            "*"
        } else {
            version_req
        };
        VersionReq::parse(normalized)
            .with_context(|| format!("Invalid dependency version requirement '{}'", version_req))?;
    }

    Ok(())
}

fn validate_package_version_common(version: &str) -> anyhow::Result<()> {
    if version.trim().is_empty() {
        anyhow::bail!("Package version cannot be empty");
    }

    Version::parse(version).with_context(|| {
        format!(
            "Package version '{}' must follow semantic versioning format\n💡 Examples: 1.0.0, 2.1.5, 1.0.0-beta.1, 2.0.0+build.1",
            version
        )
    })?;

    Ok(())
}

impl SporeConfig {
    #[allow(dead_code)]
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.trim().is_empty() {
            anyhow::bail!("Project name cannot be empty in spore.yml configuration\n💡 Add a name field to your spore.yml file\n💡 Example: name: my-awesome-project");
        }

        self.validate_safe_name(&self.name, "Project name")?;

        if let Some(scripts) = &self.scripts {
            for (name, command) in scripts {
                if name.trim().is_empty() {
                    anyhow::bail!("Script name cannot be empty");
                }
                if name.contains(char::is_whitespace) {
                    anyhow::bail!("Script name cannot contain whitespace: '{}'", name);
                }
                if command.trim().is_empty() {
                    anyhow::bail!("Script command cannot be empty for script '{}'", name);
                }
                self.validate_safe_name(name, "Script name")?;
            }
        }

        if let Some(apps) = &self.apps {
            for (app_name, app_deps) in apps {
                self.validate_safe_name(app_name, "App name")?;
                for package in app_deps.get_packages() {
                    self.validate_dependency_spec(&package)?;
                }
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn validate_safe_name(&self, name: &str, context: &str) -> anyhow::Result<()> {
        validate_safe_name_common(name, context)
    }

    fn validate_dependency_spec(&self, package: &str) -> anyhow::Result<()> {
        validate_dependency_spec_common(package)
    }
}

impl PackageConfig {
    #[allow(dead_code)]
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.trim().is_empty() {
            anyhow::bail!("Package name cannot be empty in package.yml configuration\n💡 Add a name field to your package.yml file\n💡 Example: name: my-package");
        }

        self.validate_safe_name(&self.name, "Package name")?;

        if let Some(team) = &self.team {
            if !team.trim().is_empty() {
                self.validate_safe_name(team, "Team name")?;
            }
        }

        validate_package_version_common(&self.version).map_err(|error| {
            anyhow::anyhow!(
                "Package version is invalid in package.yml configuration: {:#}",
                error
            )
        })?;

        if let Some(tags) = &self.tags {
            for tag in tags {
                if tag.trim().is_empty() {
                    anyhow::bail!("Tag cannot be empty");
                }
                if !tag.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
                    anyhow::bail!("Tag '{}' contains invalid characters. Only lowercase alphanumeric and hyphens allowed", tag);
                }
                if tag.starts_with('-') || tag.ends_with('-') {
                    anyhow::bail!("Tag '{}' cannot start or end with hyphen", tag);
                }
                if tag.len() > 50 {
                    anyhow::bail!("Tag '{}' is too long (max 50 characters)", tag);
                }
            }
        }

        if let Some(scripts) = &self.scripts {
            for (name, command) in scripts {
                if name.trim().is_empty() {
                    anyhow::bail!("Script name cannot be empty");
                }
                if name.contains(char::is_whitespace) {
                    anyhow::bail!("Script name cannot contain whitespace: '{}'", name);
                }
                if command.trim().is_empty() {
                    anyhow::bail!("Script command cannot be empty for script '{}'", name);
                }
                self.validate_safe_name(name, "Script name")?;
            }
        }

        self.validate_dependency_list(self.dependencies.as_deref(), "dependencies")?;
        self.validate_dependency_list(self.dev_dependencies.as_deref(), "dev_dependencies")?;
        self.validate_dependency_list(
            self.optional_dependencies.as_deref(),
            "optional_dependencies",
        )?;
        self.validate_dependency_list(self.peer_dependencies.as_deref(), "peer_dependencies")?;

        Ok(())
    }

    #[allow(dead_code)]
    fn validate_safe_name(&self, name: &str, context: &str) -> anyhow::Result<()> {
        validate_safe_name_common(name, context)
    }

    fn validate_dependency_list(
        &self,
        dependencies: Option<&[String]>,
        field_name: &str,
    ) -> anyhow::Result<()> {
        if let Some(entries) = dependencies {
            for dependency in entries {
                validate_dependency_spec_common(dependency).with_context(|| {
                    format!("Invalid dependency '{}' in {}", dependency, field_name)
                })?;
            }
        }

        Ok(())
    }
}

impl AppConfig {
    #[allow(dead_code)]
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.trim().is_empty() {
            anyhow::bail!("App name cannot be empty in app.yml configuration\n💡 Add a name field to your app.yml file\n💡 Example: name: my-app");
        }

        self.validate_safe_name(&self.name, "App name")?;

        if let Some(scripts) = &self.scripts {
            for (name, command) in scripts {
                if name.trim().is_empty() {
                    anyhow::bail!("Script name cannot be empty");
                }
                if name.contains(char::is_whitespace) {
                    anyhow::bail!("Script name cannot contain whitespace: '{}'", name);
                }
                if command.trim().is_empty() {
                    anyhow::bail!("Script command cannot be empty for script '{}'", name);
                }
                self.validate_safe_name(name, "Script name")?;
            }
        }

        if let Some(packages) = &self.packages {
            for package in packages {
                if package.trim().is_empty() {
                    anyhow::bail!("Package name cannot be empty");
                }
                self.validate_package_spec(package)?;
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn validate_safe_name(&self, name: &str, context: &str) -> anyhow::Result<()> {
        validate_safe_name_common(name, context)
    }

    #[allow(dead_code)]
    pub fn validate_package_name(&self, package: &str) -> anyhow::Result<()> {
        if package.contains('\0') || package.contains("..") || package.contains('\\') {
            anyhow::bail!("Package name contains unsafe characters: '{}'", package);
        }

        if let Some(package_part) = package.strip_prefix('@') {
            if package_part.is_empty() {
                anyhow::bail!("Invalid online package name format: '{}'", package);
            }
            if package_part.contains("..")
                || package_part.contains('\\')
                || package_part.contains('\0')
            {
                anyhow::bail!("Invalid online package name: '{}'", package);
            }
        } else {
            self.validate_safe_name(package, "Package name")?;
        }

        if package.len() > 100 {
            anyhow::bail!(
                "Package name is too long (max 100 characters): '{}'",
                package
            );
        }

        Ok(())
    }

    pub fn validate_package_spec(&self, package: &str) -> anyhow::Result<()> {
        validate_dependency_spec_common(package)
    }
}
