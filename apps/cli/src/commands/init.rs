mod app;
mod package;
mod project;
mod template;

use anyhow::Result;

pub fn init_project(
    name: Option<&str>,
    path: Option<&str>,
    description: Option<&str>,
) -> Result<()> {
    project::init_project(name, path, description)
}

pub async fn init_package(
    name: Option<&str>,
    team: Option<&str>,
    version: Option<&str>,
    template: Option<&str>,
    description: Option<&str>,
    path: Option<&str>,
    here: bool,
) -> Result<()> {
    package::init_package(name, team, version, template, description, path, here).await
}

pub async fn init_app(
    name: Option<&str>,
    template: Option<&str>,
    description: Option<&str>,
    path: Option<&str>,
    here: bool,
) -> Result<()> {
    app::init_app(name, template, description, path, here).await
}
