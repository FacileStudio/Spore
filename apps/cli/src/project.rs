mod context;
mod dependencies;
mod discovery;
mod filesystem;

use std::collections::HashMap;
use std::path::PathBuf;

use crate::config::{AppConfig, SporeConfig, PackageConfig};
use crate::dependency::DependencyResolver;
use crate::variables::VariableContext;

pub struct Project {
    pub root: PathBuf,
    pub config: SporeConfig,
    pub packages: HashMap<String, PackageConfig>,
    pub package_paths: HashMap<String, PathBuf>,
    pub apps: HashMap<String, AppConfig>,
    dependency_resolver: Option<DependencyResolver>,
    pub variable_context: VariableContext,
}
