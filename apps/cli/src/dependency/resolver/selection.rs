use std::collections::{HashMap, HashSet};

use crate::dependency::error::{ResolutionError, ResolutionResult};
use crate::dependency::types::{
    DependencyConflict, DependencyGraph, PackageId, PackageVersion, ResolutionStrategy,
};

use super::DependencyResolver;

impl DependencyResolver {
    pub(crate) fn resolution_phase(
        &mut self,
        graph: &mut DependencyGraph,
    ) -> ResolutionResult<HashMap<PackageId, PackageVersion>> {
        let mut resolution = HashMap::new();
        let sorted_packages = self.dependency_order_for_resolution(&graph.packages)?;

        for package_id in sorted_packages {
            let selected_version = self.select_version(graph, &package_id)?;
            resolution.insert(package_id.clone(), selected_version);
        }

        self.detect_cycles(&resolution)?;
        Ok(resolution)
    }

    pub(crate) fn select_version(
        &self,
        graph: &DependencyGraph,
        package_id: &PackageId,
    ) -> ResolutionResult<PackageVersion> {
        let versions = graph.packages.get(package_id).ok_or_else(|| {
            ResolutionError::package_not_found(
                package_id.clone(),
                vec!["Dependency graph".to_string()],
                vec![],
            )
        })?;

        let empty_constraints = Vec::new();
        let constraints = graph
            .constraints
            .get(package_id)
            .unwrap_or(&empty_constraints);

        let compatible_versions: Vec<_> = versions
            .iter()
            .filter(|v| {
                constraints.iter().all(|req| req.matches(&v.version))
                    && (v.version.pre.is_empty() || self.context.allow_prerelease)
            })
            .collect();

        if compatible_versions.is_empty() {
            let conflicting_requirements: Vec<_> = constraints
                .iter()
                .map(|req| (req.clone(), PackageId::local("unknown")))
                .collect();

            return Err(ResolutionError::version_conflict(
                package_id.clone(),
                conflicting_requirements,
                Some(
                    "Try relaxing version constraints or updating to compatible versions"
                        .to_string(),
                ),
            ));
        }

        let selected = match self.context.strategy {
            ResolutionStrategy::Latest => compatible_versions
                .iter()
                .max_by(|a, b| a.version.cmp(&b.version))
                .copied(),
            ResolutionStrategy::Strict => {
                if constraints.len() == 1
                    && constraints[0]
                        .to_string()
                        .chars()
                        .all(|c| c.is_ascii_digit() || c == '.')
                {
                    compatible_versions.first().copied()
                } else {
                    return Err(ResolutionError::configuration_error(
                        "Strict mode requires exact version specifications",
                        Some(format!("package: {}", package_id.name)),
                    ));
                }
            }
            ResolutionStrategy::Compatible => self.select_compatible_version(&compatible_versions),
            ResolutionStrategy::Conservative => compatible_versions
                .iter()
                .min_by(|a, b| a.version.cmp(&b.version))
                .copied(),
        }
        .ok_or_else(|| {
            ResolutionError::configuration_error(
                "No version could be selected with current strategy",
                Some(format!(
                    "package: {}, strategy: {:?}",
                    package_id.name, self.context.strategy
                )),
            )
        })?;

        Ok(selected.clone())
    }

    fn select_compatible_version<'a>(
        &self,
        versions: &[&'a PackageVersion],
    ) -> Option<&'a PackageVersion> {
        let mut version_groups: HashMap<(u64, u64), Vec<&'a PackageVersion>> = HashMap::new();

        for version in versions {
            let key = (version.version.major, version.version.minor);
            version_groups.entry(key).or_default().push(version);
        }

        let latest_group = version_groups.iter().max_by_key(|(key, _)| *key)?;
        latest_group
            .1
            .iter()
            .max_by(|a, b| a.version.patch.cmp(&b.version.patch))
            .copied()
    }

    pub(crate) fn dependency_order_for_resolution(
        &self,
        packages: &HashMap<PackageId, Vec<PackageVersion>>,
    ) -> ResolutionResult<Vec<PackageId>> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();

        for package_id in packages.keys() {
            if !visited.contains(package_id) {
                self.visit_for_resolution_order(package_id, packages, &mut visited, &mut result)?;
            }
        }

        Ok(result)
    }

    fn visit_for_resolution_order(
        &self,
        package_id: &PackageId,
        packages: &HashMap<PackageId, Vec<PackageVersion>>,
        visited: &mut HashSet<PackageId>,
        result: &mut Vec<PackageId>,
    ) -> ResolutionResult<()> {
        if visited.contains(package_id) {
            return Ok(());
        }

        visited.insert(package_id.clone());

        if let Some(versions) = packages.get(package_id) {
            if let Some(version) = versions.first() {
                let deps = version.get_applicable_dependencies(&self.context);
                for dep in deps {
                    if packages.contains_key(&dep.id) {
                        self.visit_for_resolution_order(&dep.id, packages, visited, result)?;
                    }
                }
            }
        }

        result.push(package_id.clone());
        Ok(())
    }

    pub(crate) fn detect_remaining_conflicts(
        &self,
        _resolution: &HashMap<PackageId, PackageVersion>,
    ) -> Vec<DependencyConflict> {
        Vec::new()
    }

    pub(crate) fn generate_warnings(
        &self,
        resolution: &HashMap<PackageId, PackageVersion>,
    ) -> Vec<String> {
        let mut warnings = Vec::new();

        for (package_id, package_version) in resolution {
            if !package_version.version.pre.is_empty() {
                warnings.push(format!(
                    "Using prerelease version {} for package '{}'",
                    package_version.version, package_id.name
                ));
            }
        }

        let mut major_versions: HashMap<String, HashSet<u64>> = HashMap::new();
        for (package_id, package_version) in resolution {
            major_versions
                .entry(package_id.name.clone())
                .or_default()
                .insert(package_version.version.major);
        }

        for (package_name, majors) in major_versions {
            if majors.len() > 1 {
                warnings.push(format!(
                    "Multiple major versions of '{}' in dependency tree: {}",
                    package_name,
                    majors
                        .iter()
                        .map(|m| m.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        }

        warnings
    }
}
