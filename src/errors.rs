use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

/// Indicating the type of error produced
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    DuplicateFlag,
    DuplicateArgument,
    MissingArgument,
    MissingValue,
    MissingFlag,
    WantsHelp,
    Other,
}

/// The error itself that is returned
#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    diagnosis: String,
}

impl Error {
    /// Constructor for Error
    pub fn new(kind: ErrorKind, diagnosis: String) -> Self {
        Self { kind, diagnosis }
    }

    /// Getter
    pub fn kind(&self) -> ErrorKind {
        self.kind.clone()
    }

    /// Getter
    pub fn diagnosis(&self) -> String {
        self.diagnosis.clone()
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        self.kind == other.kind
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.diagnosis)
    }
}
