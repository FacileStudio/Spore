use crate::config::{AppConfig, SporeConfig, PackageConfig};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct VariableContext {
    built_in: HashMap<String, String>,
    project: HashMap<String, String>,
    app: HashMap<String, String>,
    package: HashMap<String, String>,
}

impl VariableContext {
    pub fn new(project_name: &str, project_root: &std::path::Path) -> Self {
        let mut built_in = HashMap::new();
        built_in.insert("project_name".to_string(), project_name.to_string());
        built_in.insert(
            "project_root".to_string(),
            project_root.display().to_string(),
        );
        built_in.insert(
            "timestamp".to_string(),
            chrono::Utc::now().timestamp().to_string(),
        );
        built_in.insert(
            "date".to_string(),
            chrono::Utc::now().format("%Y-%m-%d").to_string(),
        );
        built_in.insert(
            "year".to_string(),
            chrono::Utc::now().format("%Y").to_string(),
        );

        Self {
            built_in,
            project: HashMap::new(),
            app: HashMap::new(),
            package: HashMap::new(),
        }
    }

    pub fn with_project_variables(mut self, config: &SporeConfig) -> Self {
        if let Some(variables) = &config.variables {
            for (key, var) in variables {
                self.project
                    .insert(key.clone(), var.get_value().to_string());
            }
        }
        self
    }

    pub fn with_app_variables(mut self, config: &AppConfig) -> Self {
        if let Some(variables) = &config.variables {
            for (key, var) in variables {
                self.app.insert(key.clone(), var.get_value().to_string());
            }
        }
        self
    }

    pub fn with_package_variables(mut self, config: &PackageConfig) -> Self {
        if let Some(variables) = &config.variables {
            for (key, var) in variables {
                self.package
                    .insert(key.clone(), var.get_value().to_string());
            }
        }
        self
    }

    pub fn get_variable(&self, key: &str) -> Option<&str> {
        self.built_in
            .get(key)
            .or_else(|| self.package.get(key))
            .or_else(|| self.app.get(key))
            .or_else(|| self.project.get(key))
            .map(|s| s.as_str())
    }

    pub fn list_variables(&self) -> Vec<VariableInfo> {
        let mut variables = Vec::new();
        let mut all_names = HashSet::new();
        all_names.extend(self.built_in.keys());
        all_names.extend(self.project.keys());
        all_names.extend(self.app.keys());
        all_names.extend(self.package.keys());

        for name in all_names {
            let source = if self.built_in.contains_key(name) {
                VariableSource::BuiltIn
            } else if self.package.contains_key(name) {
                VariableSource::Package
            } else if self.app.contains_key(name) {
                VariableSource::App
            } else {
                VariableSource::Project
            };

            variables.push(VariableInfo {
                name: name.clone(),
                value: self.get_variable(name).unwrap_or("").to_string(),
                source,
            });
        }

        variables.sort_by(|a, b| a.name.cmp(&b.name));
        variables
    }
}

#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub value: String,
    pub source: VariableSource,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableSource {
    BuiltIn,
    Project,
    App,
    Package,
}

impl std::fmt::Display for VariableSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableSource::BuiltIn => write!(f, "built-in"),
            VariableSource::Project => write!(f, "project"),
            VariableSource::App => write!(f, "app"),
            VariableSource::Package => write!(f, "package"),
        }
    }
}
