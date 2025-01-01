use std::fmt;

#[derive(Debug)]
pub enum AocError {
    IoError(std::io::Error),
    ParseError(String),
    SolutionError(String),
    ValidationError(String),
}

impl std::error::Error for AocError {}
impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AocError::IoError(err) => write!(f, "IO Error: {}", err),
            AocError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            AocError::SolutionError(msg) => write!(f, "Solution Error: {}", msg),
            AocError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
        }
    }
}
