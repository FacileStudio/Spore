use anyhow::Result;

use super::types::SporeConfig;

pub fn run() -> Result<()> {
    test_basic_yaml_parsing()?;
    test_apps_section_variations()?;
    Ok(())
}

fn test_basic_yaml_parsing() -> Result<()> {
    println!("=== Testing Basic YAML Parsing ===");

    let simple_yaml = r#"
name: test-project
description: A simple test project
"#;

    print!("Testing simple YAML... ");
    match serde_yaml::from_str::<SporeConfig>(simple_yaml) {
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

    let yaml_with_scripts = r#"
name: test-project
description: A test project with scripts
scripts:
  start: "npm start"
  build: "npm run build"
  test: "npm test"
"#;

    print!("Testing YAML with scripts... ");
    match serde_yaml::from_str::<SporeConfig>(yaml_with_scripts) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Scripts: {:?}", config.scripts);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }

    let yaml_with_apps = r#"
name: test-project
description: A test project with apps
apps:
  web:
    - types
    - utils
  backend:
    - types
    - validators
"#;

    print!("Testing YAML with apps (simple)... ");
    match serde_yaml::from_str::<SporeConfig>(yaml_with_apps) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Apps: {:?}", config.apps);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

fn test_apps_section_variations() -> Result<()> {
    println!("\n=== Testing Apps Section Variations ===");

    let yaml_object_format = r#"
name: test-project
apps:
  web:
    tsAlias: true
    packages:
      - types
      - utils
"#;

    print!("Testing apps with object format... ");
    match serde_yaml::from_str::<SporeConfig>(yaml_object_format) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Apps: {:?}", config.apps);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }

    let yaml_mixed_format = r#"
name: test-project
apps:
  web:
    - types
    - utils
  backend:
    tsAlias: true
    packages:
      - types
      - validators
"#;

    print!("Testing apps with mixed formats... ");
    match serde_yaml::from_str::<SporeConfig>(yaml_mixed_format) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Apps: {:?}", config.apps);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
