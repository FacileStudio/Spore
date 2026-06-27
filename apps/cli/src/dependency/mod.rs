pub mod cache;
pub mod error;
pub mod registry;
pub mod resolver;
pub mod spec;
pub mod types;

pub use resolver::DependencyResolver;
pub use spec::DependencyRef;
pub use types::{DependencySpec, PackageId, ResolutionContext, ResolutionStrategy};
