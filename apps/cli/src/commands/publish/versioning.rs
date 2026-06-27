use anyhow::Result;

fn is_valid_semver(version: &str) -> bool {
    let version_main = if let Some(hyphen_pos) = version.find('-') {
        &version[..hyphen_pos]
    } else {
        version
    };

    let parts: Vec<&str> = version_main.split('.').collect();
    if parts.len() != 3 {
        return false;
    }

    for part in &parts {
        if part.is_empty() {
            return false;
        }

        if part.len() > 1 && part.starts_with('0') {
            return false;
        }

        if part.parse::<u32>().is_err() {
            return false;
        }
    }

    if let Some(hyphen_pos) = version.find('-') {
        let prerelease = &version[hyphen_pos + 1..];
        if prerelease.is_empty() {
            return false;
        }

        if !prerelease
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
        {
            return false;
        }
    }

    true
}

pub fn validate_publish_version(version: &str) -> Result<()> {
    if version.trim().is_empty() {
        anyhow::bail!("Version cannot be empty");
    }

    if !is_valid_semver(version) {
        anyhow::bail!(
            "Invalid version format '{}'. Must follow semantic versioning (e.g., 1.2.3)",
            version
        );
    }

    if version.contains("-dev") || version.contains("-development") {
        anyhow::bail!(
            "Cannot publish development version '{}'. \n💡 Use 'spore version set <version>' to set a proper release version first.",
            version
        );
    }

    if version.contains("-alpha") || version.contains("-beta") || version.contains("-rc") {
        println!("⚠️  Publishing prerelease version: {}", version);
        println!("💡 Use 'spore version bump major|minor|patch' for stable releases");
    }

    if version == "0.0.0" {
        anyhow::bail!(
            "Cannot publish version '0.0.0'. \n💡 Use 'spore version set <version>' to set a proper version first."
        );
    }

    println!("📋 Publishing version: {}", version);
    Ok(())
}
