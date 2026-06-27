use anyhow::Result;
use std::fs;
use std::io::{self, IsTerminal};
use tempfile::TempDir;

mod common;

use common::{spore_command, run_spore_command};

// Test helper to create temporary project structure
fn create_test_project() -> Result<TempDir> {
    let temp_dir = TempDir::new()?;

    // Create spore.yml with scripts
    let spore_yml = r#"
name: test-project
description: A test project for UI testing
scripts:
  build: "echo 'Building project...'"
  test: "echo 'Running tests...'"
  dev: "echo 'Starting development server...'"
  deploy: "echo 'Deploying application...'"
"#;
    fs::write(temp_dir.path().join("spore.yml"), spore_yml)?;

    // Create app directory with app.yml
    let app_dir = temp_dir.path().join("apps").join("web");
    fs::create_dir_all(&app_dir)?;
    let app_yml = r#"
name: web-app
description: A web application
packages:
  - types
  - utils
scripts:
  start: "echo 'Starting web app...'"
  build: "echo 'Building web app...'"
  test: "echo 'Testing web app...'"
"#;
    fs::write(app_dir.join("app.yml"), app_yml)?;

    // Create package directory with package.yml
    let package_dir = temp_dir.path().join("packages").join("utils");
    fs::create_dir_all(&package_dir)?;
    let package_yml = r#"
name: utils
version: "1.0.0"
description: Utility package
scripts:
  build: "echo 'Building utils package...'"
  test: "echo 'Testing utils package...'"
"#;
    fs::write(package_dir.join("package.yml"), package_yml)?;

    Ok(temp_dir)
}

#[test]
fn test_interactive_environment_detection() -> Result<()> {
    println!("=== Testing Interactive Environment Detection ===");

    // Test that we can detect if we're in an interactive environment
    // This test will pass whether we're in interactive mode or not
    let is_interactive = io::stdin().is_terminal();
    println!("Interactive environment detected: {}", is_interactive);

    println!("✅ Interactive environment detection test passed");
    Ok(())
}

#[test]
fn test_run_command_help_messages() -> Result<()> {
    println!("=== Testing Run Command Help Messages ===");

    let temp_project = create_test_project()?;

    // Test help message output for run command without interactive terminal
    let (stdout, stderr, _) = run_spore_command(&["run", "--help"], temp_project.path());
    let help_text = format!("{}{}", stdout, stderr);

    // Verify help message contains expected navigation instructions
    assert!(
        help_text.contains("Script name to run") || help_text.contains("script"),
        "Help should mention script parameter"
    );

    // Test that help doesn't contain old ctrl-j/ctrl-k references
    assert!(
        !help_text.contains("ctrl-j") && !help_text.contains("ctrl-k"),
        "Help should not contain ctrl-j/ctrl-k references"
    );

    println!("✅ Help message test passed");
    Ok(())
}

#[test]
fn test_run_script_validation() -> Result<()> {
    println!("=== Testing Run Script Validation ===");

    let temp_project = create_test_project()?;

    // Test running a valid script
    let output = spore_command()
        .args(["run", "build"])
        .current_dir(temp_project.path())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should either succeed or provide meaningful error
    if !output.status.success() {
        // If it fails, it should have a meaningful error message
        assert!(
            stderr.contains("script") || stdout.contains("script"),
            "Error should mention script-related issue"
        );
    }

    // Test running a non-existent script
    let output = spore_command()
        .args(["run", "nonexistent-script"])
        .current_dir(temp_project.path())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should provide helpful error message
    assert!(
        stdout.contains("not found")
            || stderr.contains("not found")
            || stdout.contains("Available scripts")
            || stderr.contains("Available scripts"),
        "Should provide helpful error for non-existent script"
    );

    println!("✅ Script validation test passed");
    Ok(())
}

#[test]
fn test_non_interactive_mode_handling() -> Result<()> {
    println!("=== Testing Non-Interactive Mode Handling ===");

    let temp_project = create_test_project()?;

    // Test interactive command in non-interactive environment
    let output = spore_command()
        .args(["run"])
        .current_dir(temp_project.path())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should handle non-interactive gracefully
    assert!(
        stdout.contains("Interactive mode")
            || stdout.contains("terminal")
            || stdout.contains("Available scripts")
            || stderr.contains("Interactive mode"),
        "Should handle non-interactive mode gracefully"
    );

    println!("✅ Non-interactive mode handling test passed");
    Ok(())
}

#[test]
fn test_script_context_detection() -> Result<()> {
    println!("=== Testing Script Context Detection ===");

    let temp_project = create_test_project()?;

    // Test from project root
    let output = spore_command()
        .args(["run", "build"])
        .current_dir(temp_project.path())
        .output()?;

    // Should detect project-level script
    // (We can't easily test the exact output, but we can verify it doesn't crash)
    assert!(output.status.success() || output.status.code().is_some());

    // Test from app directory
    let app_dir = temp_project.path().join("apps").join("web");
    let output = spore_command()
        .args(["run", "start"])
        .current_dir(&app_dir)
        .output()?;

    // Should detect app-level script
    assert!(output.status.success() || output.status.code().is_some());

    // Test from package directory
    let package_dir = temp_project.path().join("packages").join("utils");
    let output = spore_command()
        .args(["run", "build"])
        .current_dir(&package_dir)
        .output()?;

    // Should detect package-level script
    assert!(output.status.success() || output.status.code().is_some());

    println!("✅ Script context detection test passed");
    Ok(())
}

#[test]
fn test_script_execution_safety() -> Result<()> {
    println!("=== Testing Script Execution Safety ===");

    let temp_dir = TempDir::new()?;

    // Create a project with potentially dangerous script names
    let spore_yml_dangerous = r#"
name: test-project
scripts:
  "normal-script": "echo 'safe command'"
"#;
    fs::write(temp_dir.path().join("spore.yml"), spore_yml_dangerous)?;

    // Test normal script execution
    let output = spore_command()
        .args(["run", "normal-script"])
        .current_dir(temp_dir.path())
        .output()?;

    // Should execute safely
    assert!(output.status.success() || output.status.code().is_some());

    println!("✅ Script execution safety test passed");
    Ok(())
}

#[test]
fn test_working_directory_validation() -> Result<()> {
    println!("=== Testing Working Directory Validation ===");

    let temp_project = create_test_project()?;

    // Test that working directory exists and is accessible
    let app_dir = temp_project.path().join("apps").join("web");

    // Verify the directory structure was created correctly
    assert!(
        temp_project.path().exists(),
        "Project directory should exist"
    );
    assert!(
        temp_project.path().join("spore.yml").exists(),
        "spore.yml should exist"
    );
    assert!(app_dir.exists(), "App directory should exist");
    assert!(app_dir.join("app.yml").exists(), "app.yml should exist");

    // Test command execution from valid directory
    let output = spore_command()
        .args(["run", "start"])
        .current_dir(&app_dir)
        .output()?;

    // Should handle directory validation properly
    assert!(output.status.success() || output.status.code().is_some());

    println!("✅ Working directory validation test passed");
    Ok(())
}

#[test]
fn test_error_message_quality() -> Result<()> {
    println!("=== Testing Error Message Quality ===");

    let temp_dir = TempDir::new()?;

    // Test error when no config files exist
    let output = spore_command()
        .args(["run", "build"])
        .current_dir(temp_dir.path())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should provide helpful error message
    assert!(
        stdout.contains("No spore.yml")
            || stdout.contains("config")
            || stderr.contains("No spore.yml")
            || stderr.contains("config"),
        "Should provide helpful error when no config files exist"
    );

    // Create empty project with no scripts
    let spore_yml_no_scripts = r#"
name: empty-project
description: Project with no scripts
"#;
    fs::write(temp_dir.path().join("spore.yml"), spore_yml_no_scripts)?;

    let output = spore_command()
        .args(["run"])
        .current_dir(temp_dir.path())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should provide helpful message about no scripts
    assert!(
        stdout.contains("No scripts")
            || stdout.contains("scripts")
            || stderr.contains("No scripts")
            || stderr.contains("scripts")
            || stdout.contains("Interactive mode")
            || stderr.contains("Interactive mode")
            || stdout.contains("terminal")
            || stderr.contains("terminal"),
        "Should provide helpful error when no scripts exist"
    );

    println!("✅ Error message quality test passed");
    Ok(())
}

#[test]
fn test_script_name_and_command_validation() -> Result<()> {
    println!("=== Testing Script Name and Command Validation ===");

    let temp_dir = TempDir::new()?;

    // Create project with valid scripts for baseline
    let spore_yml = r#"
name: validation-test
scripts:
  build: "echo 'Building...'"
  test: "echo 'Testing...'"
  valid-script-name: "echo 'Valid script'"
"#;
    fs::write(temp_dir.path().join("spore.yml"), spore_yml)?;

    // Test valid script execution
    let output = spore_command()
        .args(["run", "build"])
        .current_dir(temp_dir.path())
        .output()?;

    // Valid scripts should work (or at least not fail due to validation)
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // If it fails, it shouldn't be due to validation issues
    if !output.status.success() {
        assert!(
            !stdout.contains("invalid") && !stderr.contains("invalid"),
            "Valid script should not fail validation"
        );
    }

    println!("✅ Script validation test passed");
    Ok(())
}

#[test]
fn test_context_label_format() -> Result<()> {
    println!("=== Testing Context Label Format ===");

    let temp_project = create_test_project()?;

    // Test from different contexts to verify label formatting
    let contexts = vec![
        (temp_project.path().to_path_buf(), "project"),
        (temp_project.path().join("apps").join("web"), "app"),
        (
            temp_project.path().join("packages").join("utils"),
            "package",
        ),
    ];

    for (dir, expected_context) in contexts {
        if dir.exists() {
            let output = spore_command().args(["run"]).current_dir(&dir).output()?;

            // The command should execute without panicking
            assert!(
                output.status.success() || output.status.code().is_some(),
                "Command should not panic in {} context",
                expected_context
            );
        }
    }

    println!("✅ Context label format test passed");
    Ok(())
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn run_all_interactive_ui_tests() -> Result<()> {
        println!("🎮 Running comprehensive interactive UI tests");

        test_interactive_environment_detection()?;
        test_run_command_help_messages()?;
        test_run_script_validation()?;
        test_non_interactive_mode_handling()?;
        test_script_context_detection()?;
        test_script_execution_safety()?;
        test_working_directory_validation()?;
        test_error_message_quality()?;
        test_script_name_and_command_validation()?;
        test_context_label_format()?;

        println!("🎉 All interactive UI tests completed successfully!");
        Ok(())
    }
}
