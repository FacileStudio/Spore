use clap::{Arg, ArgAction, Command};

pub fn link_command() -> Command {
    Command::new("link")
        .alias("l")
        .about("Link packages to apps and setup TypeScript aliases")
        .arg(
            Arg::new("symlink")
                .help("Use symlinks instead of copying (default: false)")
                .long("symlink")
                .action(ArgAction::SetTrue),
        )
}

pub fn run_command() -> Command {
    Command::new("run")
        .alias("r")
        .about("Run a script from spore.yml/yaml, app.yml/yaml, or package.yml/yaml")
        .arg(
            Arg::new("script")
                .help("Script name to run (optional - will show interactive selection if omitted)")
                .required(false)
                .index(1),
        )
}

pub fn publish_command() -> Command {
    Command::new("publish")
        .alias("p")
        .about("Publish a package to Spore Space")
        .arg(
            Arg::new("team")
                .help("Team name (optional)")
                .short('t')
                .long("team")
                .value_name("TEAM"),
        )
        .arg(
            Arg::new("description")
                .help("Package description")
                .short('d')
                .long("description")
                .value_name("DESC"),
        )
}

pub fn delete_command() -> Command {
    Command::new("delete")
        .alias("d")
        .about("Delete a package from Spore Space")
        .arg(
            Arg::new("name")
                .help("Package name")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("version")
                .help("Package version")
                .required(true)
                .index(2),
        )
}

pub fn install_command() -> Command {
    Command::new("install")
        .alias("i")
        .alias("add")
        .about("Install a package dependency to the current app")
        .arg(
            Arg::new("package")
                .help("Package name to install with optional version (examples: 'utils', 'utils@1.2.3', 'utils@^1.0.0', 'utils@~1.2.0', '@team/pkg@latest')")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("no-link")
                .help("Skip automatic linking after installing")
                .long("no-link")
                .action(ArgAction::SetTrue),
        )
}

pub fn version_command() -> Command {
    Command::new("version")
        .alias("v")
        .about("Version management commands")
        .subcommand_required(true)
        .subcommand(Command::new("patch").about("Bump patch version (bug fixes)"))
        .subcommand(Command::new("minor").about("Bump minor version (new features)"))
        .subcommand(Command::new("major").about("Bump major version (breaking changes)"))
        .subcommand(
            Command::new("prerelease")
                .about("Create pre-release version")
                .arg(
                    Arg::new("preid")
                        .help("Pre-release identifier (alpha, beta, rc)")
                        .long("preid")
                        .value_name("ID"),
                ),
        )
        .subcommand(
            Command::new("set").about("Set specific version").arg(
                Arg::new("version")
                    .help("Version to set (e.g., 1.2.3)")
                    .required(true)
                    .index(1),
            ),
        )
}

pub fn upgrade_command() -> Command {
    Command::new("upgrade")
        .alias("u")
        .about("Upgrade Spore CLI to the latest version")
        .arg(
            Arg::new("force")
                .help("Force upgrade even if already on latest version")
                .long("force")
                .action(ArgAction::SetTrue),
        )
}
