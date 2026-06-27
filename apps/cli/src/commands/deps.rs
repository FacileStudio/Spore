mod analysis;
mod inspection;
mod mutation;
mod shared;

use anyhow::Result;

pub async fn deps_add(
    package_spec: &str,
    app_name: Option<&str>,
    dev: bool,
    optional: bool,
) -> Result<()> {
    mutation::deps_add(package_spec, app_name, dev, optional).await
}

pub async fn deps_list(app_name: Option<&str>, tree: bool, depth: Option<usize>) -> Result<()> {
    inspection::deps_list(app_name, tree, depth).await
}

pub async fn deps_resolve(
    strategy: Option<&str>,
    dry_run: bool,
    app_name: Option<&str>,
) -> Result<()> {
    mutation::deps_resolve(strategy, dry_run, app_name).await
}

pub async fn deps_check() -> Result<()> {
    analysis::deps_check().await
}

pub async fn deps_tree(app_name: Option<&str>, depth: Option<usize>) -> Result<()> {
    inspection::deps_tree(app_name, depth).await
}

pub async fn deps_outdated(app_name: Option<&str>) -> Result<()> {
    analysis::deps_outdated(app_name).await
}

pub async fn deps_why(package_name: &str, app_name: Option<&str>) -> Result<()> {
    inspection::deps_why(package_name, app_name).await
}

pub async fn deps_sync() -> Result<()> {
    analysis::deps_sync().await
}
