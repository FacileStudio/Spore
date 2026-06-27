use clap::{Arg, ArgAction, Command};

pub fn init_command() -> Command {
    Command::new("init")
        .about("Initialize a new Spore project")
        .arg(
            Arg::new("name")
                .help("Project name")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("path")
                .help("Target path for project creation (optional)")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::new("description")
                .help("Project description")
                .short('d')
                .long("description")
                .value_name("DESC"),
        )
}

pub fn init_package_command() -> Command {
    Command::new("init:package")
        .about("Initialize a new package")
        .arg(
            Arg::new("name")
                .help("Package name (optional, will prompt if not provided)")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("path")
                .help("Target path for package creation (optional)")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::new("team")
                .help("Team name")
                .short('t')
                .long("team")
                .value_name("TEAM"),
        )
        .arg(
            Arg::new("version")
                .help("Initial version")
                .short('v')
                .long("version")
                .value_name("VERSION"),
        )
        .arg(
            Arg::new("template")
                .help("Package template from Spore Space (e.g., @svelte-starter, @my-team/template)")
                .long("template")
                .value_name("TEMPLATE"),
        )
        .arg(
            Arg::new("description")
                .help("Package description")
                .short('d')
                .long("description")
                .value_name("DESC"),
        )
        .arg(
            Arg::new("here")
                .help("Create package in current directory instead of creating new subfolder")
                .long("here")
                .action(ArgAction::SetTrue),
        )
}

pub fn init_app_command() -> Command {
    Command::new("init:app")
        .about("Initialize a new app")
        .arg(
            Arg::new("name")
                .help("App name (optional, will prompt if not provided)")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("path")
                .help("Target path for app creation (optional)")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::new("template")
                .help("App template from Spore Space (e.g., @nextjs-starter, @my-team/app-template)")
                .long("template")
                .value_name("TEMPLATE"),
        )
        .arg(
            Arg::new("description")
                .help("App description")
                .short('d')
                .long("description")
                .value_name("DESC"),
        )
        .arg(
            Arg::new("here")
                .help("Create app in current directory instead of creating new subfolder")
                .long("here")
                .action(ArgAction::SetTrue),
        )
}
