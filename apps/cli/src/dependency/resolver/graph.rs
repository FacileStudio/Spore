use std::collections::{HashMap, HashSet, VecDeque};

use crate::dependency::error::{ResolutionError, ResolutionResult};
use crate::dependency::types::{PackageId, PackageVersion};

use super::DependencyResolver;

impl DependencyResolver {
    pub(crate) fn topological_sort(
        &self,
        resolution: &HashMap<PackageId, PackageVersion>,
    ) -> ResolutionResult<Vec<PackageId>> {
        let mut in_degree = HashMap::new();
        let mut graph_edges = HashMap::new();
        let mut all_nodes = HashSet::new();

        for (package_id, package_version) in resolution {
            all_nodes.insert(package_id.clone());
            in_degree.entry(package_id.clone()).or_insert(0);

            let deps = package_version.get_applicable_dependencies(&self.context);
            let dep_ids: Vec<_> = deps.iter().map(|d| d.id.clone()).collect();
            graph_edges.insert(package_id.clone(), dep_ids.clone());

            for dep_id in dep_ids {
                all_nodes.insert(dep_id.clone());
                *in_degree.entry(dep_id).or_insert(0) += 1;
            }
        }

        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        for (node, degree) in &in_degree {
            if *degree == 0 {
                queue.push_back(node.clone());
            }
        }

        while let Some(node) = queue.pop_front() {
            result.push(node.clone());

            if let Some(neighbors) = graph_edges.get(&node) {
                for neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        if result.len() != all_nodes.len() {
            let remaining: Vec<_> = all_nodes.iter().filter(|id| !result.contains(id)).collect();
            return Err(ResolutionError::circular_dependency(
                remaining.into_iter().cloned().collect(),
            ));
        }

        Ok(result)
    }

    pub(crate) fn detect_cycles(
        &self,
        resolution: &HashMap<PackageId, PackageVersion>,
    ) -> ResolutionResult<()> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for package_id in resolution.keys() {
            if !visited.contains(package_id) {
                if let Some(cycle) = self.dfs_cycle_detect(
                    package_id,
                    resolution,
                    &mut visited,
                    &mut rec_stack,
                    &mut path,
                ) {
                    return Err(ResolutionError::circular_dependency(cycle));
                }
            }
        }

        Ok(())
    }

    fn dfs_cycle_detect(
        &self,
        node: &PackageId,
        resolution: &HashMap<PackageId, PackageVersion>,
        visited: &mut HashSet<PackageId>,
        rec_stack: &mut HashSet<PackageId>,
        path: &mut Vec<PackageId>,
    ) -> Option<Vec<PackageId>> {
        visited.insert(node.clone());
        rec_stack.insert(node.clone());
        path.push(node.clone());

        if let Some(package_version) = resolution.get(node) {
            let deps = package_version.get_applicable_dependencies(&self.context);

            for dep in deps {
                if !visited.contains(&dep.id) {
                    if let Some(cycle) =
                        self.dfs_cycle_detect(&dep.id, resolution, visited, rec_stack, path)
                    {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(&dep.id) {
                    if let Some(cycle_start) = path.iter().position(|id| id == &dep.id) {
                        let mut cycle = path[cycle_start..].to_vec();
                        cycle.push(dep.id.clone());
                        return Some(cycle);
                    }
                }
            }
        }

        rec_stack.remove(node);
        path.pop();
        None
    }
}
