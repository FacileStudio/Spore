use anyhow::Result;

#[path = "../../config.rs"]
mod config;

use config::{parse_yaml_error_with_context, AppConfig, ConfigType, SporeConfig};

pub fn run() -> Result<()> {
    test_complex_structure_errors()?;
    test_variables_errors()?;
    test_edge_cases()?;
    test_real_world_common_mistakes()?;
    Ok(())
}

fn test_complex_structure_errors() -> Result<()> {
    println!("\n=== Testing Complex Structure Errors ===");

    println!("🧪 Testing invalid apps structure...");
    let invalid_apps = r#"
name: "test-project"
apps:
  - "should-be-object-not-array"
"#;
    print_error::<SporeConfig>(invalid_apps, ConfigType::Spore, "invalid apps structure");

    println!("🧪 Testing mixed valid/invalid structure in apps...");
    let mixed_apps = r#"
name: "test-project"
apps:
  web:
    - utils
    - types
  backend:
    invalid_field: "this is wrong"
    packages:
      - validators
"#;
    print_error::<SporeConfig>(mixed_apps, ConfigType::Spore, "invalid field in app");

    Ok(())
}

fn test_variables_errors() -> Result<()> {
    println!("\n=== Testing Variables Section Errors ===");

    println!("🧪 Testing invalid variable structure...");
    let invalid_variables = r#"
name: "test-project"
variables:
  - "should-be-object-not-array"
"#;
    print_error::<SporeConfig>(
        invalid_variables,
        ConfigType::Spore,
        "invalid variables structure",
    );

    println!("🧪 Testing invalid complex variable structure...");
    let invalid_complex_var = r#"
name: "test-project"
variables:
  simple_var: "this is fine"
  complex_var:
    invalid_field: "should be 'value' and 'description'"
    another_invalid: "wrong structure"
"#;
    print_error::<SporeConfig>(
        invalid_complex_var,
        ConfigType::Spore,
        "invalid complex variable",
    );

    Ok(())
}

fn test_edge_cases() -> Result<()> {
    println!("\n=== Testing Edge Cases ===");

    println!("🧪 Testing empty YAML content...");
    print_error::<SporeConfig>("", ConfigType::Spore, "empty file");

    println!("🧪 Testing whitespace-only YAML content...");
    print_error::<SporeConfig>("   \n  \t  \n   ", ConfigType::Spore, "whitespace only");

    println!("🧪 Testing comment-only YAML content...");
    print_error::<SporeConfig>("# This is just a comment", ConfigType::Spore, "comment only");

    println!("🧪 Testing invalid Unicode characters...");
    print_error::<SporeConfig>(
        "name: \"test\x00project\"",
        ConfigType::Spore,
        "invalid chars",
    );

    Ok(())
}

fn test_real_world_common_mistakes() -> Result<()> {
    println!("\n=== Testing Real-World Common Mistakes ===");

    println!("🧪 Testing mixed tabs and spaces...");
    let mixed_whitespace =
        "name: \"test-project\"\nscripts:\n\tstart: \"npm start\"\n  build: \"npm run build\"";
    print_error::<SporeConfig>(mixed_whitespace, ConfigType::Spore, "mixed whitespace");

    println!("🧪 Testing missing dash in array...");
    let missing_dash = r#"
name: "test-app"
packages:
  utils
  types
"#;
    print_error::<AppConfig>(missing_dash, ConfigType::App, "missing dash");

    println!("🧪 Testing incorrect boolean values...");
    let wrong_boolean = r#"
name: "test-app"
tsAlias: yes
packages:
  - utils
"#;
    print_error::<AppConfig>(wrong_boolean, ConfigType::App, "wrong boolean");

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
