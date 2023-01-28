use std::{error::Error as StdError, fmt::Display};

#[derive(Debug)]
pub struct Error {
    source: Box<dyn StdError>,
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    LoginFailed,
    FetchFailed(String),
    EnvError(&'static str),
}

impl ErrorKind {
    fn description(&self) -> String {
        match self {
            Self::LoginFailed => String::from("Failed to log into Dualis"),
            Self::FetchFailed(msg) => format!("Failed to fetch '{}'", msg),
            Self::EnvError(env) => format!("Environment variable '{env}' missing"),
        }
    }
}

impl Error {
    fn new(source: Box<dyn StdError>, kind: ErrorKind) -> Self {
        Self { source, kind }
    }

    pub fn login_failed(source: Box<dyn StdError>) -> Self {
        Self::new(source, ErrorKind::LoginFailed)
    }

    pub fn fetch_failed(source: Box<dyn StdError>, what: String) -> Self {
        Self::new(source, ErrorKind::FetchFailed(what))
    }

    pub fn env_error(source: Box<dyn StdError>, what: &'static str) -> Self {
        Self::new(source, ErrorKind::EnvError(what))
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.source.as_ref())
    }


    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn StdError> {
        self.source()
    }

}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind.description())
    }
}