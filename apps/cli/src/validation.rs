mod base;
mod misc;
mod naming;
mod paths;
mod versions;

pub use base::ValidationError;
pub use misc::{
    confirm_destructive_action, sanitize_input, validate_description, validate_prerelease_id,
    validate_script_name,
};
pub use naming::{
    validate_app_name, validate_package_name, validate_project_name, validate_team_name,
    validate_template_name,
};
pub use paths::validate_path;
pub use versions::{validate_package_spec, validate_semver};

#[cfg(test)]
mod tests;
