use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigVariable {
    Simple(String),
    Complex {
        value: String,
        description: Option<String>,
    },
}

impl ConfigVariable {
    #[allow(dead_code)]
    pub fn get_value(&self) -> &str {
        match self {
            ConfigVariable::Simple(value) => value,
            ConfigVariable::Complex { value, .. } => value,
        }
    }

    #[allow(dead_code)]
    pub fn get_description(&self) -> Option<&str> {
        match self {
            ConfigVariable::Simple(_) => None,
            ConfigVariable::Complex { description, .. } => description.as_deref(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ConfigType {
    Spore,
    Package,
    App,
    #[allow(dead_code)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SporeConfig {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "tsAlias")]
    pub ts_alias: Option<TsAlias>,
    pub apps: Option<HashMap<String, AppDependencies>>,
    pub scripts: Option<HashMap<String, String>>,
    pub variables: Option<HashMap<String, ConfigVariable>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TsAlias {
    Boolean(bool),
    String(String),
}

impl TsAlias {
    #[allow(dead_code)]
    pub fn get_alias(&self) -> Option<String> {
        match self {
            TsAlias::Boolean(true) => Some("#".to_string()),
            TsAlias::Boolean(false) => None,
            TsAlias::String(alias) => Some(alias.clone()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppDependencies {
    List(Vec<String>),
    Object {
        #[serde(rename = "tsAlias")]
        ts_alias: Option<TsAlias>,
        packages: Option<Vec<String>>,
    },
}

impl AppDependencies {
    #[allow(dead_code)]
    pub fn get_packages(&self) -> Vec<String> {
        match self {
            AppDependencies::List(packages) => packages.clone(),
            AppDependencies::Object { packages, .. } => packages.clone().unwrap_or_default(),
        }
    }

    #[allow(dead_code)]
    pub fn get_ts_alias(&self) -> Option<&TsAlias> {
        match self {
            AppDependencies::List(_) => None,
            AppDependencies::Object { ts_alias, .. } => ts_alias.as_ref(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageConfig {
    pub name: String,
    pub team: Option<String>,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub scripts: Option<HashMap<String, String>>,
    pub dependencies: Option<Vec<String>>,
    pub dev_dependencies: Option<Vec<String>>,
    pub optional_dependencies: Option<Vec<String>>,
    pub peer_dependencies: Option<Vec<String>>,
    pub exports: Option<HashMap<String, String>>,
    pub features: Option<Vec<String>>,
    pub variables: Option<HashMap<String, ConfigVariable>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "tsAlias")]
    pub ts_alias: Option<TsAlias>,
    pub packages: Option<Vec<String>>,
    pub scripts: Option<HashMap<String, String>>,
    pub variables: Option<HashMap<String, ConfigVariable>>,
}
