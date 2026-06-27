mod primary_cases;
mod secondary_cases;

use anyhow::Result;

fn main() -> Result<()> {
    println!("🔍 YAML Error Handling Test Suite");
    println!("Testing enhanced user-friendly error messages for malformed YAML files");

    primary_cases::run()?;
    secondary_cases::run()?;

    println!("\n🎉 All error handling tests completed!");
    println!("\n📊 Summary:");
    println!("✅ Missing required fields - Enhanced error messages with suggestions");
    println!("✅ Invalid field types - Context-specific type error messages");
    println!("✅ Unknown fields - Suggestions for common typos");
    println!("✅ YAML syntax errors - Clear explanations of syntax issues");
    println!("✅ Duplicate fields - Clear duplicate field detection");
    println!("✅ Complex structure errors - Detailed structure validation");
    println!("✅ Variables section errors - Variable-specific error messages");
    println!("✅ Edge cases - Proper handling of empty/invalid content");
    println!("✅ Real-world mistakes - Common formatting error detection");
    println!("\n💡 Error messages now include:");
    println!("  • Specific field-level context");
    println!("  • Configuration type awareness (spore.yml vs package.yml vs app.yml)");
    println!("  • Line/column information when available");
    println!("  • Actionable suggestions for fixes");
    println!("  • Examples of correct formatting");

    Ok(())
}
