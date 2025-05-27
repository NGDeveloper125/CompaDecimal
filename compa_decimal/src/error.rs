use std::fmt::Display;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CompaDecimalError {
    pub error_message: String
}

impl Display for CompaDecimalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

impl std::error::Error for CompaDecimalError { }