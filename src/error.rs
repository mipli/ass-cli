use derive_more::*;

use ass_rs::{AssError, AssErrorKind};
use std::error::Error;

#[derive(Debug, Display)]
#[display(fmt = "{}: {:?}", kind, source)]
pub struct AssCliError {
    pub kind: AssCliErrorKind,
    pub source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

#[derive(Debug, Display, Eq, PartialEq)]
pub enum AssCliErrorKind {
    #[display(fmt = "Invalid arguments, use --help for more information")]
    CommandError,
    #[display(fmt = "Error accessing ASS: {}", _0)]
    AssError(String),
    #[display(fmt = "Url does not match account")]
    UrlDoesNotMatchAccount,
    #[display(fmt = "Invalid account file: {}", _0)]
    InvalidAccountFile(String),
    #[display(fmt = "Error parsing arguments: {}", _0)]
    ArgumentParseError(String),
    #[display(fmt = "Error parsing json")]
    JsonError,
    #[display(fmt = "Error accessing path")]
    PathError,
    #[display(fmt = "Std IO Error")]
    StdIOError,
}

impl AssCliError {
    pub fn argument_parse_error(msg: String) -> Self {
        AssCliError {
            kind: AssCliErrorKind::ArgumentParseError(msg),
            source: None,
        }
    }

    pub fn invalid_account_file(msg: String) -> Self {
        AssCliError {
            kind: AssCliErrorKind::InvalidAccountFile(msg),
            source: None,
        }
    }

    pub fn command_error() -> Self {
        AssCliError {
            kind: AssCliErrorKind::CommandError,
            source: None,
        }
    }

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
        let kind = match err.kind {
            AssErrorKind::UrlDoesNotMatchAccount(_) => AssCliErrorKind::UrlDoesNotMatchAccount,
            _ => AssCliErrorKind::AssError(err.to_string()),
        };
        AssCliError {
            kind,
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
