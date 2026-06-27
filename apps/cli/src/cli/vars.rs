use clap::{Arg, Command};

pub fn vars_command() -> Command {
    Command::new("vars")
        .alias("variables")
        .about("Variable management commands")
        .subcommand_required(true)
        .subcommand(
            Command::new("list")
                .about("List all available variables with their sources")
                .arg(
                    Arg::new("app")
                        .help("Show variables for specific app")
                        .long("app")
                        .value_name("APP"),
                )
                .arg(
                    Arg::new("package")
                        .help("Show variables for specific package")
                        .long("package")
                        .value_name("PACKAGE"),
                ),
        )
        .subcommand(
            Command::new("get")
                .about("Get the value of a specific variable")
                .arg(
                    Arg::new("name")
                        .help("Variable name")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("app")
                        .help("Context app name")
                        .long("app")
                        .value_name("APP"),
                )
                .arg(
                    Arg::new("package")
                        .help("Context package name")
                        .long("package")
                        .value_name("PACKAGE"),
                ),
        )
}
