use std::fmt;
use std::io;

#[derive(Debug)]
pub enum PodmanError {
    Command(io::Error),
}

impl std::error::Error for PodmanError {}

impl fmt::Display for PodmanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PodmanError::Command(error) => write!(f, "{}", error),
        }
    }
}

impl From<io::Error> for PodmanError {
    fn from(error: io::Error) -> Self {
        PodmanError::Command(error)
    }
}
