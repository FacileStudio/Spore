use super::ConfigType;

#[allow(dead_code)]
pub fn parse_yaml_error_to_user_friendly(error: &serde_yaml::Error) -> String {
    parse_yaml_error_with_context(error, ConfigType::Unknown)
}

pub fn parse_yaml_error_with_context(error: &serde_yaml::Error, config_type: ConfigType) -> String {
    let error_msg = error.to_string();
    let location_info = extract_error_location(&error_msg);
    let field_name = extract_field_name(&error_msg);

    if error_msg.contains("missing field") {
        if let Some(field) = field_name {
            let suggestion = get_field_suggestion_with_context(&field, config_type);
            return format_error_with_location(
                &format_missing_field_error_with_context(&field, suggestion, config_type),
                &location_info,
            );
        }
        return format_error_with_location("Missing required field", &location_info);
    }

    if error_msg.contains("invalid type") {
        let type_error = parse_type_error(&error_msg, field_name.as_deref());
        return format_error_with_location(&type_error, &location_info);
    }

    if error_msg.contains("unknown field") {
        if let Some(field) = field_name {
            let suggestion = suggest_similar_field(&field);
            return format_error_with_location(
                &format!("Unknown field '{}'{}", field, suggestion),
                &location_info,
            );
        }
        return format_error_with_location("Unknown field found", &location_info);
    }

    if error_msg.contains("duplicate key") {
        if let Some(field) = field_name {
            return format_error_with_location(
                &format!(
                    "Duplicate field '{}' found. Each field can only be defined once.",
                    field
                ),
                &location_info,
            );
        }
        return format_error_with_location("Duplicate field found", &location_info);
    }

    if error_msg.contains("while parsing") {
        let syntax_error = parse_yaml_syntax_error(&error_msg);
        return format_error_with_location(&syntax_error, &location_info);
    }

    if error_msg.contains("bad indentation")
        || error_msg.contains("found character that cannot start any token")
    {
        return format_error_with_location(
            "Invalid YAML indentation. Make sure you use consistent spaces (not tabs) for indentation.",
            &location_info,
        );
    }

    if error_msg.contains("while scanning") {
        if error_msg.contains("quote") {
            return format_error_with_location(
                "Unclosed quotes detected. Make sure all strings are properly quoted.",
                &location_info,
            );
        }
        if error_msg.contains("mapping") {
            return format_error_with_location(
                "Invalid mapping structure. Check for missing colons or incorrect nesting.",
                &location_info,
            );
        }
    }

    if error_msg.contains("invalid value") {
        return format_error_with_location(
            "Invalid value format. Check the data type and format requirements.",
            &location_info,
        );
    }

    if !error_msg.is_empty() {
        return format_error_with_location(&error_msg, &location_info);
    }

    "Unknown YAML parsing error occurred".to_string()
}

fn extract_error_location(error_msg: &str) -> Option<(usize, usize)> {
    if let Some(line_start) = error_msg.find("line ") {
        if let Some(line_end) = error_msg[line_start + 5..].find(char::is_whitespace) {
            let line_str = &error_msg[line_start + 5..line_start + 5 + line_end];
            if let Ok(line) = line_str.parse::<usize>() {
                if let Some(col_start) = error_msg.find("column ") {
                    if let Some(col_end) = error_msg[col_start + 7..].find(char::is_whitespace) {
                        let col_str = &error_msg[col_start + 7..col_start + 7 + col_end];
                        if let Ok(col) = col_str.parse::<usize>() {
                            return Some((line, col));
                        }
                    }
                }
                return Some((line, 0));
            }
        }
    }

    None
}

fn extract_field_name(error_msg: &str) -> Option<String> {
    if let Some(start) = error_msg.find('`') {
        if let Some(end) = error_msg[start + 1..].find('`') {
            return Some(error_msg[start + 1..start + 1 + end].to_string());
        }
    }

    if let Some(start) = error_msg.find("field \"") {
        if let Some(end) = error_msg[start + 7..].find('"') {
            return Some(error_msg[start + 7..start + 7 + end].to_string());
        }
    }

    None
}

fn format_error_with_location(message: &str, location: &Option<(usize, usize)>) -> String {
    match location {
        Some((line, 0)) => format!("{} (line {})", message, line),
        Some((line, col)) => format!("{} (line {}, column {})", message, line, col),
        None => message.to_string(),
    }
}

#[allow(dead_code)]
fn format_missing_field_error(field_name: &str, suggestion: Option<String>) -> String {
    format_missing_field_error_with_context(field_name, suggestion, ConfigType::Unknown)
}

fn format_missing_field_error_with_context(
    field_name: &str,
    suggestion: Option<String>,
    config_type: ConfigType,
) -> String {
    let base_message = match (field_name, config_type) {
        ("name", ConfigType::Spore) => {
            "Missing required field 'name'. Every spore.yml configuration must have a project name."
        }
        ("name", ConfigType::Package) => {
            "Missing required field 'name'. Every package.yml configuration must have a package name."
        }
        ("name", ConfigType::App) => {
            "Missing required field 'name'. Every app.yml configuration must have an app name."
        }
        ("name", _) => "Missing required field 'name'. Every configuration must have a name.",
        ("version", ConfigType::Package) => {
            "Missing required field 'version'. Package configurations must specify a version."
        }
        ("version", _) => {
            "Missing required field 'version'. This configuration requires a version."
        }
        _ => return format!("Missing required field '{}'", field_name),
    };

    match suggestion {
        Some(suggestion) => format!("{}\n{}", base_message, suggestion),
        None => base_message.to_string(),
    }
}

#[allow(dead_code)]
fn get_field_suggestion(field_name: &str) -> Option<String> {
    get_field_suggestion_with_context(field_name, ConfigType::Unknown)
}

fn get_field_suggestion_with_context(field_name: &str, config_type: ConfigType) -> Option<String> {
    match (field_name, config_type) {
        ("name", ConfigType::Spore) => Some(
            "Suggestion: Add 'name: \"your-project-name\"' to your spore.yml file.".to_string(),
        ),
        ("name", ConfigType::Package) => Some(
            "Suggestion: Add 'name: \"your-package-name\"' to your package.yml file.".to_string(),
        ),
        ("name", ConfigType::App) => Some(
            "Suggestion: Add 'name: \"your-app-name\"' to your app.yml file.".to_string(),
        ),
        ("name", _) => Some(
            "Suggestion: Add 'name: \"your-configuration-name\"' to your configuration."
                .to_string(),
        ),
        ("version", ConfigType::Package) => Some(
            "Suggestion: Add 'version: \"1.0.0\"' using semantic versioning format (major.minor.patch).".to_string(),
        ),
        ("version", _) => Some(
            "Suggestion: Add 'version: \"1.0.0\"' using semantic versioning format.".to_string(),
        ),
        ("description", _) => Some(
            "Suggestion: Add 'description: \"Brief description of your project\"' (optional but recommended).".to_string(),
        ),
        ("scripts", ConfigType::Spore) => Some(
            "Suggestion: Add scripts like 'scripts:\n  start: \"command to start\"' to define project-level scripts.".to_string(),
        ),
        ("scripts", ConfigType::Package) => Some(
            "Suggestion: Add scripts like 'scripts:\n  build: \"build command\"' to define package-level scripts.".to_string(),
        ),
        ("scripts", ConfigType::App) => Some(
            "Suggestion: Add scripts like 'scripts:\n  dev: \"development command\"' to define app-level scripts.".to_string(),
        ),
        ("packages", ConfigType::App) => Some(
            "Suggestion: Add packages like 'packages:\n  - package-name' to define app dependencies."
                .to_string(),
        ),
        ("apps", ConfigType::Spore) => Some(
            "Suggestion: Add apps like 'apps:\n  app-name:\n    - package-name' to define project apps.".to_string(),
        ),
        _ => None,
    }
}

fn parse_type_error(error_msg: &str, field_name: Option<&str>) -> String {
    let base_error = if error_msg.contains("expected a string") {
        "Expected string value"
    } else if error_msg.contains("expected a sequence") || error_msg.contains("expected array") {
        "Expected array/list value"
    } else if error_msg.contains("expected a map") || error_msg.contains("expected object") {
        "Expected object/mapping value"
    } else if error_msg.contains("expected a boolean") {
        "Expected boolean value (true/false)"
    } else if error_msg.contains("expected a number") || error_msg.contains("expected integer") {
        "Expected numeric value"
    } else {
        "Invalid field type"
    };

    match field_name {
        Some("scripts") => format!(
            "{} for 'scripts'. Use format: scripts:\n  script-name: \"command\"",
            base_error
        ),
        Some("packages") => format!(
            "{} for 'packages'. Use format: packages:\n  - package-name",
            base_error
        ),
        Some("apps") => format!(
            "{} for 'apps'. Use either list format or object format with tsAlias and packages.",
            base_error
        ),
        Some("variables") => format!(
            "{} for 'variables'. Use format: variables:\n  var-name: \"value\"",
            base_error
        ),
        Some("tsAlias") => {
            "Expected boolean or string for 'tsAlias'. Use: true, false, or \"@\"".to_string()
        }
        Some(field) => format!("{} for field '{}'", base_error, field),
        None => base_error.to_string(),
    }
}

fn suggest_similar_field(field_name: &str) -> String {
    let suggestions = match field_name.to_lowercase().as_str() {
        "script" => Some("scripts"),
        "package" => Some("packages"),
        "app" => Some("apps"),
        "variable" => Some("variables"),
        "tsalias" => Some("tsAlias"),
        "desc" | "description" => Some("description"),
        "ver" | "version" => Some("version"),
        "dependencies" => Some("packages (for app config) or dependencies (for package config)"),
        _ => None,
    };

    match suggestions {
        Some(suggestion) => format!(". Did you mean '{}'?", suggestion),
        None => ". Check the field name for typos.".to_string(),
    }
}

fn parse_yaml_syntax_error(error_msg: &str) -> String {
    if error_msg.contains("mapping") {
        "Invalid YAML mapping. Check for missing colons (:) after field names."
    } else if error_msg.contains("sequence") {
        "Invalid YAML sequence. Check for proper list formatting with dashes (-)."
    } else if error_msg.contains("scalar") {
        "Invalid YAML scalar value. Check for proper quoting of string values."
    } else if error_msg.contains("anchor") {
        "Invalid YAML anchor or alias. Check anchor definitions and references."
    } else {
        "Invalid YAML syntax. Check indentation, colons, and list formatting."
    }
    .to_string()
}
