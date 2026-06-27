use semver::{Version, VersionReq};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::dependency::error::{ResolutionError, ResolutionResult};
use crate::dependency::registry::PackageRegistry;
use crate::dependency::types::{
    DependencyGraph, DependencySpec, PackageId, PackageVersion, ResolutionRequest,
    ResolutionResult as TypesResolutionResult,
};

use super::utils::edit_distance;
use super::DependencyResolver;

impl DependencyResolver {
    pub async fn resolve_dependencies(
        &mut self,
        root_dependencies: Vec<DependencySpec>,
    ) -> ResolutionResult<TypesResolutionResult> {
        let request = ResolutionRequest {
            dependencies: root_dependencies.clone(),
            context: self.context.clone(),
            overrides: HashMap::new(),
            excludes: Vec::new(),
        };

        if let Some(cached_result) = self.cache.get_cached_resolution(&request).await {
            return Ok(cached_result);
        }

        let mut graph = DependencyGraph::new();
        self.discover_phase(&mut graph, &root_dependencies).await?;
        self.constraint_phase(&mut graph, &root_dependencies)?;
        let resolution = self.resolution_phase(&mut graph)?;
        let dependency_order = self.topological_sort(&resolution)?;
        let conflicts = self.detect_remaining_conflicts(&resolution);
        let warnings = self.generate_warnings(&resolution);

        let result = TypesResolutionResult {
            resolved_packages: resolution,
            dependency_order,
            conflicts,
            warnings,
            lock_file_hash: None,
        };

        self.cache.cache_resolution(&request, &result).await?;
        Ok(result)
    }

    pub(crate) async fn discover_phase(
        &mut self,
        graph: &mut DependencyGraph,
        root_deps: &[DependencySpec],
    ) -> ResolutionResult<()> {
        let mut queue = VecDeque::from(root_deps.to_vec());
        let mut discovered = HashSet::new();

        self.local_registry.discover_packages().await?;

        while let Some(dep_spec) = queue.pop_front() {
            if !dep_spec.is_applicable(&self.context) {
                continue;
            }

            if discovered.contains(&dep_spec.id) {
                continue;
            }
            discovered.insert(dep_spec.id.clone());

            let versions = self.discover_package_versions(&dep_spec.id).await?;
            if versions.is_empty() {
                return Err(ResolutionError::package_not_found(
                    dep_spec.id.clone(),
                    vec!["Local packages".to_string(), "Remote registry".to_string()],
                    self.find_similar_packages(&dep_spec.id.name).await,
                ));
            }

            graph.packages.insert(dep_spec.id.clone(), versions);

            if let Some(package_versions) = graph.packages.get(&dep_spec.id) {
                for version in package_versions {
                    if dep_spec.version_req.matches(&version.version) {
                        let transitive_deps = version.get_applicable_dependencies(&self.context);
                        for transitive_dep in transitive_deps {
                            if !discovered.contains(&transitive_dep.id) {
                                queue.push_back(transitive_dep.clone());
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub(crate) fn constraint_phase(
        &mut self,
        graph: &mut DependencyGraph,
        root_deps: &[DependencySpec],
    ) -> ResolutionResult<()> {
        let mut work_queue = VecDeque::new();

        for dep_spec in root_deps {
            if dep_spec.is_applicable(&self.context) {
                self.add_constraint(graph, &dep_spec.id, &dep_spec.version_req);
                work_queue.push_back(dep_spec.id.clone());
            }
        }

        while let Some(package_id) = work_queue.pop_front() {
            let versions_clone = graph.packages.get(&package_id).cloned();
            if let Some(versions) = versions_clone {
                for version in versions {
                    if self.version_satisfies_constraints(graph, &package_id, &version.version) {
                        let deps = version.get_applicable_dependencies(&self.context);
                        for dep in deps {
                            let had_constraint = graph.constraints.contains_key(&dep.id);
                            self.add_constraint(graph, &dep.id, &dep.version_req);
                            if !had_constraint {
                                work_queue.push_back(dep.id.clone());
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub(crate) async fn discover_package_versions(
        &self,
        package_id: &PackageId,
    ) -> ResolutionResult<Vec<PackageVersion>> {
        match &package_id.source {
            crate::dependency::types::PackageSource::Local => {
                self.local_registry.list_versions(package_id).await
            }
            crate::dependency::types::PackageSource::Remote { .. } => {
                self.remote_registry.list_versions(package_id).await
            }
        }
    }

    pub(crate) async fn find_similar_packages(&self, query: &str) -> Vec<String> {
        let mut similar = Vec::new();

        if let Ok(local_matches) = self.local_registry.search_packages(query).await {
            similar.extend(local_matches);
        }

        if let Ok(remote_matches) = self.remote_registry.search_packages(query).await {
            similar.extend(remote_matches);
        }

        similar.sort_by(|a, b| {
            let a_distance = edit_distance(query, a);
            let b_distance = edit_distance(query, b);
            a_distance.cmp(&b_distance)
        });

        similar.truncate(5);
        similar
    }

    pub(crate) fn add_constraint(
        &self,
        graph: &mut DependencyGraph,
        package_id: &PackageId,
        version_req: &VersionReq,
    ) {
        graph
            .constraints
            .entry(package_id.clone())
            .or_default()
            .push(version_req.clone());
    }

    pub(crate) fn version_satisfies_constraints(
        &self,
        graph: &DependencyGraph,
        package_id: &PackageId,
        version: &Version,
    ) -> bool {
        if let Some(constraints) = graph.constraints.get(package_id) {
            constraints.iter().all(|req| req.matches(version))
        } else {
            true
        }
    }
}
