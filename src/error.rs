use derive_more::*;

use ass_rs::AssError;
use std::error::Error;

#[derive(Debug, Display)]
#[display(fmt = "{}", kind)]
pub struct AssCliError {
    pub kind: AssCliErrorKind,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

#[derive(Debug, Display, Eq, PartialEq)]
pub enum AssCliErrorKind {
    #[display(fmt = "Error accessing ASS")]
    AssError,
    #[display(fmt = "Error parsing arguments: {}", .0)]
    ArgumentParseError(String),
    #[display(fmt = "Error parsing json")]
    JsonError,
    #[display(fmt = "Error accessing path")]
    PathError,
    #[display(fmt = "Std IO Error")]
    StdIOError,
}

impl AssCliError {
    pub fn json_error() -> Self {
        AssCliError {
            kind: AssCliErrorKind::JsonError,
            source: None,
        }
    }

    pub fn path_error() -> Self {
        AssCliError {
            kind: AssCliErrorKind::PathError,
            source: None,
        }
    }
}

impl From<clap::Error> for AssCliError {
    fn from(err: clap::Error) -> AssCliError {
        AssCliError {
            kind: AssCliErrorKind::ArgumentParseError(err.message.clone()),
            source: Some(Box::new(err)),
        }
    }
}

impl From<AssError> for AssCliError {
    fn from(err: AssError) -> AssCliError {
        AssCliError {
            kind: AssCliErrorKind::AssError,
            source: Some(Box::new(err)),
        }
    }
}

impl From<std::io::Error> for AssCliError {
    fn from(err: std::io::Error) -> AssCliError {
        AssCliError {
            kind: AssCliErrorKind::StdIOError,
            source: Some(Box::new(err)),
        }
    }
}
