use clap::{Arg, ArgAction, Command};

pub fn deps_command() -> Command {
    Command::new("deps")
        .about("Dependency management commands")
        .subcommand_required(true)
        .subcommand(
            Command::new("add")
                .about("Add a dependency")
                .arg(
                    Arg::new("package")
                        .help("Package specification (e.g., utils, utils@1.2.3, @team/pkg@^1.0.0)")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("app")
                        .help("Target app name (optional - will detect from current directory)")
                        .long("app")
                        .value_name("APP"),
                )
                .arg(
                    Arg::new("dev")
                        .help("Add as development dependency")
                        .long("dev")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("optional")
                        .help("Add as optional dependency")
                        .long("optional")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("List dependencies")
                .arg(
                    Arg::new("app")
                        .help("App name (optional - will list all apps if omitted)")
                        .index(1),
                )
                .arg(
                    Arg::new("tree")
                        .help("Show as dependency tree")
                        .long("tree")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("depth")
                        .help("Maximum tree depth")
                        .long("depth")
                        .value_name("DEPTH")
                        .value_parser(clap::value_parser!(usize)),
                ),
        )
        .subcommand(
            Command::new("resolve")
                .about("Resolve and install dependencies")
                .arg(
                    Arg::new("strategy")
                        .help("Resolution strategy")
                        .long("strategy")
                        .value_name("STRATEGY")
                        .value_parser(["latest", "strict", "compatible", "conservative"])
                        .default_value("compatible"),
                )
                .arg(
                    Arg::new("dry-run")
                        .help("Show what would be resolved without applying")
                        .long("dry-run")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("app")
                        .help("Resolve for specific app only")
                        .long("app")
                        .value_name("APP"),
                ),
        )
        .subcommand(Command::new("check").about("Check for dependency issues"))
        .subcommand(
            Command::new("tree")
                .about("Show dependency tree")
                .arg(Arg::new("app").help("App name (optional)").index(1))
                .arg(
                    Arg::new("depth")
                        .help("Maximum tree depth")
                        .long("depth")
                        .value_name("DEPTH")
                        .value_parser(clap::value_parser!(usize)),
                ),
        )
        .subcommand(
            Command::new("outdated")
                .about("Show outdated dependencies")
                .arg(Arg::new("app").help("App name (optional)").index(1)),
        )
        .subcommand(
            Command::new("why")
                .about("Explain why a package is included")
                .arg(
                    Arg::new("package")
                        .help("Package name")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("app")
                        .help("App name (optional)")
                        .long("app")
                        .value_name("APP"),
                ),
        )
        .subcommand(Command::new("sync").about("Synchronize dependency versions across apps"))
}
