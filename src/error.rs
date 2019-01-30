use failure::{*};
use ass_rs::{AssError};

#[derive(Fail, Debug)]
pub enum AssCliError {
    #[fail(display = "Error accessing ASS: {}", err)]
    AssError {
        err: AssError
    },
    #[fail(display = "Error parsing arguments: {}", message)]
    ArgumentParseError {
        message: String
    },
    #[fail(display = "Error parsing json")]
    JsonError,
    #[fail(display = "Error accessing path")]
    PathError,
    #[fail(display = "Std IO Error")]
    StdIOError,
}

impl From<clap::Error> for AssCliError {
    fn from(err: clap::Error) -> AssCliError {
        AssCliError::ArgumentParseError{ message: err.message }
    }
}

impl From<serde_json::Error> for AssCliError {
    fn from(_err: serde_json::Error) -> AssCliError {
        AssCliError::JsonError
    }
}

impl From<AssError> for AssCliError {
    fn from(err: AssError) -> AssCliError {
        AssCliError::AssError {
            err
        }
    }
}

impl From<std::io::Error> for AssCliError {
    fn from(_err: std::io::Error) -> AssCliError {
        AssCliError::StdIOError
    }
}
