use anyhow::Result;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

use crate::downloader::PackageDownloader;

pub(super) async fn use_package_as_template(
    package_spec: &str,
    target_dir: &Path,
    name: Option<&str>,
    version: Option<&str>,
    description: Option<&str>,
) -> Result<()> {
    println!("📥 Downloading template package: {}", package_spec);

    let temp_dir = TempDir::new()?;
    let download_path = temp_dir.path();

    PackageDownloader::download_package(package_spec, download_path).await?;
    copy_template_files(download_path, target_dir)?;
    update_config_files(target_dir, name, version, description)?;

    println!(
        "✅ Successfully applied template from package: {}",
        package_spec
    );

    Ok(())
}

fn copy_template_files(source: &Path, destination: &Path) -> Result<()> {
    for entry in walkdir::WalkDir::new(source) {
        let entry = entry?;
        let path = entry.path();

        if path == source {
            continue;
        }

        let relative_path = path.strip_prefix(source)?;
        let dest_path = destination.join(relative_path);

        if entry.file_type().is_dir() {
            if !dest_path.exists() {
                fs::create_dir_all(&dest_path)?;
            }
        } else {
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            if file_name == "package.yml" || file_name == "app.yml" || file_name == "spore.yml" {
                continue;
            }

            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &dest_path)?;
        }
    }

    Ok(())
}

fn update_config_files(
    target_dir: &Path,
    name: Option<&str>,
    version: Option<&str>,
    description: Option<&str>,
) -> Result<()> {
    let package_json_path = target_dir.join("package.json");
    if package_json_path.exists() {
        let content = fs::read_to_string(&package_json_path)?;
        if let Ok(mut json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(name) = name {
                json["name"] = serde_json::Value::String(name.to_string());
            }
            if let Some(version) = version {
                json["version"] = serde_json::Value::String(version.to_string());
            }
            if let Some(description) = description {
                json["description"] = serde_json::Value::String(description.to_string());
            }
            let updated_content = serde_json::to_string_pretty(&json)?;
            fs::write(&package_json_path, updated_content)?;
        }
    }

    Ok(())
}

pub(super) fn create_basic_structure(target_dir: &Path, is_package: bool) -> Result<()> {
    let src_dir = target_dir.join("src");
    fs::create_dir_all(&src_dir)?;

    let index_path = src_dir.join("index.ts");
    let content = if is_package {
        "// Your package code here\nexport {};"
    } else {
        "// Your app code here\nconsole.log('Hello from Spore!');"
    };
    fs::write(index_path, content)?;

    let package_json = serde_json::json!({
        "name": target_dir.file_name().and_then(|name| name.to_str()).unwrap_or("my-project"),
        "version": "1.0.0",
        "main": "src/index.ts",
        "scripts": {
            "dev": "node src/index.ts"
        }
    });
    let package_json_path = target_dir.join("package.json");
    fs::write(
        &package_json_path,
        serde_json::to_string_pretty(&package_json)?,
    )?;

    Ok(())
}
