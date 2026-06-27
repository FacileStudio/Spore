use anyhow::Result;
use console::style;
use std::collections::HashMap;

use crate::dependency::{DependencyRef, ResolutionStrategy};
use crate::project::Project;

use super::shared::{create_resolver, get_app_dependencies};

pub(super) async fn deps_check() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = Project::find_and_load(&current_dir)?;

    println!("🔍 Analyzing project dependencies...");

    let mut issues_found = 0;
    let app_names = project.get_app_names();

    for app_name in app_names {
        println!("\n📱 Checking app '{}':", style(&app_name).green());

        match analyze_app_dependencies(&project, &app_name).await {
            Ok(analysis) => {
                if analysis.has_issues() {
                    issues_found += 1;
                    print_dependency_analysis(&analysis);
                } else {
                    println!("  ✅ No issues found");
                }
            }
            Err(error) => {
                println!("  ❌ Analysis failed: {}", error);
                issues_found += 1;
            }
        }
    }

    if issues_found == 0 {
        println!("\n🎉 All dependencies look good!");
    } else {
        println!("\n⚠️  Found issues in {} app(s)", issues_found);
    }

    Ok(())
}

pub(super) async fn deps_outdated(app_name: Option<&str>) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = Project::find_and_load(&current_dir)?;

    println!("🔍 Checking for outdated dependencies...");

    if let Some(app) = app_name {
        check_outdated_dependencies(&project, app).await?;
    } else {
        let app_names = project.get_app_names();

        for (index, app_name) in app_names.iter().enumerate() {
            if index > 0 {
                println!();
            }
            println!("📱 App: {}", style(app_name).green().bold());
            check_outdated_dependencies(&project, app_name).await?;
        }
    }

    Ok(())
}

pub(super) async fn deps_sync() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = Project::find_and_load(&current_dir)?;

    println!("🔄 Synchronizing dependencies across all apps...");

    let common_deps = find_common_dependencies(&project).await?;

    if common_deps.is_empty() {
        println!("No common dependencies found across apps.");
        return Ok(());
    }

    println!("Found {} common dependencies:", common_deps.len());

    for (package_name, versions) in common_deps {
        println!("\n📦 Package: {}", style(&package_name).cyan());

        if versions.len() > 1 {
            println!("  ⚠️  Version conflict detected:");
            for (app_name, version) in &versions {
                println!(
                    "    {} uses {}",
                    style(app_name).green(),
                    style(version).yellow()
                );
            }

            if let Some(suggested) = find_latest_compatible_version(&versions) {
                println!("  💡 Suggested version: {}", style(suggested).green());
            }
        } else {
            let (_, version) = versions.first().unwrap();
            println!("  ✅ All apps use version {}", style(version).green());
        }
    }

    Ok(())
}

async fn analyze_app_dependencies(project: &Project, app_name: &str) -> Result<DependencyAnalysis> {
    let mut issues = project.validate_dependency_tree(app_name)?;
    let suggestions = issues
        .iter()
        .map(|issue| {
            if issue.contains("Duplicate dependency") {
                "Remove duplicate entries from app.yml".to_string()
            } else if issue.contains("not found in packages directory") {
                "Create the missing package or remove it from the dependency list".to_string()
            } else {
                "Review the dependency configuration for this app".to_string()
            }
        })
        .collect::<Vec<_>>();

    issues.sort();
    issues.dedup();

    Ok(DependencyAnalysis {
        app_name: app_name.to_string(),
        issues,
        suggestions,
    })
}

struct DependencyAnalysis {
    app_name: String,
    issues: Vec<String>,
    suggestions: Vec<String>,
}

impl DependencyAnalysis {
    fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }
}

fn print_dependency_analysis(analysis: &DependencyAnalysis) {
    println!("  App: {}", style(&analysis.app_name).green());

    for issue in &analysis.issues {
        println!("  ❌ {}", issue);
    }

    for suggestion in &analysis.suggestions {
        println!("  💡 {}", suggestion);
    }
}

async fn check_outdated_dependencies(project: &Project, app_name: &str) -> Result<()> {
    let deps = get_app_dependencies(project, app_name)?;

    if deps.is_empty() {
        println!("  No dependencies");
        return Ok(());
    }

    let mut resolver = create_resolver(project, ResolutionStrategy::Compatible).await?;
    let resolution = resolver.resolve_dependencies(deps.clone()).await?;
    let mut outdated = Vec::new();

    for dep in deps {
        if let Some(current) = resolution.resolved_packages.get(&dep.id) {
            let versions = resolver.get_package_info(&dep.id).await?;
            if let Some(latest) = versions
                .iter()
                .max_by(|left, right| left.version.cmp(&right.version))
            {
                if latest.version > current.version {
                    outdated.push((
                        dep.id.name.clone(),
                        current.version.clone(),
                        latest.version.clone(),
                    ));
                }
            }
        }
    }

    if outdated.is_empty() {
        println!("  All dependencies are up to date");
    } else {
        for (name, current, latest) in outdated {
            println!(
                "  ⬆️  {}: {} -> {}",
                style(name).cyan(),
                style(current).yellow(),
                style(latest).green()
            );
        }
    }

    Ok(())
}

async fn find_common_dependencies(
    project: &Project,
) -> Result<HashMap<String, Vec<(String, String)>>> {
    let mut common_deps: HashMap<String, Vec<(String, String)>> = HashMap::new();

    for app_name in project.get_app_names() {
        for raw_dep in project.get_app_dependencies(&app_name) {
            let dep_ref = DependencyRef::parse(&raw_dep)?;
            let version = dep_ref
                .version_req_text
                .clone()
                .unwrap_or_else(|| "latest".to_string());
            common_deps
                .entry(dep_ref.name)
                .or_default()
                .push((app_name.clone(), version));
        }
    }

    common_deps.retain(|_, entries| entries.len() > 1);

    Ok(common_deps)
}

fn find_latest_compatible_version(versions: &[(String, String)]) -> Option<&str> {
    if let Some((_, version)) = versions.iter().find(|(_, version)| version == "latest") {
        return Some(version.as_str());
    }

    versions
        .iter()
        .filter_map(|(_, version)| {
            semver::Version::parse(version)
                .ok()
                .map(|parsed| (parsed, version.as_str()))
        })
        .max_by(|(left, _), (right, _)| left.cmp(right))
        .map(|(_, version)| version)
        .or_else(|| versions.first().map(|(_, version)| version.as_str()))
}
