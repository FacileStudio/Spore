use anyhow::Result;
use clap::ArgMatches;

use crate::{cli, commands};

pub async fn handle_matches(matches: ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name");
            let path = sub_matches.get_one::<String>("path");
            let description = sub_matches
                .get_one::<String>("description")
                .map(|value| value.as_str());
            commands::init_project(
                name.map(|value| value.as_str()),
                path.map(|value| value.as_str()),
                description,
            )?;
        }
        Some(("init:package", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name");
            let path = sub_matches.get_one::<String>("path");
            let team = sub_matches
                .get_one::<String>("team")
                .map(|value| value.as_str());
            let version = sub_matches
                .get_one::<String>("version")
                .map(|value| value.as_str());
            let template = sub_matches
                .get_one::<String>("template")
                .map(|value| value.as_str());
            let description = sub_matches
                .get_one::<String>("description")
                .map(|value| value.as_str());
            let here = sub_matches.get_flag("here");

            commands::init_package(
                name.map(|value| value.as_str()),
                team,
                version,
                template,
                description,
                path.map(|value| value.as_str()),
                here,
            )
            .await?;
        }
        Some(("init:app", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name");
            let path = sub_matches.get_one::<String>("path");
            let template = sub_matches
                .get_one::<String>("template")
                .map(|value| value.as_str());
            let description = sub_matches
                .get_one::<String>("description")
                .map(|value| value.as_str());
            let here = sub_matches.get_flag("here");

            commands::init_app(
                name.map(|value| value.as_str()),
                template,
                description,
                path.map(|value| value.as_str()),
                here,
            )
            .await?;
        }
        Some(("link", sub_matches)) => {
            commands::link_packages(sub_matches.get_flag("symlink")).await?;
        }
        Some(("run", sub_matches)) => {
            if let Some(script_name) = sub_matches.get_one::<String>("script") {
                commands::run_script(script_name).await?;
            } else {
                commands::run_script_interactive().await?;
            }
        }
        Some(("status", _)) => {
            commands::show_status()?;
        }
        Some(("auth", _)) => {
            commands::auth_status().await?;
        }
        Some(("publish", sub_matches)) => {
            let team = sub_matches
                .get_one::<String>("team")
                .map(|value| value.as_str());
            let description = sub_matches
                .get_one::<String>("description")
                .map(|value| value.as_str());
            commands::publish_package(team, description).await?;
        }
        Some(("delete", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name")
                .ok_or_else(|| anyhow::anyhow!("Package name is required for deletion\n💡 Usage: spore delete <package-name> <version>\n💡 Example: spore delete my-package 1.0.0"))?;
            let version = sub_matches.get_one::<String>("version")
                .ok_or_else(|| anyhow::anyhow!("Package version is required for deletion\n💡 Usage: spore delete <package-name> <version>\n💡 Example: spore delete my-package 1.0.0"))?;
            commands::delete_package(name, version).await?;
        }
        Some(("install", sub_matches)) | Some(("add", sub_matches)) => {
            let package = sub_matches.get_one::<String>("package")
                .ok_or_else(|| anyhow::anyhow!("Package specification is required for installation\n💡 Usage: spore install <package-spec>\n💡 Examples:\n  - spore install utils (install latest version)\n  - spore install utils@1.2.3 (install specific version)\n  - spore install @team/package (install from Spore Space)"))?;
            let auto_link = !sub_matches.get_flag("no-link");
            commands::add_package(package, auto_link).await?;
        }
        Some(("version", sub_matches)) => match sub_matches.subcommand() {
            Some(("patch", _)) => commands::version_bump("patch").await?,
            Some(("minor", _)) => commands::version_bump("minor").await?,
            Some(("major", _)) => commands::version_bump("major").await?,
            Some(("prerelease", prerelease_sub)) => {
                let preid = prerelease_sub
                    .get_one::<String>("preid")
                    .map(|value| value.as_str());
                commands::version_prerelease(preid).await?;
            }
            Some(("set", set_sub)) => {
                let version = set_sub.get_one::<String>("version")
                    .ok_or_else(|| anyhow::anyhow!("Version number is required for version set\n💡 Usage: spore version set <version>\n💡 Example: spore version set 2.1.0\n💡 Must follow semantic versioning (major.minor.patch)"))?;
                commands::version_set(version).await?;
            }
            _ => unreachable!(),
        },
        Some(("team", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", team_sub)) => {
                let name = team_sub
                    .get_one::<String>("name")
                    .map(|value| value.as_str());
                let description = team_sub
                    .get_one::<String>("description")
                    .map(|value| value.as_str());
                commands::create_team(name, description).await?;
            }
            Some(("list", _)) => commands::list_teams().await?,
            Some(("info", team_sub)) => {
                let name = team_sub
                    .get_one::<String>("name")
                    .map(|value| value.as_str());
                commands::team_info(name).await?;
            }
            Some(("add-member", team_sub)) => {
                let team = team_sub.get_one::<String>("team");
                let username = team_sub.get_one::<String>("username");
                let role = team_sub.get_one::<String>("role")
                    .ok_or_else(|| anyhow::anyhow!("Member role is required for adding team members\n💡 Usage: spore team add-member [team] [username] --role <admin|member>\n💡 Valid roles: 'admin' or 'member'\n💡 Example: spore team add-member my-team john --role admin"))?;
                commands::add_team_member(
                    team.map(|value| value.as_str()),
                    username.map(|value| value.as_str()),
                    role,
                )
                .await?;
            }
            Some(("remove-member", team_sub)) => {
                let team = team_sub.get_one::<String>("team");
                let username = team_sub.get_one::<String>("username");
                commands::remove_team_member(
                    team.map(|value| value.as_str()),
                    username.map(|value| value.as_str()),
                )
                .await?;
            }
            _ => unreachable!(),
        },
        Some(("deps", sub_matches)) => match sub_matches.subcommand() {
            Some(("add", deps_sub)) => {
                let package_spec = deps_sub.get_one::<String>("package")
                    .ok_or_else(|| anyhow::anyhow!("Package specification is required for dependency addition\n💡 Usage: spore deps add <package-spec> [options]\n💡 Examples:\n  - spore deps add utils@1.0.0\n  - spore deps add @team/package@^2.0.0\n  - spore deps add local-package --dev"))?;
                let app_name = deps_sub
                    .get_one::<String>("app")
                    .map(|value| value.as_str());
                commands::deps_add(
                    package_spec,
                    app_name,
                    deps_sub.get_flag("dev"),
                    deps_sub.get_flag("optional"),
                )
                .await?;
            }
            Some(("list", deps_sub)) => {
                let app_name = deps_sub
                    .get_one::<String>("app")
                    .map(|value| value.as_str());
                commands::deps_list(
                    app_name,
                    deps_sub.get_flag("tree"),
                    deps_sub.get_one::<usize>("depth").copied(),
                )
                .await?;
            }
            Some(("resolve", deps_sub)) => {
                let strategy = deps_sub
                    .get_one::<String>("strategy")
                    .map(|value| value.as_str());
                let app_name = deps_sub
                    .get_one::<String>("app")
                    .map(|value| value.as_str());
                commands::deps_resolve(strategy, deps_sub.get_flag("dry-run"), app_name).await?;
            }
            Some(("check", _)) => commands::deps_check().await?,
            Some(("tree", deps_sub)) => {
                let app_name = deps_sub
                    .get_one::<String>("app")
                    .map(|value| value.as_str());
                commands::deps_tree(app_name, deps_sub.get_one::<usize>("depth").copied()).await?;
            }
            Some(("outdated", deps_sub)) => {
                let app_name = deps_sub
                    .get_one::<String>("app")
                    .map(|value| value.as_str());
                commands::deps_outdated(app_name).await?;
            }
            Some(("why", deps_sub)) => {
                let package_name = deps_sub.get_one::<String>("package")
                    .ok_or_else(|| anyhow::anyhow!("Package name is required to explain dependency inclusion\n💡 Usage: spore deps why <package-name> [--app <app-name>]\n💡 Example: spore deps why lodash --app my-frontend"))?;
                let app_name = deps_sub
                    .get_one::<String>("app")
                    .map(|value| value.as_str());
                commands::deps_why(package_name, app_name).await?;
            }
            Some(("sync", _)) => commands::deps_sync().await?,
            _ => unreachable!(),
        },
        Some(("vars", sub_matches)) | Some(("variables", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("list", vars_sub)) => {
                    let app_name = vars_sub
                        .get_one::<String>("app")
                        .map(|value| value.as_str());
                    let package_name = vars_sub
                        .get_one::<String>("package")
                        .map(|value| value.as_str());
                    commands::vars_list(app_name, package_name)?;
                }
                Some(("get", vars_sub)) => {
                    let var_name = vars_sub.get_one::<String>("name")
                    .ok_or_else(|| anyhow::anyhow!("Variable name is required to retrieve variable value\n💡 Usage: spore vars get <variable-name> [options]\n💡 Example: spore vars get api_url --app my-frontend\n💡 Use 'spore vars list' to see all available variables"))?;
                    let app_name = vars_sub
                        .get_one::<String>("app")
                        .map(|value| value.as_str());
                    let package_name = vars_sub
                        .get_one::<String>("package")
                        .map(|value| value.as_str());
                    commands::vars_get(var_name, app_name, package_name)?;
                }
                _ => unreachable!(),
            }
        }
        Some(("upgrade", sub_matches)) => {
            commands::update_cli(sub_matches.get_flag("force")).await?;
        }
        Some((script_name, _)) => {
            commands::run_script(script_name).await?;
        }
        None => print_root_help()?,
    }

    Ok(())
}

fn print_root_help() -> Result<()> {
    println!(
        "{}",
        console::style("🪢 Spore - Monorepo package manager")
            .bold()
            .cyan()
    );
    println!(
        "{} {}",
        console::style("Version:").dim(),
        env!("CARGO_PKG_VERSION")
    );
    println!();

    cli::build_cli().print_help()?;
    println!();
    commands::common::display_tip(
        "Run 'spore <command> --help' for more information on a specific command",
    );

    Ok(())
}
