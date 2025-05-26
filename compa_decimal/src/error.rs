use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompaDecimalError {
    pub error_message: String
}

impl Display for CompaDecimalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

impl Default for CompaDecimalError {
    fn default() -> Self {
        Self { error_message: String::new() }
    }
}

impl std::error::Error for CompaDecimalError { }