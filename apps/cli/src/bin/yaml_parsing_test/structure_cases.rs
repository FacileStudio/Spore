use anyhow::Result;

use super::types::SporeConfig;

pub fn run() -> Result<()> {
    test_real_world_yaml()?;
    test_step_by_step_parsing()?;
    Ok(())
}

fn test_real_world_yaml() -> Result<()> {
    println!("\n=== Testing Real-World YAML (from spore.yml) ===");

    let real_spore_yaml = r#"
name: Marcel
description: A gamified organisation app to boost productivity

apps:
  web:
    tsAlias: true
    packages:
      - types
      - validators

  backend:
    tsAlias: true
    packages:
      - types
      - utils
      - validators

  marcel-cli:
    tsAlias: true
    packages:
      - types
      - utils
      - validators

scripts:
  frontend: cd frontend && bun run dev
  backend: cd backend && bun run dev
  cli: cd apps/marcel-cli && bun run dev
  cli-build: cd apps/marcel-cli && bun run build
  cli-tui: cd apps/marcel-cli && bun run build:go
"#;

    print!("Testing real-world spore.yml... ");
    match serde_yaml::from_str::<SporeConfig>(real_spore_yaml) {
        Ok(config) => {
            println!("✅ SUCCESS");
            println!("  - Name: {}", config.name);
            println!("  - Description: {:?}", config.description);
            println!(
                "  - Apps count: {}",
                config.apps.as_ref().map(|a| a.len()).unwrap_or(0)
            );
            println!(
                "  - Scripts count: {}",
                config.scripts.as_ref().map(|s| s.len()).unwrap_or(0)
            );
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

fn test_step_by_step_parsing() -> Result<()> {
    println!("\n=== Testing Step-by-Step Field Parsing ===");

    print!("Testing name field only... ");
    let name_only = r#"name: "test-project""#;
    match serde_yaml::from_str::<SporeConfig>(name_only) {
        Ok(_) => println!("✅ SUCCESS"),
        Err(e) => println!("❌ FAILED: {}", e),
    }

    print!("Testing name + description... ");
    let name_desc = r#"
name: "test-project"
description: "A test project"
"#;
    match serde_yaml::from_str::<SporeConfig>(name_desc) {
        Ok(_) => println!("✅ SUCCESS"),
        Err(e) => println!("❌ FAILED: {}", e),
    }

    print!("Testing name + description + scripts... ");
    let with_scripts = r#"
name: "test-project"
description: "A test project"
scripts:
  start: "npm start"
"#;
    match serde_yaml::from_str::<SporeConfig>(with_scripts) {
        Ok(_) => println!("✅ SUCCESS"),
        Err(e) => println!("❌ FAILED: {}", e),
    }

    print!("Testing with simple apps... ");
    let with_apps = r#"
name: "test-project"
description: "A test project"
scripts:
  start: "npm start"
apps:
  web:
    - types
"#;
    match serde_yaml::from_str::<SporeConfig>(with_apps) {
        Ok(_) => println!("✅ SUCCESS"),
        Err(e) => println!("❌ FAILED: {}", e),
    }

    print!("Testing with complex apps... ");
    let with_complex_apps = r#"
name: "test-project"
description: "A test project"
scripts:
  start: "npm start"
apps:
  web:
    tsAlias: true
    packages:
      - types
"#;
    match serde_yaml::from_str::<SporeConfig>(with_complex_apps) {
        Ok(_) => println!("✅ SUCCESS"),
        Err(e) => println!("❌ FAILED: {}", e),
    }

    print!("Testing with variables... ");
    let with_variables = r#"
name: "test-project"
description: "A test project"
scripts:
  start: "npm start"
apps:
  web:
    tsAlias: true
    packages:
      - types
variables:
  test_var: "test_value"
"#;
    match serde_yaml::from_str::<SporeConfig>(with_variables) {
        Ok(_) => println!("✅ SUCCESS"),
        Err(e) => println!("❌ FAILED: {}", e),
    }

    Ok(())
}
