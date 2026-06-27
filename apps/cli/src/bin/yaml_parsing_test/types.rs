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
    pub fn get_value(&self) -> &str {
        match self {
            ConfigVariable::Simple(value) => value,
            ConfigVariable::Complex { value, .. } => value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TsAlias {
    Boolean(bool),
    String(String),
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
