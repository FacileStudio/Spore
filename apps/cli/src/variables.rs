mod context;
mod interpolation;

pub use context::{VariableContext, VariableInfo, VariableSource};
pub use interpolation::VariableInterpolation;

#[cfg(test)]
mod tests;
