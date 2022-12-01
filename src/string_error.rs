use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct StringError {
    cause: String,
}

impl StringError {
    pub fn new(cause: String) -> Self {
        Self { cause }
    }
}

impl Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cause.fmt(f)
    }
}

impl From<&str> for StringError {
    fn from(s: &str) -> Self {
        StringError::new(s.to_owned())
    }
}

impl From<String> for StringError {
    fn from(s: String) -> Self {
        StringError::new(s)
    }
}

impl Error for StringError {}
