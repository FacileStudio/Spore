use std::path::Path;
use std::process::{Command, Stdio};

pub fn spore_command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_spore"));
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command
}

pub fn run_spore_command(args: &[&str], working_dir: &Path) -> (String, String, bool) {
    let output = spore_command()
        .args(args)
        .current_dir(working_dir)
        .output()
        .expect("Failed to execute spore command");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let success = output.status.success();

    (stdout, stderr, success)
}
