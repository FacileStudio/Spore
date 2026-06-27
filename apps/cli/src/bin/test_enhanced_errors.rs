// Test program to demonstrate enhanced YAML error handling in real scenarios

use anyhow::Result;
use std::path::Path;

#[path = "../config.rs"]
mod config;

use config::{AppConfig, SporeConfig, PackageConfig};

fn test_file_loading() -> Result<()> {
    println!("🔍 Testing Enhanced YAML Error Handling in File Loading");

    // Test spore.yml with missing name field
    println!("\n📁 Testing missing required field in spore.yml...");
    let spore_path = Path::new("test-malformed-spore.yml");
    match SporeConfig::from_file(spore_path) {
        Ok(_) => println!("❌ Should have failed!"),
        Err(e) => {
            println!("✅ Enhanced error message:");
            println!("   {}", e);
        }
    }

    // Test spore.yml with YAML syntax error
    println!("\n📁 Testing YAML syntax error in spore.yml...");
    let syntax_error_path = Path::new("test-syntax-error-spore.yml");
    match SporeConfig::from_file(syntax_error_path) {
        Ok(_) => println!("❌ Should have failed!"),
        Err(e) => {
            println!("✅ Enhanced error message:");
            println!("   {}", e);
        }
    }

    // Test package.yml with type error
    println!("\n📁 Testing type error in package.yml...");
    let type_error_path = Path::new("test-type-error-package.yml");
    match PackageConfig::from_file(type_error_path) {
        Ok(_) => println!("❌ Should have failed!"),
        Err(e) => {
            println!("✅ Enhanced error message:");
            println!("   {}", e);
        }
    }

    // Test app.yml with unknown fields (this will pass serde validation)
    println!("\n📁 Testing app.yml with unknown fields (should pass YAML parsing)...");
    let app_path = Path::new("test-malformed-app.yml");
    match AppConfig::from_file(app_path) {
        Ok(_) => println!("✅ Parsed successfully (unknown fields are ignored by default)"),
        Err(e) => {
            println!("✅ Enhanced error message:");
            println!("   {}", e);
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    test_file_loading()?;

    println!("\n🎉 Enhanced YAML Error Handling Demonstration Complete!");
    println!("\n💡 Key Improvements:");
    println!("   • Context-specific error messages for each config type");
    println!("   • Line/column information when available");
    println!("   • Actionable suggestions for common mistakes");
    println!("   • Field-specific validation and guidance");
    println!("   • Better handling of unknown fields with suggestions");

    Ok(())
}
