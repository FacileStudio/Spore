use anyhow::Result;

#[path = "../../config.rs"]
mod config;

use config::{parse_yaml_error_with_context, AppConfig, ConfigType, SporeConfig, PackageConfig};

pub fn run() -> Result<()> {
    test_missing_required_fields()?;
    test_invalid_field_types()?;
    test_unknown_fields()?;
    test_yaml_syntax_errors()?;
    test_duplicate_fields()?;
    Ok(())
}

fn test_missing_required_fields() -> Result<()> {
    println!("\n=== Testing Missing Required Fields ===");

    println!("🧪 Testing missing 'name' field in spore.yml...");
    let spore_no_name = r#"
description: "A project without a name"
scripts:
  start: "npm start"
"#;
    print_error::<SporeConfig>(spore_no_name, ConfigType::Spore, "missing name");

    println!("🧪 Testing missing 'version' field in package.yml...");
    let package_no_version = r#"
name: "my-package"
description: "A package without version"
"#;
    print_error::<PackageConfig>(package_no_version, ConfigType::Package, "missing version");

    println!("🧪 Testing missing 'name' field in app.yml...");
    let app_no_name = r#"
description: "An app without a name"
packages:
  - utils
"#;
    print_error::<AppConfig>(app_no_name, ConfigType::App, "missing name");

    Ok(())
}

fn test_invalid_field_types() -> Result<()> {
    println!("\n=== Testing Invalid Field Types ===");

    println!("🧪 Testing scripts field with string instead of object...");
    let invalid_scripts = r#"
name: "test-project"
scripts: "this should be an object"
"#;
    print_error::<SporeConfig>(invalid_scripts, ConfigType::Spore, "invalid scripts type");

    println!("🧪 Testing packages field with string instead of array...");
    let invalid_packages = r#"
name: "test-app"
packages: "should-be-array"
"#;
    print_error::<AppConfig>(invalid_packages, ConfigType::App, "invalid packages type");

    println!("🧪 Testing version field with number instead of string...");
    let invalid_version = r#"
name: "test-package"
version: 1.0.0
"#;
    print_error::<PackageConfig>(invalid_version, ConfigType::Package, "invalid version type");

    Ok(())
}

fn test_unknown_fields() -> Result<()> {
    println!("\n=== Testing Unknown Fields ===");

    println!("🧪 Testing unknown field 'script' (should be 'scripts')...");
    let unknown_field = r#"
name: "test-project"
script:
  start: "npm start"
"#;
    print_error::<SporeConfig>(unknown_field, ConfigType::Spore, "unknown field");

    println!("🧪 Testing unknown field 'tsalias' (should be 'tsAlias')...");
    let case_error = r#"
name: "test-app"
tsalias: true
packages:
  - utils
"#;
    print_error::<AppConfig>(case_error, ConfigType::App, "case sensitive field");

    Ok(())
}

fn test_yaml_syntax_errors() -> Result<()> {
    println!("\n=== Testing YAML Syntax Errors ===");

    println!("🧪 Testing missing colon after field name...");
    let missing_colon = r#"
name "test-project"
description: "Missing colon above"
"#;
    print_error::<SporeConfig>(missing_colon, ConfigType::Spore, "missing colon");

    println!("🧪 Testing invalid indentation...");
    let bad_indentation = r#"
name: "test-project"
scripts:
start: "npm start"
  build: "npm run build"
"#;
    print_error::<SporeConfig>(bad_indentation, ConfigType::Spore, "bad indentation");

    println!("🧪 Testing unclosed quotes...");
    let unclosed_quotes = r#"
name: "test-project
description: "Missing closing quote above"
"#;
    print_error::<SporeConfig>(unclosed_quotes, ConfigType::Spore, "unclosed quotes");

    Ok(())
}

fn test_duplicate_fields() -> Result<()> {
    println!("\n=== Testing Duplicate Fields ===");

    println!("🧪 Testing duplicate 'name' field...");
    let duplicate_name = r#"
name: "first-name"
description: "Test project"
name: "second-name"
"#;
    print_error::<SporeConfig>(duplicate_name, ConfigType::Spore, "duplicate field");

    Ok(())
}

fn print_error<T: serde::de::DeserializeOwned>(input: &str, config_type: ConfigType, label: &str) {
    match serde_yaml::from_str::<T>(input) {
        Ok(_) => println!("❌ Should have failed - {}", label),
        Err(e) => {
            let friendly_error = parse_yaml_error_with_context(&e, config_type);
            println!("✅ Caught error: {}", friendly_error);
        }
    }
}
