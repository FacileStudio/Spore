#[cfg(test)]
use crate::config::ConfigVariable;
use crate::config::{AppConfig, SporeConfig};
use std::collections::HashMap;
use std::path::Path;

use super::interpolation::interpolate_variables;
use super::*;

#[test]
fn test_variable_context_precedence() {
    let context = VariableContext::new("test-project", Path::new("/test"))
        .with_project_variables(&SporeConfig {
            name: "test".to_string(),
            description: None,
            ts_alias: None,
            apps: None,
            scripts: None,
            variables: Some({
                let mut vars = HashMap::new();
                vars.insert(
                    "test_var".to_string(),
                    ConfigVariable::Simple("project".to_string()),
                );
                vars
            }),
        })
        .with_app_variables(&AppConfig {
            name: "test-app".to_string(),
            description: None,
            ts_alias: None,
            packages: None,
            scripts: None,
            variables: Some({
                let mut vars = HashMap::new();
                vars.insert(
                    "test_var".to_string(),
                    ConfigVariable::Simple("app".to_string()),
                );
                vars
            }),
        });

    assert_eq!(context.get_variable("test_var"), Some("app"));
    assert_eq!(context.get_variable("project_name"), Some("test-project"));
}

#[test]
fn test_variable_interpolation() {
    let context = VariableContext::new("my-project", Path::new("/test"));

    let result = interpolate_variables("Hello {{project_name}}!", &context).unwrap();
    assert_eq!(result, "Hello my-project!");

    let result = interpolate_variables("Hello {{missing_var}}!", &context);
    assert!(result.is_err());
}

#[test]
fn test_config_variable_types() {
    let simple = ConfigVariable::Simple("test".to_string());
    assert_eq!(simple.get_value(), "test");
    assert_eq!(simple.get_description(), None);

    let complex = ConfigVariable::Complex {
        value: "test".to_string(),
        description: Some("A test variable".to_string()),
    };
    assert_eq!(complex.get_value(), "test");
    assert_eq!(complex.get_description(), Some("A test variable"));
}
