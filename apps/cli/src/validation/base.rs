#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub example: Option<String>,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)?;
        if let Some(example) = &self.example {
            write!(f, "\nExample: {}", example)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationError {}
