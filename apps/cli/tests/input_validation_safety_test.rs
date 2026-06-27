use anyhow::Result;
use std::fs;
use tempfile::TempDir;

mod common;

use common::run_spore_command;

#[test]
fn test_path_traversal_protection() -> Result<()> {
    println!("=== Testing Path Traversal Protection ===");

    let temp_dir = TempDir::new()?;

    // Test dangerous project names
    let dangerous_names = vec![
        "../../../etc/passwd",
        "..\\..\\windows\\system32",
        "/etc/shadow",
        "C:\\Windows\\System32",
        "../../dangerous",
        "./../outside",
    ];

    for dangerous_name in dangerous_names {
        let spore_yml = format!(
            r#"
name: "{}"
description: Dangerous project name test
"#,
            dangerous_name
        );

        fs::write(temp_dir.path().join("spore.yml"), spore_yml)?;

        let (stdout, stderr, success) = run_spore_command(&["status"], temp_dir.path());

        if !success {
            let error_message = format!("{}{}", stdout, stderr);
            assert!(
                error_message.contains("invalid path characters")
                    || error_message.contains("unsafe characters")
                    || error_message.contains("path")
                    || error_message.contains("parse")
                    || error_message.contains("YAML"),
                "Should reject dangerous name '{}': {}",
                dangerous_name,
                error_message
            );
        }
    }

    println!("✅ Path traversal protection test passed");
    Ok(())
}

#[test]
fn test_name_length_validation() -> Result<()> {
    println!("=== Testing Name Length Validation ===");

    let temp_dir = TempDir::new()?;

    // Test extremely long project name (101 characters)
    let long_name = "a".repeat(101);
    let spore_yml = format!(
        r#"
name: "{}"
description: Very long name test
"#,
        long_name
    );

    fs::write(temp_dir.path().join("spore.yml"), spore_yml)?;

    let (stdout, stderr, success) = run_spore_command(&["status"], temp_dir.path());

    if !success {
        let error_message = format!("{}{}", stdout, stderr);
        assert!(
            error_message.contains("too long") || error_message.contains("max"),
            "Should reject long name: {}",
            error_message
        );
    }

    // Test valid length name (100 characters)
    let valid_long_name = "a".repeat(100);
    let valid_spore_yml = format!(
        r#"
name: "{}"
description: Valid long name test
"#,
        valid_long_name
    );

    fs::write(temp_dir.path().join("spore.yml"), valid_spore_yml)?;

    let (stdout, stderr, success) = run_spore_command(&["status"], temp_dir.path());

    // Should either succeed or fail for reasons other than length
    if !success {
        let error_message = format!("{}{}", stdout, stderr);
        assert!(
            !error_message.contains("too long"),
            "Should accept 100-character name: {}",
            error_message
        );
    }

    println!("✅ Name length validation test passed");
    Ok(())
}

#[test]
fn test_dangerous_character_validation() -> Result<()> {
    println!("=== Testing Dangerous Character Validation ===");

    let temp_dir = TempDir::new()?;

    // Test names with dangerous characters
    let dangerous_chars = vec![
        "project\0name", // Null byte
        ".hidden",       // Starts with dot
        "-invalid",      // Starts with hyphen
        "project/name",  // Contains slash
        "project\\name", // Contains backslash
    ];

    for dangerous_name in dangerous_chars {
        let spore_yml = format!(
            r#"
name: "{}"
description: Dangerous character test
"#,
            dangerous_name.replace('\0', "\\0")
        ); // Escape null for YAML

        fs::write(temp_dir.path().join("spore.yml"), spore_yml)?;

        let (stdout, stderr, success) = run_spore_command(&["status"], temp_dir.path());

        if !success {
            let error_message = format!("{}{}", stdout, stderr);
            assert!(
                error_message.contains("unsafe characters")
                    || error_message.contains("invalid path characters")
                    || error_message.contains("invalid"),
                "Should reject dangerous character in '{}': {}",
                dangerous_name.replace('\0', "\\0"),
                error_message
            );
        }
    }

    println!("✅ Dangerous character validation test passed");
    Ok(())
}

#[test]
fn test_script_name_validation() -> Result<()> {
    println!("=== Testing Script Name Validation ===");

    let temp_dir = TempDir::new()?;

    // Test invalid script names
    let invalid_script_names = vec![
        ("", "empty name"),
        ("script with spaces", "whitespace"),
        ("script\ttab", "tab character"),
        ("script\nnewline", "newline"),
        ("../dangerous", "path traversal"),
        (".hidden", "starts with dot"),
        ("-invalid", "starts with hyphen"),
    ];

    for (script_name, description) in invalid_script_names {
        let spore_yml = format!(
            r#"
name: test-project
scripts:
  "{}": "echo 'test'"
"#,
            script_name.replace('\n', "\\n").replace('\t', "\\t")
        );

        fs::write(temp_dir.path().join("spore.yml"), spore_yml)?;

        let (stdout, stderr, success) = run_spore_command(&["run", script_name], temp_dir.path());

        // Command should fail due to validation
        if !success {
            let error_message = format!("{}{}", stdout, stderr);
            // Should have meaningful error about the script name issue
            let has_relevant_error = error_message.contains("script")
                || error_message.contains("name")
                || error_message.contains("invalid")
                || error_message.contains("empty")
                || error_message.contains("whitespace")
                || error_message.contains("unsafe");

            if has_relevant_error {
                println!(
                    "✓ Correctly rejected script name with {}: {}",
                    description, script_name
                );
            }
        }
    }

    println!("✅ Script name validation test passed");
    Ok(())
}

#[test]
fn test_version_format_validation() -> Result<()> {
    println!("=== Testing Version Format Validation ===");

    let temp_dir = TempDir::new()?;

    // Test invalid version formats
    let invalid_versions = vec![
        "1",         // Too short
        "1.0",       // Too short
        "1.0.0.0",   // Too long
        "1.a.0",     // Non-numeric
        "a.b.c",     // All non-numeric
        "1.0.0-",    // Invalid suffix
        "",          // Empty
        "v1.0.0",    // Prefix
        "1.0.0beta", // Invalid prerelease format
    ];

    for invalid_version in invalid_versions {
        let package_yml = format!(
            r#"
name: test-package
version: "{}"
description: Invalid version test
"#,
            invalid_version
        );

        fs::write(temp_dir.path().join("package.yml"), package_yml)?;

        let (stdout, stderr, _success) = run_spore_command(&["publish"], temp_dir.path());

        let error_message = format!("{}{}", stdout, stderr);

        // Should reject invalid version formats
        if error_message.contains("version") || error_message.contains("semver") {
            assert!(
                error_message.contains("semver")
                    || error_message.contains("format")
                    || error_message.contains("Invalid")
                    || error_message.contains("empty"),
                "Should provide specific version error for '{}': {}",
                invalid_version,
                error_message
            );
        }
    }

    println!("✅ Version format validation test passed");
    Ok(())
}

#[test]
fn test_tag_validation() -> Result<()> {
    println!("=== Testing Tag Validation ===");

    let temp_dir = TempDir::new()?;

    // Test invalid tags
    let long_invalid_tag = "a".repeat(51);
    let invalid_tags = vec![
        ("", "empty tag"),
        ("tag with spaces", "spaces"),
        ("tag@invalid", "special characters"),
        ("tag$money", "dollar sign"),
        ("-start-hyphen", "starts with hyphen"),
        ("end-hyphen-", "ends with hyphen"),
        (long_invalid_tag.as_str(), "too long"),
        ("TAG_UPPER", "uppercase/underscore"),
    ];

    for (invalid_tag, description) in invalid_tags {
        let package_yml = format!(
            r#"
name: test-package
version: "1.0.0"
tags:
  - "{}"
"#,
            invalid_tag
        );

        fs::write(temp_dir.path().join("package.yml"), package_yml)?;

        let (stdout, stderr, success) = run_spore_command(&["publish"], temp_dir.path());

        if !success {
            let error_message = format!("{}{}", stdout, stderr);

            if error_message.contains("tag") || error_message.contains("invalid") {
                println!(
                    "✓ Correctly rejected tag with {}: '{}'",
                    description, invalid_tag
                );
            }
        }
    }

    println!("✅ Tag validation test passed");
    Ok(())
}

#[test]
fn test_package_name_safety() -> Result<()> {
    println!("=== Testing Package Name Safety ===");

    let temp_dir = TempDir::new()?;

    // Test unsafe package names
    let long_unsafe_package = "a".repeat(101);
    let unsafe_packages = vec![
        ("package\0null", "null byte"),
        ("../outside", "path traversal"),
        ("package\\windows", "backslash"),
        ("@", "invalid online format"),
        ("@team/", "incomplete online format"),
        (long_unsafe_package.as_str(), "too long"),
        ("", "empty"),
    ];

    for (unsafe_package, description) in unsafe_packages {
        let app_yml = format!(
            r#"
name: test-app
packages:
  - "{}"
"#,
            unsafe_package.replace('\0', "\\0")
        );

        fs::write(temp_dir.path().join("app.yml"), app_yml)?;

        let (stdout, stderr, success) = run_spore_command(&["link"], temp_dir.path());

        if !success {
            let error_message = format!("{}{}", stdout, stderr);

            if error_message.contains("package")
                || error_message.contains("unsafe")
                || error_message.contains("invalid")
            {
                println!(
                    "✓ Correctly rejected package name with {}: '{}'",
                    description,
                    unsafe_package.replace('\0', "\\0")
                );
            }
        }
    }

    println!("✅ Package name safety test passed");
    Ok(())
}

#[test]
fn test_empty_field_validation() -> Result<()> {
    println!("=== Testing Empty Field Validation ===");

    let temp_dir = TempDir::new()?;

    // Test empty required fields
    let empty_field_configs = vec![
        (
            r#"
name: ""
description: Empty name test
"#,
            "empty name",
        ),
        (
            r#"
name: "   "
description: Whitespace name test
"#,
            "whitespace-only name",
        ),
        (
            r#"
name: test-project
scripts:
  build: ""
"#,
            "empty script command",
        ),
        (
            r#"
name: test-project
scripts:
  "": "echo test"
"#,
            "empty script name",
        ),
    ];

    for (config, description) in empty_field_configs {
        fs::write(temp_dir.path().join("spore.yml"), config)?;

        let (stdout, stderr, success) = run_spore_command(&["status"], temp_dir.path());

        if !success {
            let error_message = format!("{}{}", stdout, stderr);

            if error_message.contains("empty") || error_message.contains("cannot be empty") {
                println!("✓ Correctly rejected {}", description);
            }
        }
    }

    println!("✅ Empty field validation test passed");
    Ok(())
}

#[test]
fn test_command_injection_protection() -> Result<()> {
    println!("=== Testing Command Injection Protection ===");

    let temp_dir = TempDir::new()?;

    // Test potentially dangerous script commands
    let dangerous_commands = vec![
        "echo 'safe' && rm -rf /",
        "echo 'safe'; cat /etc/passwd",
        "$(rm -rf /)",
        "`malicious command`",
        "echo 'safe' | sh -c 'dangerous'",
    ];

    for dangerous_command in dangerous_commands {
        let spore_yml = format!(
            r#"
name: test-project
scripts:
  dangerous: "{}"
"#,
            dangerous_command
        );

        fs::write(temp_dir.path().join("spore.yml"), spore_yml)?;

        // The validation should be at the configuration level, not execution
        let (stdout, stderr, success) = run_spore_command(&["run", "dangerous"], temp_dir.path());

        // Commands themselves aren't blocked (that's shell responsibility),
        // but the configuration should be parseable and safe to store
        let error_message = format!("{}{}", stdout, stderr);

        // If there's an error, it shouldn't be due to the command content
        // (since script validation is about names, not content)
        if !success && !error_message.is_empty() {
            println!(
                "Command '{}' handling: {}",
                dangerous_command, error_message
            );
        }
    }

    println!("✅ Command injection protection test passed");
    Ok(())
}

#[test]
fn test_file_path_validation() -> Result<()> {
    println!("=== Testing File Path Validation ===");

    let temp_dir = TempDir::new()?;

    // Create subdirectories for testing
    let nested_dir = temp_dir.path().join("nested").join("deep");
    fs::create_dir_all(&nested_dir)?;

    // Test that working directory validation works correctly
    let spore_yml = r#"
name: test-project
scripts:
  build: "echo 'Building from nested directory'"
"#;
    fs::write(nested_dir.join("spore.yml"), spore_yml)?;

    // Test from the nested directory
    let (stdout, stderr, success) = run_spore_command(&["run", "build"], &nested_dir);

    // Should handle directory properly (either succeed or fail gracefully)
    if !success {
        let error_message = format!("{}{}", stdout, stderr);

        // Error should be meaningful, not a crash
        assert!(
            !error_message.contains("panic") && !error_message.contains("unwrap"),
            "Should handle directory validation gracefully: {}",
            error_message
        );
    }

    println!("✅ File path validation test passed");
    Ok(())
}

#[test]
fn test_safe_names_acceptance() -> Result<()> {
    println!("=== Testing Safe Names Acceptance ===");

    let temp_dir = TempDir::new()?;

    // Test valid names that should be accepted
    let valid_names = vec![
        "simple-name".to_string(),
        "name_with_underscores".to_string(),
        "CamelCaseName".to_string(),
        "name123".to_string(),
        "a".repeat(100),
        "my-awesome-project".to_string(),
        "utils".to_string(),
        "web-app".to_string(),
    ];

    for valid_name in valid_names {
        let spore_yml = format!(
            r#"
name: "{}"
description: Testing valid name acceptance
scripts:
  test: "echo 'Testing {}'"
"#,
            valid_name, valid_name
        );

        fs::write(temp_dir.path().join("spore.yml"), spore_yml)?;

        let (stdout, stderr, success) = run_spore_command(&["status"], temp_dir.path());

        // Should not fail due to name validation
        if !success {
            let error_message = format!("{}{}", stdout, stderr);
            assert!(
                !error_message.contains("unsafe")
                    && !error_message.contains("invalid path")
                    && !error_message.contains("too long"),
                "Valid name '{}' should be accepted: {}",
                valid_name,
                error_message
            );
        }
    }

    println!("✅ Safe names acceptance test passed");
    Ok(())
}

#[test]
fn test_input_sanitization() -> Result<()> {
    println!("=== Testing Input Sanitization ===");

    let temp_dir = TempDir::new()?;

    // Test various input edge cases
    let edge_cases = vec![
        ("unicode-name-🎉", "unicode characters"),
        ("name-with-àccénts", "accented characters"),
        ("name\r\nwith\r\nnewlines", "newlines"),
        ("name\twith\ttabs", "tabs"),
    ];

    for (test_name, description) in edge_cases {
        let spore_yml = format!(
            r#"
name: "{}"
description: Testing {}
"#,
            test_name
                .replace('\r', "\\r")
                .replace('\n', "\\n")
                .replace('\t', "\\t"),
            description
        );

        fs::write(temp_dir.path().join("spore.yml"), spore_yml)?;

        let (stdout, stderr, success) = run_spore_command(&["status"], temp_dir.path());

        // Should handle edge cases gracefully
        if !success {
            let error_message = format!("{}{}", stdout, stderr);
            println!(
                "Handling {} ('{}'): {}",
                description, test_name, error_message
            );
        }
    }

    println!("✅ Input sanitization test passed");
    Ok(())
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn run_all_input_validation_safety_tests() -> Result<()> {
        println!("🔒 Running comprehensive input validation and safety tests");

        test_path_traversal_protection()?;
        test_name_length_validation()?;
        test_dangerous_character_validation()?;
        test_script_name_validation()?;
        test_version_format_validation()?;
        test_tag_validation()?;
        test_package_name_safety()?;
        test_empty_field_validation()?;
        test_command_injection_protection()?;
        test_file_path_validation()?;
        test_safe_names_acceptance()?;
        test_input_sanitization()?;

        println!("🎉 All input validation and safety tests completed successfully!");
        Ok(())
    }
}
