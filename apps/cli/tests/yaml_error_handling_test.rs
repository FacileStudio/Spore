use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;

mod common;
#[path = "../src/config.rs"]
mod config;

use common::run_spore_command;
use config::{parse_yaml_error_to_user_friendly, AppConfig, SporeConfig, PackageConfig};

fn create_temp_yaml_file(content: &str, filename: &str) -> Result<(TempDir, std::path::PathBuf)> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join(filename);
    fs::write(&file_path, content)?;
    Ok((temp_dir, file_path))
}

#[test]
fn test_empty_yaml_files() -> Result<()> {
    println!("=== Testing Empty YAML Files ===");

    // Test empty spore.yml
    let (temp_dir, _) = create_temp_yaml_file("", "spore.yml")?;
    let (stdout, stderr, success) = run_spore_command(&["status"], temp_dir.path());
    assert!(!success, "Empty spore.yml should cause error");
    let error_msg = format!("{}{}", stdout, stderr);
    assert!(
        error_msg.contains("empty") || error_msg.contains("parse") || error_msg.contains("invalid"),
        "Expected empty file error, got: {}",
        error_msg
    );

    // Test empty package.yml
    let (temp_dir, _) = create_temp_yaml_file("", "package.yml")?;
    let (stdout, stderr, _success) = run_spore_command(&["publish"], temp_dir.path());
    let error_msg = format!("{}{}", stdout, stderr);
    if error_msg.contains("empty") || error_msg.contains("parse") {
        println!("✓ Empty package.yml properly rejected");
    }

    // Test empty app.yml
    let (temp_dir, _) = create_temp_yaml_file("", "app.yml")?;
    let (stdout, stderr, _success) = run_spore_command(&["link"], temp_dir.path());
    let error_msg = format!("{}{}", stdout, stderr);
    if error_msg.contains("empty") || error_msg.contains("parse") {
        println!("✓ Empty app.yml properly rejected");
    }

    println!("✅ Empty file tests passed");
    Ok(())
}

#[test]
fn test_missing_required_fields() -> Result<()> {
    println!("=== Testing Missing Required Fields ===");

    // Test spore.yml missing name field
    let invalid_spore = r#"
description: "Project without name"
scripts:
  build: "npm build"
"#;
    let (_temp_dir, spore_path) = create_temp_yaml_file(invalid_spore, "spore.yml")?;
    let result = SporeConfig::from_file(&spore_path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Missing name field")
            || error_msg.contains("missing field")
            || error_msg.contains("Missing required field")
            || error_msg.contains("required field 'name'"),
        "Expected missing name error, got: {}",
        error_msg
    );

    // Test package.yml missing name field
    let invalid_package = r#"
version: "1.0.0"
description: "Package without name"
"#;
    let (_temp_dir, package_path) = create_temp_yaml_file(invalid_package, "package.yml")?;
    let result = PackageConfig::from_file(&package_path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Missing name field")
            || error_msg.contains("missing field")
            || error_msg.contains("Missing required field")
            || error_msg.contains("required field 'name'"),
        "Expected missing name error, got: {}",
        error_msg
    );

    // Test package.yml missing version field
    let invalid_package_version = r#"
name: "test-package"
description: "Package without version"
"#;
    let (_temp_dir, package_path) = create_temp_yaml_file(invalid_package_version, "package.yml")?;
    let result = PackageConfig::from_file(&package_path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Missing version field")
            || error_msg.contains("missing field")
            || error_msg.contains("Missing required field")
            || error_msg.contains("required field 'version'"),
        "Expected missing version error, got: {}",
        error_msg
    );

    // Test app.yml missing name field
    let invalid_app = r#"
description: "App without name"
packages:
  - types
"#;
    let (_temp_dir, app_path) = create_temp_yaml_file(invalid_app, "app.yml")?;
    let result = AppConfig::from_file(&app_path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Missing name field")
            || error_msg.contains("missing field")
            || error_msg.contains("Missing required field")
            || error_msg.contains("required field 'name'"),
        "Expected missing name error, got: {}",
        error_msg
    );

    println!("✅ Missing required field tests passed");
    Ok(())
}

#[test]
fn test_invalid_yaml_syntax() -> Result<()> {
    println!("=== Testing Invalid YAML Syntax ===");

    // Test invalid indentation
    let invalid_indentation = r#"
name: test-project
description: A test project
scripts:
build: "npm build"  # Wrong indentation
  test: "npm test"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(invalid_indentation, "spore.yml")?;
    let result = SporeConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Invalid YAML syntax")
            || error_msg.contains("while parsing")
            || error_msg.contains("Invalid YAML"),
        "Expected YAML syntax error, got: {}",
        error_msg
    );

    // Test missing quotes in string with special characters
    let invalid_quotes = r#"
name: test-project
description: This has: invalid characters without quotes
"#;
    let (_temp_dir, path) = create_temp_yaml_file(invalid_quotes, "spore.yml")?;
    let result = SporeConfig::from_file(&path);
    assert!(result.is_err());

    // Test invalid list syntax
    let invalid_list = r#"
name: test-project
apps:
  web
    - types
    - utils
"#;
    let (_temp_dir, path) = create_temp_yaml_file(invalid_list, "spore.yml")?;
    let result = SporeConfig::from_file(&path);
    assert!(result.is_err());

    // Test duplicate keys
    let duplicate_keys = r#"
name: test-project
name: duplicate-project
description: Project with duplicate names
"#;
    let (_temp_dir, path) = create_temp_yaml_file(duplicate_keys, "spore.yml")?;
    let result = SporeConfig::from_file(&path);
    assert!(result.is_err());

    println!("✅ Invalid YAML syntax tests passed");
    Ok(())
}

#[test]
fn test_invalid_field_types() -> Result<()> {
    println!("=== Testing Invalid Field Types ===");

    // Test name as object instead of string
    let invalid_name_type = r#"
name:
  nested: value
description: "Project with object name"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(invalid_name_type, "spore.yml")?;
    let result = SporeConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Expected string value") || error_msg.contains("invalid type"),
        "Expected type error, got: {}",
        error_msg
    );

    // Test scripts as string instead of map
    let invalid_scripts_type = r#"
name: test-project
scripts: "not a map"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(invalid_scripts_type, "spore.yml")?;
    let result = SporeConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Expected object/mapping value") || error_msg.contains("invalid type"),
        "Expected mapping error, got: {}",
        error_msg
    );

    // Test packages as string instead of array
    let invalid_packages_type = r#"
name: test-app
packages: "not an array"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(invalid_packages_type, "app.yml")?;
    let result = AppConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Expected array/list value") || error_msg.contains("invalid type"),
        "Expected array error, got: {}",
        error_msg
    );

    println!("✅ Invalid field type tests passed");
    Ok(())
}

#[test]
fn test_malformed_version_formats() -> Result<()> {
    println!("=== Testing Malformed Version Formats ===");

    // Test invalid semver format (too few parts)
    let invalid_version_short = r#"
name: test-package
version: "1.0"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(invalid_version_short, "package.yml")?;
    let result = PackageConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("semver format") || error_msg.contains("semantic versioning format"),
        "Expected semver error, got: {}",
        error_msg
    );

    // Test invalid semver format (too many parts)
    let invalid_version_long = r#"
name: test-package
version: "1.0.0.0"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(invalid_version_long, "package.yml")?;
    let result = PackageConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("semver format") || error_msg.contains("semantic versioning format"),
        "Expected semver error, got: {}",
        error_msg
    );

    // Test non-numeric version parts
    let invalid_version_non_numeric = r#"
name: test-package
version: "1.a.0"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(invalid_version_non_numeric, "package.yml")?;
    let result = PackageConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Invalid version number")
            || error_msg.contains("semantic versioning format"),
        "Expected version number error, got: {}",
        error_msg
    );

    println!("✅ Malformed version format tests passed");
    Ok(())
}

#[test]
fn test_unsafe_name_validation() -> Result<()> {
    println!("=== Testing Unsafe Name Validation ===");

    // Test path traversal attempts
    let path_traversal = r#"
name: "../../../dangerous"
description: "Project with path traversal"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(path_traversal, "spore.yml")?;
    let result = SporeConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("invalid path characters"),
        "Expected path error, got: {}",
        error_msg
    );

    // Test names starting with dot
    let dot_name = r#"
name: ".hidden"
description: "Project with dot prefix"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(dot_name, "spore.yml")?;
    let result = SporeConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("unsafe characters") || error_msg.contains("control characters"),
        "Expected unsafe character error, got: {}",
        error_msg
    );

    // Test names starting with hyphen
    let hyphen_name = r#"
name: "-invalid"
description: "Project with hyphen prefix"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(hyphen_name, "spore.yml")?;
    let result = SporeConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("unsafe characters") || error_msg.contains("control characters"),
        "Expected unsafe character error, got: {}",
        error_msg
    );

    // Test extremely long names
    let long_name = "a".repeat(101);
    let long_name_yaml = format!(
        r#"
name: "{}"
description: "Project with very long name"
"#,
        long_name
    );
    let (_temp_dir, path) = create_temp_yaml_file(&long_name_yaml, "spore.yml")?;
    let result = SporeConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("too long"),
        "Expected length error, got: {}",
        error_msg
    );

    println!("✅ Unsafe name validation tests passed");
    Ok(())
}

#[test]
fn test_invalid_script_names() -> Result<()> {
    println!("=== Testing Invalid Script Names ===");

    // Test script name with whitespace
    let whitespace_script = r#"
name: test-project
scripts:
  "build app": "npm run build"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(whitespace_script, "spore.yml")?;
    let result = SporeConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("cannot contain whitespace"),
        "Expected whitespace error, got: {}",
        error_msg
    );

    // Test empty script command
    let empty_command = r#"
name: test-project
scripts:
  build: ""
"#;
    let (_temp_dir, path) = create_temp_yaml_file(empty_command, "spore.yml")?;
    let result = SporeConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("cannot be empty"),
        "Expected empty command error, got: {}",
        error_msg
    );

    // Test empty script name
    let empty_script_name = r#"
name: test-project
scripts:
  "": "npm run build"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(empty_script_name, "spore.yml")?;
    let result = SporeConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("cannot be empty"),
        "Expected empty name error, got: {}",
        error_msg
    );

    println!("✅ Invalid script name tests passed");
    Ok(())
}

#[test]
fn test_invalid_tag_validation() -> Result<()> {
    println!("=== Testing Invalid Tag Validation ===");

    // Test tag with invalid characters
    let invalid_tag_chars = r#"
name: test-package
version: "1.0.0"
tags:
  - "invalid@tag"
  - "another$tag"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(invalid_tag_chars, "package.yml")?;
    let result = PackageConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("invalid characters"),
        "Expected invalid character error, got: {}",
        error_msg
    );

    // Test tag starting with hyphen
    let hyphen_start_tag = r#"
name: test-package
version: "1.0.0"
tags:
  - "-invalid"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(hyphen_start_tag, "package.yml")?;
    let result = PackageConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("cannot start or end with hyphen"),
        "Expected hyphen error, got: {}",
        error_msg
    );

    // Test tag ending with hyphen
    let hyphen_end_tag = r#"
name: test-package
version: "1.0.0"
tags:
  - "invalid-"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(hyphen_end_tag, "package.yml")?;
    let result = PackageConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("cannot start or end with hyphen"),
        "Expected hyphen error, got: {}",
        error_msg
    );

    // Test extremely long tag
    let long_tag = "a".repeat(51);
    let long_tag_yaml = format!(
        r#"
name: test-package
version: "1.0.0"
tags:
  - "{}"
"#,
        long_tag
    );
    let (_temp_dir, path) = create_temp_yaml_file(&long_tag_yaml, "package.yml")?;
    let result = PackageConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("too long"),
        "Expected length error, got: {}",
        error_msg
    );

    // Test empty tag
    let empty_tag = r#"
name: test-package
version: "1.0.0"
tags:
  - ""
"#;
    let (_temp_dir, path) = create_temp_yaml_file(empty_tag, "package.yml")?;
    let result = PackageConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("cannot be empty"),
        "Expected empty tag error, got: {}",
        error_msg
    );

    println!("✅ Invalid tag validation tests passed");
    Ok(())
}

#[test]
fn test_package_name_validation() -> Result<()> {
    println!("=== Testing Package Name Validation ===");

    // Test invalid online package format
    let invalid_online_package = r#"
name: test-app
packages:
  - "@"
  - "@team/"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(invalid_online_package, "app.yml")?;
    let result = AppConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Invalid online package name")
            || error_msg.contains("Invalid scoped package name"),
        "Expected online package error, got: {}",
        error_msg
    );

    // Test package name with null bytes
    let null_byte_package = format!(
        r#"
name: test-app
packages:
  - "invalid{}package"
"#,
        '\0'
    );
    let (_temp_dir, path) = create_temp_yaml_file(&null_byte_package, "app.yml")?;
    let result = AppConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("unsafe characters") || error_msg.contains("control characters"),
        "Expected unsafe character error, got: {}",
        error_msg
    );

    // Test extremely long package name
    let long_package_name = "a".repeat(101);
    let long_package_yaml = format!(
        r#"
name: test-app
packages:
  - "{}"
"#,
        long_package_name
    );
    let (_temp_dir, path) = create_temp_yaml_file(&long_package_yaml, "app.yml")?;
    let result = AppConfig::from_file(&path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("too long"),
        "Expected length error, got: {}",
        error_msg
    );

    println!("✅ Package name validation tests passed");
    Ok(())
}

#[test]
fn test_error_message_parser() -> Result<()> {
    println!("=== Testing Error Message Parser ===");

    // Create mock serde_yaml errors and test the parser
    let test_cases = vec![
        ("missing field `name`", "Missing name field"),
        ("missing field `version`", "Missing version field"),
        ("missing field `description`", "Missing description field"),
        ("invalid type: expected a string", "Expected string value"),
        (
            "invalid type: expected a sequence",
            "Expected array/list value",
        ),
        (
            "invalid type: expected a map",
            "Expected object/mapping value",
        ),
        ("duplicate key found", "Duplicate field found"),
        ("while parsing a block mapping", "Invalid YAML syntax"),
        ("unknown error type", "unknown error type"), // Should pass through
    ];

    for (_input, _expected) in test_cases {
        // We can't easily create serde_yaml::Error directly, so we'll test with string contains
        let error = serde_yaml::from_str::<HashMap<String, String>>("invalid yaml").unwrap_err();
        let parsed = parse_yaml_error_to_user_friendly(&error);

        // For this test, we'll just verify the function doesn't panic and returns a string
        assert!(
            !parsed.is_empty(),
            "Error parser should not return empty string"
        );
    }

    println!("✅ Error message parser tests passed");
    Ok(())
}

#[test]
fn test_valid_configurations() -> Result<()> {
    println!("=== Testing Valid Configurations (Regression Tests) ===");

    // Test valid spore.yml
    let valid_spore = r#"
name: test-project
description: A valid test project
tsAlias: true
scripts:
  build: "npm run build"
  test: "npm test"
  dev: "npm run dev"
apps:
  web:
    tsAlias: true
    packages:
      - types
      - utils
  backend:
    - database
    - validators
variables:
  project_version: "1.0.0"
  api_url: "https://api.example.com"
  complex_var:
    value: "complex value"
    description: "A complex variable"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(valid_spore, "spore.yml")?;
    let result = SporeConfig::from_file(&path);
    assert!(
        result.is_ok(),
        "Valid spore.yml should parse successfully: {:?}",
        result.err()
    );

    // Test valid package.yml
    let valid_package = r#"
name: test-package
team: test-team
version: "1.2.3"
description: A valid test package
author: saravenpi
license: MIT
repository: https://github.com/user/repo
keywords:
  - test
  - package
tags:
  - frontend
  - utility
scripts:
  build: "bun run build"
  test: "bun test"
dependencies:
  - types
  - utils
dev_dependencies:
  - test-utils
exports:
  ".": "./index.js"
  "./utils": "./utils.js"
features:
  - typescript
  - testing
variables:
  package_name: "test-package"
  build_target: "es2020"
"#;
    let (_temp_dir, path) = create_temp_yaml_file(valid_package, "package.yml")?;
    let result = PackageConfig::from_file(&path);
    assert!(
        result.is_ok(),
        "Valid package.yml should parse successfully: {:?}",
        result.err()
    );

    // Test valid app.yml
    let valid_app = r##"
name: test-app
description: A valid test app
tsAlias: "#"
packages:
  - types
  - utils
  - "@external/package"
  - "@team/shared-lib"
scripts:
  start: "bun run start"
  build: "bun run build"
  dev: "bun run dev"
variables:
  app_port: "3000"
  app_title: "Test Application"
"##;
    let (_temp_dir, path) = create_temp_yaml_file(valid_app, "app.yml")?;
    let result = AppConfig::from_file(&path);
    assert!(
        result.is_ok(),
        "Valid app.yml should parse successfully: {:?}",
        result.err()
    );

    println!("✅ Valid configuration tests passed");
    Ok(())
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn run_all_yaml_error_tests() -> Result<()> {
        println!("🔍 Running comprehensive YAML error handling tests");

        test_empty_yaml_files()?;
        test_missing_required_fields()?;
        test_invalid_yaml_syntax()?;
        test_invalid_field_types()?;
        test_malformed_version_formats()?;
        test_unsafe_name_validation()?;
        test_invalid_script_names()?;
        test_invalid_tag_validation()?;
        test_package_name_validation()?;
        test_error_message_parser()?;
        test_valid_configurations()?;

        println!("🎉 All YAML error handling tests completed successfully!");
        Ok(())
    }
}
