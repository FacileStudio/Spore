mod basic_cases;
mod structure_cases;
mod types;
mod variable_cases;

use anyhow::Result;

fn main() -> Result<()> {
    println!("🔍 YAML Parsing Validation Test");
    println!("Testing serde_yaml version: 0.9");

    basic_cases::run()?;
    structure_cases::run()?;
    variable_cases::run()?;

    println!("\n🎉 All tests completed successfully!");
    println!("\n🔍 Summary:");
    println!("- Basic YAML parsing works");
    println!("- Apps section parsing works with both list and object formats");
    println!("- Variables section parsing works");
    println!("- Both ${{}} and {{{{}}}} variable syntax parse successfully");
    println!("- Real-world YAML from spore.yml parses correctly");
    println!("- Step-by-step field addition all works");
    println!("- Generic YAML parsing and conversion works");
    println!("- Specific problematic examples parse correctly");

    Ok(())
}
