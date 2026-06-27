use anyhow::Result;

use super::types::SporeConfig;

pub fn run() -> Result<()> {
    test_variables_section()?;
    test_problematic_yaml_patterns()?;
    test_as_generic_yaml()?;
    test_specific_problematic_examples()?;
    Ok(())
}

fn test_variables_section() -> Result<()> {
    println!("\n=== Testing Variables Section ===");

    let yaml_simple_vars = r#"
name: test-project
variables:
  project_name: "my-project"
  version: "1.0.0"
  author: "saravenpi"
"#;

    print!("Testing simple variables... ");
    match serde_yaml::from_str::<SporeConfig>(yaml_simple_vars) {
        Ok(config) => {
            println!("✅ SUCCESS");
            if let Some(vars) = &config.variables {
                for (key, value) in vars {
                    println!("  - {}: {}", key, value.get_value());
                }
            }
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }

    let yaml_complex_vars = r#"
name: test-project
variables:
  simple_var: "simple value"
  complex_var:
    value: "complex value"
    description: "This is a complex variable"
"#;

    print!("Testing complex variables... ");
    match serde_yaml::from_str::<SporeConfig>(yaml_complex_vars) {
        Ok(config) => {
            println!("✅ SUCCESS");
            if let Some(vars) = &config.variables {
                for (key, value) in vars {
                    println!("  - {}: {}", key, value.get_value());
                }
            }
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

fn test_problematic_yaml_patterns() -> Result<()> {
    println!("\n=== Testing Potentially Problematic YAML Patterns ===");

    let yaml_with_dollar_syntax = r#"
name: test-project
variables:
  project_name: "my-project"
  version: "1.0.0"
  full_name: "${project_name}-v${version}"
description: "A project called ${full_name}"
"#;

    print!("Testing YAML with ${{}} variable syntax... ");
    match serde_yaml::from_str::<SporeConfig>(yaml_with_dollar_syntax) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Name: {}", config.name);
            println!("  - Description: {:?}", config.description);
            if let Some(vars) = &config.variables {
                for (key, value) in vars {
                    println!("  - Variable {}: {}", key, value.get_value());
                }
            }
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }

    let yaml_with_brace_syntax = r#"
name: test-project
variables:
  project_name: "my-project"
  version: "1.0.0"
  full_name: "{{project_name}}-v{{version}}"
description: "A project called {{full_name}}"
"#;

    print!("Testing YAML with {{{{}}}} variable syntax... ");
    match serde_yaml::from_str::<SporeConfig>(yaml_with_brace_syntax) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Name: {}", config.name);
            println!("  - Description: {:?}", config.description);
            if let Some(vars) = &config.variables {
                for (key, value) in vars {
                    println!("  - Variable {}: {}", key, value.get_value());
                }
            }
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

fn test_as_generic_yaml() -> Result<()> {
    println!("\n=== Testing as Generic YAML (serde_yaml::Value) ===");

    let problematic_yaml = r#"
name: test-project
variables:
  project_name: "my-project"
  version: "1.0.0"
  full_name: "${project_name}-v${version}"
description: "A project called ${full_name}"
apps:
  web:
    tsAlias: true
    packages:
      - types
      - utils
"#;

    print!("Parsing as generic YAML... ");
    match serde_yaml::from_str::<serde_yaml::Value>(problematic_yaml) {
        Ok(value) => {
            println!("✅ SUCCESS");
            println!("YAML structure: {:#?}", value);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }

    print!("Converting generic YAML to SporeConfig... ");
    let value: serde_yaml::Value = serde_yaml::from_str(problematic_yaml)?;
    match serde_yaml::from_value::<SporeConfig>(value) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("Config: {:#?}", config);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

fn test_specific_problematic_examples() -> Result<()> {
    println!("\n=== Testing Specific Problematic Examples ===");

    let example_yaml = r#"
variables:
  org_name: "acme-corp"
  project_name: "awesome-platform"
  version: "2.1.0"
  author: "saravenpi"
  full_name: "${org_name}-${project_name}"
  display_name: "${project_name} v${version}"
  environment: "development"
  api_port: 3000
  db_port: 5432

name: "${full_name}"
description: "${display_name} - A comprehensive platform built by ${author}"

scripts:
  start: "echo 'Starting ${display_name} on port ${api_port}'"
  build: "echo 'Building ${full_name} for ${environment}'"

apps:
  web:
    name: "${full_name}-web"
    description: "Web interface for ${project_name}"
    packages:
      - "${org_name}-types"
      - "${org_name}-ui-components"
"#;

    print!("Testing example-with-variables.yml content... ");
    match serde_yaml::from_str::<SporeConfig>(example_yaml) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Name: {}", config.name);
            println!("  - Description: {:?}", config.description);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
