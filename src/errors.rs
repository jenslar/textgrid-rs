use std::fmt;

#[derive(Debug)]
pub enum TgError {
    SerializeError
}

impl std::error::Error for TgError {}
impl fmt::Display for TgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SerializeError => write!(f, "Error serializing TextGrid."),
        }
    }
}