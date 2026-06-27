use anyhow::{Context, Result};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::path::Path;
use tar::Builder;

use crate::ignore::SporeIgnore;

pub fn create_package_tarball(output_path: &str) -> Result<()> {
    let tar_gz = fs::File::create(output_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    let current_dir = std::env::current_dir()?;
    let sporeignore_path = current_dir.join(".sporeignore");

    let ignore = match SporeIgnore::from_file(&sporeignore_path) {
        Ok(ignore) => {
            if sporeignore_path.exists() {
                println!(
                    "📋 Using .sporeignore file with {} patterns",
                    ignore.patterns().len()
                );
            } else {
                println!("📋 Using default ignore patterns (create .sporeignore to customize)");
            }
            ignore
        }
        Err(e) => {
            println!("⚠️  Failed to read .sporeignore file: {}", e);
            println!("📋 Using default ignore patterns");
            SporeIgnore::default()
        }
    };

    add_directory_to_tar(&mut tar, &current_dir, ".", &ignore)?;
    tar.finish()?;

    println!("📦 Created package tarball: {}", output_path);
    Ok(())
}

fn add_directory_to_tar(
    tar: &mut Builder<GzEncoder<fs::File>>,
    base_path: &Path,
    relative_path: &str,
    ignore: &SporeIgnore,
) -> Result<()> {
    let dir_path = base_path.join(relative_path);

    for entry in fs::read_dir(&dir_path)? {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        let file_path = entry.path();

        let entry_relative_path = if relative_path == "." {
            file_name.to_string()
        } else {
            format!("{}/{}", relative_path, file_name)
        };

        if ignore.is_ignored(&entry_relative_path) || ignore.is_ignored(&file_name) {
            continue;
        }

        if file_path.is_dir() {
            add_directory_to_tar(tar, base_path, &entry_relative_path, ignore)?;
        } else {
            let file = fs::File::open(&file_path)
                .with_context(|| format!("Failed to open file: {}", file_path.display()))?;
            let metadata = file.metadata().with_context(|| {
                format!("Failed to get metadata for file: {}", file_path.display())
            })?;

            let mut header = tar::Header::new_gnu();
            header.set_size(metadata.len());
            header.set_mode(0o644);
            header.set_cksum();

            tar.append_data(&mut header, &entry_relative_path, file)
                .with_context(|| {
                    format!("Failed to add file to archive: {}", entry_relative_path)
                })?;
        }
    }

    Ok(())
}
