use clap::{Arg, Command};

pub fn team_command() -> Command {
    Command::new("team")
        .alias("t")
        .about("Team management commands")
        .subcommand_required(true)
        .subcommand(
            Command::new("create")
                .about("Create a new team")
                .arg(
                    Arg::new("name")
                        .help("Team name (optional - will prompt if not provided)")
                        .required(false)
                        .index(1),
                )
                .arg(
                    Arg::new("description")
                        .help("Team description")
                        .short('d')
                        .long("description")
                        .value_name("DESC"),
                ),
        )
        .subcommand(Command::new("list").about("List all teams"))
        .subcommand(
            Command::new("info").about("Show team information").arg(
                Arg::new("name")
                    .help("Team name (optional - will show selection if not provided)")
                    .required(false)
                    .index(1),
            ),
        )
        .subcommand(
            Command::new("add-member")
                .about("Add member to team")
                .arg(Arg::new("team").help("Team name").required(false).index(1))
                .arg(
                    Arg::new("username")
                        .help("Username to add")
                        .required(false)
                        .index(2),
                )
                .arg(
                    Arg::new("role")
                        .help("Role (admin or member)")
                        .short('r')
                        .long("role")
                        .value_name("ROLE")
                        .value_parser(["admin", "member"])
                        .default_value("member"),
                ),
        )
        .subcommand(
            Command::new("remove-member")
                .about("Remove member from team")
                .arg(Arg::new("team").help("Team name").required(false).index(1))
                .arg(
                    Arg::new("username")
                        .help("Username to remove")
                        .required(false)
                        .index(2),
                ),
        )
}
