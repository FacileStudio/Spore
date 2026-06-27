#[path = "config/errors.rs"]
mod errors;
#[path = "config/loading.rs"]
mod loading;
#[path = "config/types.rs"]
mod types;
#[path = "config/validation.rs"]
mod validation;

#[allow(unused_imports)]
pub use errors::{parse_yaml_error_to_user_friendly, parse_yaml_error_with_context};
#[allow(unused_imports)]
pub use types::{
    AppConfig, AppDependencies, ConfigType, ConfigVariable, SporeConfig, PackageConfig, TsAlias,
};
