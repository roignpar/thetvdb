//! Errors that may occur while using this crate.

use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IOError;

use chrono::format::ParseError as TimeParseError;
use jsonwebtoken::errors::Error as JWTError;
use reqwest::Error as ReqwestError;
use url::ParseError as URLParseError;

/// `Result` with error case set to `thetvdb::error::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type containing possible failure cases of this crate.
#[derive(Debug)]
pub enum Error {
    /// Occurs when [`reqwest`], the HTTP client underlying this crate, returns
    /// an error.
    ///
    /// [`reqwest`]: https://docs.rs/reqwest/latest/reqwest/index.html
    HTTP(ReqwestError),

    /// IO error from `std`.
    IO(IOError),

    /// Occurs when the provided API key is not valid.
    InvalidAPIKey,

    /// Occurs when TheTVDB API returns a `5XX` error response.
    ServerError,

    /// Occurs when resources (series, episodes, etc...) are not found.
    NotFound,

    /// Occurs when a header returned by the API is not representable as a
    /// string.
    ///
    /// See [`reqwest::header::ToStrError`] for more info.
    ///
    /// [`reqwest::header::ToStrError`]: https://docs.rs/reqwest/latest/reqwest/header/struct.ToStrError.html
    InvalidHTTPHeader(reqwest::header::ToStrError),

    /// Occurs when the API doesn't return a header containing the date and time
    /// when the series was last modified.
    MissingLastModified,

    /// Occurs when the API returns dates and times in formats that are not
    /// known by this crate.
    InvalidDateFormat(TimeParseError),

    /// Occurs when [`Client::series_filter`] is called with empty
    /// `SeriesFilterKeys`.
    ///
    /// [`Client::series_filter`]: ../client/struct.Client.html#method.series_filter
    MissingSeriesFilterKeys,

    /// Occurs when a image URL method is called, but the image file path is not
    /// known.
    MissingImage,

    /// Occurs when a series website URL method is called, but the slug is not
    /// known.
    MissingSeriesSlug,

    /// Occurs when a URL cannot be parsed.
    InvalidUrl(URLParseError),

    /// Occurs when the JWT returned by the API on login is invalid.
    InvalidJWT(JWTError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            HTTP(e) => write!(f, "HTTP error: {}", e),
            IO(e) => write!(f, "IO error: {}", e),
            InvalidAPIKey => write!(f, "Invalid API key"),
            ServerError => write!(f, "API Server error"),
            NotFound => write!(f, "Not found"),
            InvalidHTTPHeader(e) => write!(f, "Non-parsable HTTP header: {}", e),
            MissingLastModified => write!(f, "Last modified data missing"),
            InvalidDateFormat(e) => write!(f, "Invalid date format: {}", e),
            MissingSeriesFilterKeys => write!(f, "No series filter keys provided"),
            MissingImage => write!(f, "Image data is missing"),
            MissingSeriesSlug => write!(f, "Series slug is missing"),
            InvalidUrl(e) => write!(f, "Invalid URL: {}", e),
            InvalidJWT(e) => write!(f, "Could not decode authentication JWT: {}", e),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use Error::*;

        match self {
            HTTP(e) => Some(e),
            IO(e) => Some(e),
            InvalidHTTPHeader(e) => Some(e),
            InvalidDateFormat(e) => Some(e),
            InvalidUrl(e) => Some(e),
            InvalidJWT(e) => Some(e),
            InvalidAPIKey
            | ServerError
            | NotFound
            | MissingLastModified
            | MissingSeriesFilterKeys
            | MissingImage
            | MissingSeriesSlug => None,
        }
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
mod tests {
    use super::*;

    #[test]
    fn error_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<Error>();
    }
}
