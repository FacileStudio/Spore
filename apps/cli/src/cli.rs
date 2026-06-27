mod deps;
mod init;
mod package;
mod team;
mod vars;

use clap::Command;

pub fn build_cli() -> Command {
    Command::new("spore")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Spore - Monorepo package manager")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .allow_external_subcommands(true)
        .subcommand(init::init_command())
        .subcommand(init::init_package_command())
        .subcommand(init::init_app_command())
        .subcommand(package::link_command())
        .subcommand(package::run_command())
        .subcommand(
            Command::new("status")
                .alias("s")
                .about("Show project status"),
        )
        .subcommand(
            Command::new("auth")
                .alias("a")
                .about("Check authentication status"),
        )
        .subcommand(package::publish_command())
        .subcommand(package::delete_command())
        .subcommand(package::install_command())
        .subcommand(package::version_command())
        .subcommand(team::team_command())
        .subcommand(deps::deps_command())
        .subcommand(vars::vars_command())
        .subcommand(package::upgrade_command())
}
