use failure::Fail;
use reqwest::Error as ReqwestError;
use serde_json::error::Error as JsonError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "JSON error: {}", _0)]
    JSON(#[cause] JsonError),

    #[fail(display = "HTTP error: {}", _0)]
    HTTP(#[cause] ReqwestError),

    #[fail(display = "IO error: {}", _0)]
    IO(#[cause] std::io::Error),

    #[fail(display = "Invalid API key")]
    InvalidAPIKey,

    #[fail(display = "API Server error")]
    ServerError,

    #[fail(display = "Not found")]
    NotFound,
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Self {
        Self::JSON(e)
    }
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Self {
        Self::HTTP(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}
