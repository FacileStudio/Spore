use anyhow::{Context, Result};
use console::style;

use crate::dependency::{DependencyRef, ResolutionStrategy};
use crate::project::Project;
use crate::typescript::TypeScriptManager;
use crate::validation::{sanitize_input, validate_app_name};

use super::shared::{
    create_resolver, detect_current_app, parse_resolution_strategy, print_resolution_summary,
    update_app_config,
};

pub(super) async fn deps_add(
    package_spec: &str,
    app_name: Option<&str>,
    dev: bool,
    optional: bool,
) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = Project::find_and_load(&current_dir)?;
    let sanitized_spec = sanitize_input(package_spec);
    let dependency_ref = DependencyRef::parse(&sanitized_spec)?;

    let target_app = if let Some(app) = app_name {
        let sanitized_app = sanitize_input(app);
        validate_app_name(&sanitized_app)?;
        sanitized_app
    } else {
        detect_current_app(&current_dir, &project)?
    };

    println!(
        "📦 Adding dependency '{}' to app '{}'",
        style(&sanitized_spec).cyan(),
        style(&target_app).green()
    );

    if dev || optional {
        println!(
            "ℹ️  App configuration stores dependencies in a single list. '--dev' and '--optional' are treated as normal dependencies."
        );
    }

    update_app_config(&project, &target_app, &dependency_ref)?;

    let mut updated_project = Project::find_and_load(&project.root)?;
    let resolution = updated_project
        .resolve_dependencies(&target_app, false, None)
        .await
        .with_context(|| format!("Failed to resolve dependencies for app '{}'", target_app))?;

    println!("🔗 Linking packages...");
    updated_project
        .install_dependencies(&target_app, &resolution, false)
        .await
        .with_context(|| format!("Failed to install dependencies for app '{}'", target_app))?;

    if let Some(alias) = updated_project.get_app_ts_alias(&target_app) {
        TypeScriptManager::new(&updated_project)
            .setup_tsconfig_alias(&target_app, &alias)
            .with_context(|| format!("Failed to update tsconfig for app '{}'", target_app))?;
    }

    println!(
        "✅ Successfully added dependency '{}' to app '{}'",
        style(package_spec).cyan(),
        style(&target_app).green()
    );

    print_resolution_summary(&resolution);

    Ok(())
}

pub(super) async fn deps_resolve(
    strategy: Option<&str>,
    dry_run: bool,
    app_name: Option<&str>,
) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = Project::find_and_load(&current_dir)?;
    let resolution_strategy = parse_resolution_strategy(strategy)?;

    if let Some(app) = app_name {
        resolve_app_dependencies(&project, app, resolution_strategy, dry_run).await?;
    } else {
        let app_names = project.get_app_names();

        for app_name in app_names {
            if dry_run {
                println!(
                    "\n📱 Would resolve dependencies for app: {}",
                    style(&app_name).green()
                );
            } else {
                println!(
                    "\n📱 Resolving dependencies for app: {}",
                    style(&app_name).green()
                );
            }

            resolve_app_dependencies(&project, &app_name, resolution_strategy.clone(), dry_run)
                .await?;
        }
    }

    Ok(())
}

async fn resolve_app_dependencies(
    project: &Project,
    app_name: &str,
    strategy: ResolutionStrategy,
    dry_run: bool,
) -> Result<()> {
    let mut working_project = Project::find_and_load(&project.root)?;
    let deps = working_project.get_app_dependency_specs(app_name, false)?;

    if deps.is_empty() {
        println!("  No dependencies to resolve");
        return Ok(());
    }

    let mut resolver = create_resolver(project, strategy).await?;

    match resolver.resolve_dependencies(deps).await {
        Ok(resolution) => {
            if dry_run {
                println!(
                    "  Would resolve {} packages",
                    resolution.resolved_packages.len()
                );
            } else {
                println!(
                    "  ✅ Resolved {} packages",
                    resolution.resolved_packages.len()
                );

                working_project
                    .install_dependencies(app_name, &resolution, false)
                    .await
                    .with_context(|| {
                        format!("Failed to install dependencies for app '{}'", app_name)
                    })?;

                if let Some(alias) = working_project.get_app_ts_alias(app_name) {
                    TypeScriptManager::new(&working_project)
                        .setup_tsconfig_alias(app_name, &alias)
                        .with_context(|| {
                            format!("Failed to update tsconfig for app '{}'", app_name)
                        })?;
                }
            }

            if !resolution.warnings.is_empty() {
                for warning in &resolution.warnings {
                    println!("    ⚠️  {}", warning);
                }
            }
        }
        Err(e) => {
            println!("  ❌ Resolution failed: {}", e);
        }
    }

    Ok(())
}
