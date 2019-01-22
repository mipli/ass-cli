#[derive(Debug)]
pub enum Error {
    InvalidAccountFile(String),
    NotFound(String),
    PermissionDenied(String),
    ArgumentParseError(String),
    RequestError,
    PathError,
    UrlParseError,
    JsonError,
    InvalidHeaderValue
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(_err: reqwest::header::InvalidHeaderValue) -> Error {
        Error::InvalidHeaderValue
    }
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Error {
        Error::ArgumentParseError(err.message)
    }
}

impl From<reqwest::UrlError> for Error {
    fn from(_err: reqwest::UrlError) -> Error {
        Error::UrlParseError
    }
}

impl From<std::io::Error> for Error {
    fn from(_err: std::io::Error) -> Error {
        Error::PathError
    }
}

impl From<reqwest::Error> for Error {
    fn from(_err: reqwest::Error) -> Error {
        Error::RequestError
    }
}

impl From<serde_json::Error> for Error {
    fn from(_err: serde_json::Error) -> Error {
        Error::JsonError
    }
}
