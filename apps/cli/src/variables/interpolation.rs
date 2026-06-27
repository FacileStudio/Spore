use anyhow::{Context, Result};
use std::collections::HashMap;

use crate::config::{AppConfig, SporeConfig, PackageConfig};

use super::VariableContext;

pub fn interpolate_variables(text: &str, context: &VariableContext) -> Result<String> {
    let mut result = text.to_string();
    let mut missing_variables = Vec::new();
    let re = regex::Regex::new(r"\{\{(\w+)\}\}").context("Failed to compile variable regex")?;

    for captures in re.captures_iter(text) {
        let full_match = captures.get(0).unwrap().as_str();
        let var_name = captures.get(1).unwrap().as_str();

        if let Some(value) = context.get_variable(var_name) {
            result = result.replace(full_match, value);
        } else {
            missing_variables.push(var_name.to_string());
        }
    }

    if !missing_variables.is_empty() {
        anyhow::bail!(
            "Missing variables in template: {}. Available variables: {}",
            missing_variables.join(", "),
            context
                .list_variables()
                .iter()
                .map(|v| v.name.as_str())
                .collect::<Vec<&str>>()
                .join(", ")
        );
    }

    Ok(result)
}

pub trait VariableInterpolation {
    fn interpolate_variables(&mut self, context: &VariableContext) -> Result<()>;
}

impl VariableInterpolation for HashMap<String, String> {
    fn interpolate_variables(&mut self, context: &VariableContext) -> Result<()> {
        for (key, value) in self.iter_mut() {
            *value = interpolate_variables(value, context)
                .with_context(|| format!("Failed to interpolate variables in field '{}'", key))?;
        }
        Ok(())
    }
}

impl VariableInterpolation for Vec<String> {
    fn interpolate_variables(&mut self, context: &VariableContext) -> Result<()> {
        for (index, value) in self.iter_mut().enumerate() {
            *value = interpolate_variables(value, context).with_context(|| {
                format!("Failed to interpolate variables in array item {}", index)
            })?;
        }
        Ok(())
    }
}

impl VariableInterpolation for Option<String> {
    fn interpolate_variables(&mut self, context: &VariableContext) -> Result<()> {
        if let Some(value) = self {
            *value = interpolate_variables(value, context)
                .with_context(|| "Failed to interpolate variables in optional string")?;
        }
        Ok(())
    }
}

impl VariableInterpolation for SporeConfig {
    fn interpolate_variables(&mut self, context: &VariableContext) -> Result<()> {
        self.description
            .interpolate_variables(context)
            .context("Failed to interpolate project description")?;

        if let Some(scripts) = &mut self.scripts {
            scripts
                .interpolate_variables(context)
                .context("Failed to interpolate project scripts")?;
        }

        Ok(())
    }
}

impl VariableInterpolation for AppConfig {
    fn interpolate_variables(&mut self, context: &VariableContext) -> Result<()> {
        self.description
            .interpolate_variables(context)
            .context("Failed to interpolate app description")?;

        if let Some(packages) = &mut self.packages {
            packages
                .interpolate_variables(context)
                .context("Failed to interpolate app packages")?;
        }

        if let Some(scripts) = &mut self.scripts {
            scripts
                .interpolate_variables(context)
                .context("Failed to interpolate app scripts")?;
        }

        Ok(())
    }
}

impl VariableInterpolation for PackageConfig {
    fn interpolate_variables(&mut self, context: &VariableContext) -> Result<()> {
        self.description
            .interpolate_variables(context)
            .context("Failed to interpolate package description")?;
        self.author
            .interpolate_variables(context)
            .context("Failed to interpolate package author")?;
        self.license
            .interpolate_variables(context)
            .context("Failed to interpolate package license")?;
        self.repository
            .interpolate_variables(context)
            .context("Failed to interpolate package repository")?;

        if let Some(keywords) = &mut self.keywords {
            keywords
                .interpolate_variables(context)
                .context("Failed to interpolate package keywords")?;
        }
        if let Some(tags) = &mut self.tags {
            tags.interpolate_variables(context)
                .context("Failed to interpolate package tags")?;
        }
        if let Some(scripts) = &mut self.scripts {
            scripts
                .interpolate_variables(context)
                .context("Failed to interpolate package scripts")?;
        }
        if let Some(dependencies) = &mut self.dependencies {
            dependencies
                .interpolate_variables(context)
                .context("Failed to interpolate package dependencies")?;
        }
        if let Some(dev_dependencies) = &mut self.dev_dependencies {
            dev_dependencies
                .interpolate_variables(context)
                .context("Failed to interpolate package dev dependencies")?;
        }
        if let Some(optional_dependencies) = &mut self.optional_dependencies {
            optional_dependencies
                .interpolate_variables(context)
                .context("Failed to interpolate package optional dependencies")?;
        }
        if let Some(peer_dependencies) = &mut self.peer_dependencies {
            peer_dependencies
                .interpolate_variables(context)
                .context("Failed to interpolate package peer dependencies")?;
        }
        if let Some(exports) = &mut self.exports {
            exports
                .interpolate_variables(context)
                .context("Failed to interpolate package exports")?;
        }
        if let Some(features) = &mut self.features {
            features
                .interpolate_variables(context)
                .context("Failed to interpolate package features")?;
        }

        Ok(())
    }
}
