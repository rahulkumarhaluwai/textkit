use std::fmt;

#[derive(Debug)]
pub enum TextkitError {
    MissingArgument(String),
    UnknownCommand(String),
    Io(std::io::Error),
}

impl fmt::Display for TextkitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextkitError::MissingArgument(what) => write!(f, "missing argument: {what}"),
            TextkitError::UnknownCommand(cmd) => write!(f, "unknown command: {cmd}"),
            TextkitError::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl std::error::Error for TextkitError {}

impl From<std::io::Error> for TextkitError {
    fn from(e: std::io::Error) -> Self {
        TextkitError::Io(e)
    }
}