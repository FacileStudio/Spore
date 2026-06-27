use anyhow::{Context, Result};

use crate::config::AppConfig;
use crate::dependency::registry::{LocalPackageRegistry, RemotePackageRegistry};
use crate::dependency::{
    DependencyRef, DependencyResolver, DependencySpec, ResolutionContext, ResolutionStrategy,
};
use crate::downloader::get_spore_space_url;
use crate::project::Project;

pub(super) async fn create_resolver(
    project: &Project,
    strategy: ResolutionStrategy,
) -> Result<DependencyResolver> {
    let context = ResolutionContext {
        strategy,
        allow_prerelease: false,
        max_depth: 50,
        include_dev: false,
        include_optional: true,
        platform: Some(std::env::consts::OS.to_string()),
        arch: Some(std::env::consts::ARCH.to_string()),
        environment: Some("production".to_string()),
    };

    let mut local_registry = LocalPackageRegistry::new(project.root.join("packages"));
    local_registry
        .discover_packages()
        .await
        .context("Failed to discover local packages")?;

    let remote_registry = RemotePackageRegistry::new(get_spore_space_url())
        .context("Failed to create remote package registry")?;
    let cache_dir = project.root.join(".spore").join("cache");

    Ok(DependencyResolver::new(
        context,
        local_registry,
        remote_registry,
        cache_dir,
    ))
}

pub(super) fn detect_current_app(
    current_dir: &std::path::Path,
    project: &Project,
) -> Result<String> {
    let relative_path = current_dir.strip_prefix(&project.root)?;

    if let Some(app_segment) = relative_path.components().nth(1) {
        if relative_path.starts_with("apps") {
            return Ok(app_segment.as_os_str().to_string_lossy().to_string());
        }
    }

    let app_names = project.get_app_names();

    if app_names.is_empty() {
        anyhow::bail!("No apps found in this project");
    }

    if app_names.len() == 1 {
        return Ok(app_names.into_iter().next().unwrap());
    }

    let selection = inquire::Select::new("Select target app:", app_names)
        .with_vim_mode(true)
        .with_help_message("Use arrow keys or j/k to navigate, Enter to select, Esc to cancel")
        .prompt()
        .context("Failed to select app")?;

    Ok(selection)
}

pub(super) fn parse_package_spec(spec: &str, dev: bool, optional: bool) -> Result<DependencySpec> {
    DependencyRef::parse(spec)?.to_dependency_spec(dev, optional)
}

pub(super) fn parse_resolution_strategy(strategy: Option<&str>) -> Result<ResolutionStrategy> {
    match strategy {
        Some("latest") => Ok(ResolutionStrategy::Latest),
        Some("strict") => Ok(ResolutionStrategy::Strict),
        Some("compatible") => Ok(ResolutionStrategy::Compatible),
        Some("conservative") => Ok(ResolutionStrategy::Conservative),
        Some(unknown) => anyhow::bail!("Unknown resolution strategy: {}", unknown),
        None => Ok(ResolutionStrategy::Compatible),
    }
}

pub(super) fn get_app_dependencies(
    project: &Project,
    app_name: &str,
) -> Result<Vec<DependencySpec>> {
    let raw_deps = project.get_app_dependencies(app_name);
    let mut dep_specs = Vec::new();

    for dep_str in raw_deps {
        let dep_spec = parse_package_spec(&dep_str, false, false)?;
        dep_specs.push(dep_spec);
    }

    Ok(dep_specs)
}

pub(super) fn update_app_config(
    project: &Project,
    app_name: &str,
    dependency_ref: &DependencyRef,
) -> Result<()> {
    let app_config_path = project.root.join("apps").join(app_name).join("app.yml");
    if !app_config_path.exists() {
        anyhow::bail!(
            "App configuration not found at '{}'",
            app_config_path.display()
        );
    }

    if matches!(
        dependency_ref.source,
        crate::dependency::types::PackageSource::Local
    ) && !project.packages.contains_key(&dependency_ref.name)
    {
        anyhow::bail!(
            "Local package '{}' does not exist in this project",
            dependency_ref.name
        );
    }

    let mut app_config = AppConfig::from_file(&app_config_path)?;
    let storage_spec = dependency_ref.storage_spec_with_default("latest");

    if app_config.packages.is_none() {
        app_config.packages = Some(Vec::new());
    }

    let packages = app_config.packages.as_mut().unwrap();
    packages.retain(|existing| {
        DependencyRef::parse(existing)
            .map(|dep| dep.name != dependency_ref.name)
            .unwrap_or(true)
    });
    packages.push(storage_spec);

    let yaml_content = serde_yaml::to_string(&app_config)?;
    std::fs::write(&app_config_path, yaml_content)
        .with_context(|| format!("Failed to write '{}'", app_config_path.display()))?;

    Ok(())
}

pub(super) fn print_resolution_summary(resolution: &crate::dependency::types::ResolutionResult) {
    println!("\n📊 Resolution Summary:");
    println!(
        "  Packages resolved: {}",
        resolution.resolved_packages.len()
    );

    if !resolution.conflicts.is_empty() {
        println!("  Conflicts resolved: {}", resolution.conflicts.len());
    }

    if !resolution.warnings.is_empty() {
        println!("  Warnings: {}", resolution.warnings.len());
        for warning in &resolution.warnings {
            println!("    ⚠️  {}", warning);
        }
    }
}
