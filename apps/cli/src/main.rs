mod cli;
mod commands;
mod config;
mod dependency;
mod dispatch;
mod downloader;
mod ignore;
mod interpolation;
mod project;
mod templates;
mod typescript;
mod utils;
mod validation;
mod variables;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = commands::common::setup_ctrl_c_handler();

    let matches = cli::build_cli().get_matches();
    dispatch::handle_matches(matches).await
}
