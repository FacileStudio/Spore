use crate::variables::VariableContext;

use super::Project;

impl Project {
    pub fn get_app_dependencies(&self, app_name: &str) -> Vec<String> {
        let mut dependencies = Vec::new();

        if let Some(app_deps) = self
            .config
            .apps
            .as_ref()
            .and_then(|apps| apps.get(app_name))
        {
            dependencies.extend(app_deps.get_packages());
        }

        if let Some(app_config) = self.apps.get(app_name) {
            if let Some(packages) = &app_config.packages {
                dependencies.extend(packages.clone());
            }
        }

        dependencies.sort();
        dependencies.dedup();
        dependencies
    }

    pub fn get_app_names(&self) -> Vec<String> {
        let mut names = Vec::new();

        if let Some(apps) = &self.config.apps {
            names.extend(apps.keys().cloned());
        }

        names.extend(self.apps.keys().cloned());
        names.sort();
        names.dedup();
        names
    }

    pub fn get_app_ts_alias(&self, app_name: &str) -> Option<String> {
        if let Some(app_config) = self.apps.get(app_name) {
            if let Some(ts_alias) = &app_config.ts_alias {
                return ts_alias.get_alias();
            }
        }

        if let Some(app_deps) = self
            .config
            .apps
            .as_ref()
            .and_then(|apps| apps.get(app_name))
        {
            if let Some(ts_alias) = app_deps.get_ts_alias() {
                return ts_alias.get_alias();
            }
        }

        self.config
            .ts_alias
            .as_ref()
            .and_then(|ts_alias| ts_alias.get_alias())
    }

    pub fn get_app_variable_context(&self, app_name: &str) -> VariableContext {
        let mut context = self.variable_context.clone();

        if let Some(app_config) = self.apps.get(app_name) {
            context = context.with_app_variables(app_config);
        }

        context
    }

    pub fn get_package_variable_context(&self, package_name: &str) -> VariableContext {
        let mut context = self.variable_context.clone();

        if let Some(package_config) = self.packages.get(package_name) {
            context = context.with_package_variables(package_config);
        }

        context
    }

    pub fn get_full_variable_context(&self, app_name: &str, package_name: &str) -> VariableContext {
        let mut context = self.variable_context.clone();

        if let Some(app_config) = self.apps.get(app_name) {
            context = context.with_app_variables(app_config);
        }

        if let Some(package_config) = self.packages.get(package_name) {
            context = context.with_package_variables(package_config);
        }

        context
    }
}
