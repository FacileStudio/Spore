use anyhow::Result;
use console::style;
use std::collections::HashSet;

use crate::dependency::{PackageId, ResolutionContext, ResolutionStrategy};
use crate::project::Project;

use super::shared::{create_resolver, detect_current_app, get_app_dependencies};

pub(super) async fn deps_list(
    app_name: Option<&str>,
    tree: bool,
    depth: Option<usize>,
) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = Project::find_and_load(&current_dir)?;

    if let Some(app) = app_name {
        list_app_dependencies(&project, app, tree, depth).await?;
    } else {
        let app_names = project.get_app_names();

        if app_names.is_empty() {
            println!("No apps found in this project.");
            return Ok(());
        }

        for (index, app_name) in app_names.iter().enumerate() {
            if index > 0 {
                println!();
            }
            println!("📱 App: {}", style(app_name).green().bold());
            list_app_dependencies(&project, app_name, tree, depth).await?;
        }
    }

    Ok(())
}

pub(super) async fn deps_tree(app_name: Option<&str>, depth: Option<usize>) -> Result<()> {
    deps_list(app_name, true, depth).await
}

pub(super) async fn deps_why(package_name: &str, app_name: Option<&str>) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = Project::find_and_load(&current_dir)?;

    let target_app = if let Some(app) = app_name {
        app.to_string()
    } else {
        detect_current_app(&current_dir, &project)?
    };

    println!(
        "🔍 Explaining why '{}' is included in app '{}'",
        style(package_name).cyan(),
        style(&target_app).green()
    );

    explain_dependency_inclusion(&project, &target_app, package_name).await
}

async fn list_app_dependencies(
    project: &Project,
    app_name: &str,
    tree: bool,
    depth: Option<usize>,
) -> Result<()> {
    let deps = get_app_dependencies(project, app_name)?;

    if deps.is_empty() {
        println!("  No dependencies");
        return Ok(());
    }

    if tree {
        let mut resolver = create_resolver(project, ResolutionStrategy::Compatible).await?;
        let resolution = resolver.resolve_dependencies(deps).await?;
        print_dependency_tree(project, app_name, &resolution, depth);
    } else {
        for dep in deps {
            let version_str = if dep.version_req.to_string() == "*" {
                "latest".to_string()
            } else {
                dep.version_req.to_string()
            };

            println!(
                "  📦 {} @ {}",
                style(&dep.id.name).cyan(),
                style(version_str).yellow()
            );
        }
    }

    Ok(())
}

fn print_dependency_tree(
    project: &Project,
    app_name: &str,
    resolution: &crate::dependency::types::ResolutionResult,
    max_depth: Option<usize>,
) {
    println!("  📦 Dependency tree:");

    let root_deps = match project.get_app_dependency_specs(app_name, false) {
        Ok(deps) => deps,
        Err(_) => return,
    };

    let mut visited = HashSet::new();
    for root_dep in root_deps {
        print_dependency_branch(resolution, &root_dep.id, 0, max_depth, &mut visited);
    }
}

fn print_dependency_branch(
    resolution: &crate::dependency::types::ResolutionResult,
    package_id: &PackageId,
    depth: usize,
    max_depth: Option<usize>,
    visited: &mut HashSet<PackageId>,
) {
    if max_depth.is_some_and(|limit| depth > limit) {
        return;
    }

    let prefix = "  ".repeat(depth + 1);

    if let Some(package) = resolution.resolved_packages.get(package_id) {
        println!(
            "{}{} @ {}",
            prefix,
            style(&package.id.name).cyan(),
            style(&package.version).yellow()
        );

        if !visited.insert(package_id.clone()) {
            return;
        }

        for dependency in package.get_applicable_dependencies(&ResolutionContext::default()) {
            print_dependency_branch(resolution, &dependency.id, depth + 1, max_depth, visited);
        }

        visited.remove(package_id);
    }
}

async fn explain_dependency_inclusion(
    project: &Project,
    app_name: &str,
    package_name: &str,
) -> Result<()> {
    let deps = get_app_dependencies(project, app_name)?;
    let mut resolver = create_resolver(project, ResolutionStrategy::Compatible).await?;
    let resolution = resolver.resolve_dependencies(deps.clone()).await?;

    for dep in &deps {
        if dep.id.name == package_name {
            println!(
                "  📦 '{}' is a direct dependency of app '{}'",
                package_name, app_name
            );
            return Ok(());
        }
    }

    for dep in &deps {
        let mut visited = HashSet::new();
        if let Some(path) = find_dependency_path(&resolution, &dep.id, package_name, &mut visited) {
            println!("  📦 '{}' is included through:", package_name);
            for (index, item) in path.iter().enumerate() {
                if index == 0 {
                    println!("    {}", style(item).green());
                } else {
                    println!("    └── {}", style(item).cyan());
                }
            }
            return Ok(());
        }
    }

    println!(
        "  📦 '{}' is not present in the resolved dependency graph for app '{}'",
        package_name, app_name
    );

    Ok(())
}

fn find_dependency_path(
    resolution: &crate::dependency::types::ResolutionResult,
    current: &PackageId,
    target_name: &str,
    visited: &mut HashSet<PackageId>,
) -> Option<Vec<String>> {
    if !visited.insert(current.clone()) {
        return None;
    }

    if current.name == target_name {
        visited.remove(current);
        return Some(vec![current.name.clone()]);
    }

    let result = resolution
        .resolved_packages
        .get(current)
        .and_then(|package| {
            for dependency in package.get_applicable_dependencies(&ResolutionContext::default()) {
                if let Some(mut path) =
                    find_dependency_path(resolution, &dependency.id, target_name, visited)
                {
                    let mut result = vec![current.name.clone()];
                    result.append(&mut path);
                    return Some(result);
                }
            }
            None
        });

    visited.remove(current);
    result
}
