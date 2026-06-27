use anyhow::{Context, Result};
use std::path::Path;

use super::Project;

impl Project {
    pub(crate) fn create_package_link_static(source_path: &Path, target_path: &Path) -> Result<()> {
        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {}", parent.display()))?;
        }

        if target_path.exists() {
            if target_path.is_dir() {
                std::fs::remove_dir_all(target_path)
            } else {
                std::fs::remove_file(target_path)
            }
            .with_context(|| format!("Failed to remove existing path {}", target_path.display()))?;
        }

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(source_path, target_path).with_context(|| {
                format!(
                    "Failed to create symlink from {} to {}",
                    source_path.display(),
                    target_path.display()
                )
            })?;
        }

        #[cfg(not(unix))]
        {
            Self::copy_dir_recursive_static(source_path, target_path).with_context(|| {
                format!(
                    "Failed to copy from {} to {}",
                    source_path.display(),
                    target_path.display()
                )
            })?;
        }

        Ok(())
    }

    pub(crate) fn copy_dir_recursive_static(src: &Path, dst: &Path) -> std::io::Result<()> {
        use std::fs;

        fs::create_dir_all(dst)?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                Self::copy_dir_recursive_static(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }
}
