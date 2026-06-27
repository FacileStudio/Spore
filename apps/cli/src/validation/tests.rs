use super::misc::validate_url;
use super::*;

#[test]
fn test_validate_package_name() {
    assert!(validate_package_name("my-package").is_ok());
    assert!(validate_package_name("@team/utils").is_ok());
    assert!(validate_package_name("utils123").is_ok());
    assert!(validate_package_name("ui.components").is_ok());

    assert!(validate_package_name("").is_err());
    assert!(validate_package_name(".invalid").is_err());
    assert!(validate_package_name("_invalid").is_err());
    assert!(validate_package_name("package..name").is_err());
    assert!(validate_package_name("node_modules").is_err());
}

#[test]
fn test_validate_semver() {
    assert!(validate_semver("1.0.0").is_ok());
    assert!(validate_semver("1.2.3-alpha.1").is_ok());
    assert!(validate_semver("2.0.0+build.1").is_ok());

    assert!(validate_semver("").is_err());
    assert!(validate_semver("1.0").is_err());
    assert!(validate_semver("invalid").is_err());
}

#[test]
fn test_validate_package_spec() {
    assert!(validate_package_spec("utils").is_ok());
    assert!(validate_package_spec("utils@1.0.0").is_ok());
    assert!(validate_package_spec("@team/utils@^1.0.0").is_ok());

    assert!(validate_package_spec("").is_err());
    assert!(validate_package_spec("@").is_err());
}

#[test]
fn test_validate_url() {
    assert!(validate_url("https://example.com", "URL").is_ok());
    assert!(validate_url("http://localhost:3000", "URL").is_ok());

    assert!(validate_url("", "URL").is_err());
    assert!(validate_url("ftp://example.com", "URL").is_err());
    assert!(validate_url("invalid-url", "URL").is_err());
}

#[test]
fn test_sanitize_input() {
    assert_eq!(sanitize_input("  hello world  "), "hello world");
    assert_eq!(sanitize_input("test\x00invalid"), "testinvalid");
    assert_eq!(sanitize_input("normal text"), "normal text");
}
