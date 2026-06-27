use anyhow::{Context, Result};
use semver::VersionReq;
use std::path::PathBuf;

use crate::validation::validate_package_spec;

use super::types::{DependencySpec, PackageId, PackageSource};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyRef {
    pub name: String,
    pub version_req_text: Option<String>,
    pub source: PackageSource,
}

impl DependencyRef {
    pub fn parse(raw: &str) -> Result<Self> {
        let trimmed = raw.trim();
        let (name, version_req_text) = validate_package_spec(trimmed)
            .with_context(|| format!("Invalid dependency specification '{}'", trimmed))?;

        let source = if name.starts_with('@') {
            PackageSource::Remote {
                registry: "spore-space".to_string(),
            }
        } else {
            PackageSource::Local
        };

        Ok(Self {
            name,
            version_req_text,
            source,
        })
    }

    pub fn from_name_and_version(name: &str, version_req_text: Option<&str>) -> Result<Self> {
        let spec = match version_req_text {
            Some(version) => format!("{}@{}", name, version),
            None => name.to_string(),
        };

        Self::parse(&spec)
    }

    pub fn to_dependency_spec(&self, dev_only: bool, optional: bool) -> Result<DependencySpec> {
        let version_req_text = self.version_req_text.as_deref().unwrap_or("*");
        let parsed_req = if version_req_text == "latest" {
            "*"
        } else {
            version_req_text
        };

        let version_req = VersionReq::parse(parsed_req).with_context(|| {
            format!(
                "Invalid version requirement '{}' for package '{}'",
                version_req_text, self.name
            )
        })?;

        Ok(DependencySpec {
            id: self.package_id(),
            version_req,
            optional,
            dev_only,
            conditions: None,
            features: None,
        })
    }

    pub fn package_id(&self) -> PackageId {
        PackageId {
            name: self.name.clone(),
            source: self.source.clone(),
        }
    }

    pub fn storage_spec_with_default(&self, default_version: &str) -> String {
        let version = self.version_req_text.as_deref().unwrap_or(default_version);
        format!("{}@{}", self.name, version)
    }
}

pub fn install_subpath_for_name(name: &str) -> PathBuf {
    let mut path = PathBuf::new();
    for component in name.split('/') {
        path.push(component);
    }
    path
}
