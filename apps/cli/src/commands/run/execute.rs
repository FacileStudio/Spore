use anyhow::{Context, Result};
use std::path::Path;

use crate::commands::common::{create_spinner, display_success};

pub(super) async fn execute_script(
    script_name: &str,
    script_command: &str,
    working_dir: &Path,
    context: &str,
) -> Result<()> {
    if script_name.is_empty() {
        anyhow::bail!("Script name cannot be empty when executing script\n💡 Provide a script name as argument: spore run <script-name>\n💡 Use 'spore run' without arguments for interactive selection");
    }

    if script_command.is_empty() {
        anyhow::bail!("Script command cannot be empty for script '{}'\n💡 Add a command to your script in spore.yml, app.yml, or package.yml\n💡 Example: scripts:\n    {}: \"npm start\"", script_name, script_name);
    }

    if !working_dir.exists() {
        anyhow::bail!(
            "Cannot execute script '{}': Working directory does not exist at '{}'\n💡 The {} directory may have been moved or deleted\n💡 Ensure you're in the correct project directory",
            script_name, working_dir.display(), context
        );
    }

    if !working_dir.is_dir() {
        anyhow::bail!(
            "Cannot execute script '{}': Working directory at '{}' is not a directory\n💡 Expected a directory but found a file\n💡 Check for naming conflicts with files and directories",
            script_name, working_dir.display()
        );
    }

    let spinner = create_spinner(&format!(
        "Preparing to run {} script '{}'",
        context, script_name
    ));

    println!("🚀 Running {} script '{}'...", context, script_name);
    println!("📝 Command: {}", script_command);
    println!("📂 Working directory: {}", working_dir.display());

    spinner.finish_and_clear();

    let shell = if cfg!(target_os = "windows") {
        "cmd"
    } else {
        "sh"
    };

    let shell_flag = if cfg!(target_os = "windows") {
        "/C"
    } else {
        "-c"
    };

    let mut cmd = tokio::process::Command::new(shell);
    cmd.arg(shell_flag);
    cmd.arg(script_command);
    cmd.current_dir(working_dir);
    cmd.stdin(std::process::Stdio::inherit());
    cmd.stdout(std::process::Stdio::inherit());
    cmd.stderr(std::process::Stdio::inherit());

    let status = cmd.status().await.with_context(|| {
        format!(
            "Failed to execute script '{}': {}",
            script_name, script_command
        )
    })?;

    if status.success() {
        display_success(&format!("Script '{}' completed successfully", script_name));
    } else {
        let exit_code = status.code().unwrap_or(-1);
        anyhow::bail!("Script '{}' failed with exit code {}\n💡 The script command '{}' returned an error\n💡 Check the command output above for error details\n💡 Verify the script is correct in your configuration file", script_name, exit_code, script_command);
    }

    Ok(())
}
