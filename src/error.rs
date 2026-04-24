use std::fmt;

#[derive(Debug)]
pub enum SeymourError {
    ParseError(String),
}

impl fmt::Display for SeymourError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SeymourError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for SeymourError {}
