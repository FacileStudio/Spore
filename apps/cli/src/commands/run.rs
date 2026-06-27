mod direct;
mod execute;
mod interactive;
mod lookup;

use anyhow::Result;

pub async fn run_script(script_name: &str) -> Result<()> {
    direct::run_script(script_name).await
}

pub async fn run_script_interactive() -> Result<()> {
    interactive::run_script_interactive().await
}
