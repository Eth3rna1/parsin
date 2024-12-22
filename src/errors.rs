use std::fmt;
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
    pub fn new(kind: ErrorKind, diagnosis: String) -> Self {
        Self { kind, diagnosis }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind.clone()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result {
        write!(f, "{}", self.diagnosis)
    }
}

/*
impl ToString for Error {
    /// Returns the diagnosis
    fn to_string(&self) -> String {
        self.diagnosis.clone()
    }
}
*/
