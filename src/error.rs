use failure::Fail;
use jsonwebtoken::errors::Error as JWTError;
use reqwest::Error as ReqwestError;
use serde_json::error::Error as JsonError;
use url::ParseError as UrlParseError;

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

    #[fail(display = "Non-parsable HTTP header: {}", _0)]
    InvalidHTTPHeader(#[cause] reqwest::header::ToStrError),

    #[fail(display = "Last modified data missing")]
    MissingLastModified,

    #[fail(display = "Invalid date format: {}", _0)]
    InvalidDateFormat(#[cause] chrono::format::ParseError),

    #[fail(display = "No series filter keys provided")]
    MissingSeriesFilterKeys,

    #[fail(display = "Image data is missing")]
    MissingImage,

    #[fail(display = "Invalid url: {}", _0)]
    InvalidUrl(#[cause] UrlParseError),

    #[fail(display = "Could not decode authentication JWT: {}", _0)]
    InvalidJWT(#[cause] JWTError),
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

impl From<reqwest::header::ToStrError> for Error {
    fn from(e: reqwest::header::ToStrError) -> Self {
        Self::InvalidHTTPHeader(e)
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(e: chrono::format::ParseError) -> Self {
        Self::InvalidDateFormat(e)
    }
}

impl From<UrlParseError> for Error {
    fn from(e: UrlParseError) -> Self {
        Self::InvalidUrl(e)
    }
}

impl From<JWTError> for Error {
    fn from(e: JWTError) -> Self {
        Self::InvalidJWT(e)
    }
}
