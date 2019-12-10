#![deny(missing_docs, missing_debug_implementations, unsafe_code)]

//! Errors that may occur while using this crate.

use std::io::Error as IOError;

use chrono::format::ParseError as TimeParseError;
use failure::Fail;
use jsonwebtoken::errors::Error as JWTError;
use reqwest::Error as ReqwestError;
use serde_json::error::Error as JSONError;
use url::ParseError as URLParseError;

/// `Result` with error case set to `thetvdb::error::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type containing possible failure cases
/// of this crate.
#[derive(Debug, Fail)]
pub enum Error {
    /// Occurs when parsing JSON data fails.
    #[fail(display = "JSON error: {}", _0)]
    JSON(#[cause] JSONError),

    /// Occurs when [`reqwest`](https://docs.rs/reqwest/latest/reqwest/index.html),
    /// the HTTP client underlying this crate, returns an error.
    #[fail(display = "HTTP error: {}", _0)]
    HTTP(#[cause] ReqwestError),

    /// IO error from `std`.
    #[fail(display = "IO error: {}", _0)]
    IO(#[cause] IOError),

    /// Occurs when the provided API key is not valid.
    #[fail(display = "Invalid API key")]
    InvalidAPIKey,

    /// Occurs when TheTVDB API returns a `5XX` error response.
    #[fail(display = "API Server error")]
    ServerError,

    /// Occurs when resources (series, episodes, etc...) are
    /// not found.
    #[fail(display = "Not found")]
    NotFound,

    /// Occurs when a header returned by the API is not representable
    /// as a string.
    ///
    /// See
    /// [`reqwest::header::ToStrError`](https://docs.rs/reqwest/latest/reqwest/header/struct.ToStrError.html)
    /// for more info.
    #[fail(display = "Non-parsable HTTP header: {}", _0)]
    InvalidHTTPHeader(#[cause] reqwest::header::ToStrError),

    /// Occurs when the API doesn't return a header containing
    /// the date and time when the series was last modified.
    #[fail(display = "Last modified data missing")]
    MissingLastModified,

    /// Occurs when the API returns dates and times in formats
    /// that are not known by this crate.
    #[fail(display = "Invalid date format: {}", _0)]
    InvalidDateFormat(#[cause] TimeParseError),

    /// Occurs when [`Client.series_filter`](../client/struct.Client.html#method.series_filter)
    /// is called with empty `SeriesFilterKeys`.
    #[fail(display = "No series filter keys provided")]
    MissingSeriesFilterKeys,

    /// Occurs when a image URL method is called, but the
    /// image file path is not known.
    #[fail(display = "Image data is missing")]
    MissingImage,

    /// Occurs when a series website URL method is called,
    /// but the slug is not known.
    #[fail(display = "Series slug is missing")]
    MissingSeriesSlug,

    /// Occurs when a URL cannot be parsed.
    #[fail(display = "Invalid url: {}", _0)]
    InvalidUrl(#[cause] URLParseError),

    /// Occurs when the JWT returned by the API on login is invalid.
    #[fail(display = "Could not decode authentication JWT: {}", _0)]
    InvalidJWT(#[cause] JWTError),
}

impl From<JSONError> for Error {
    fn from(e: JSONError) -> Self {
        Self::JSON(e)
    }
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Self {
        Self::HTTP(e)
    }
}

impl From<IOError> for Error {
    fn from(e: IOError) -> Self {
        Self::IO(e)
    }
}

impl From<reqwest::header::ToStrError> for Error {
    fn from(e: reqwest::header::ToStrError) -> Self {
        Self::InvalidHTTPHeader(e)
    }
}

impl From<TimeParseError> for Error {
    fn from(e: TimeParseError) -> Self {
        Self::InvalidDateFormat(e)
    }
}

impl From<URLParseError> for Error {
    fn from(e: URLParseError) -> Self {
        Self::InvalidUrl(e)
    }
}

impl From<JWTError> for Error {
    fn from(e: JWTError) -> Self {
        Self::InvalidJWT(e)
    }
}

#[cfg(test)]
pub(crate) mod test_util {
    use super::*;

    pub fn wrong_error_kind(expected: Error, got: Error) {
        panic!("Wrong error kind: expected {:?}, got {:?}", expected, got);
    }
}
