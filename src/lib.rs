#![doc(html_root_url = "https://docs.rs/thetvdb/0.1.0-beta.2")]
#![deny(missing_docs, missing_debug_implementations, unsafe_code)]

//! This crate provides an async [client] as well as helpful types to request
//! and interact with data from **TheTVDB API v3**.
//!
//! You will need a valid API key to create a new client.
//! To generate a key log in and go to the [API Keys page].
//!
//! # Examples
//! Search for a series by name:
//! ```no_run
//! # use thetvdb::error::Result;
//! use thetvdb::{Client, params::SearchBy};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let client = Client::new("YOUR_API_KEY").await?;
//!
//! let results = client.search(SearchBy::Name("Planet Earth")).await?;
//!
//! println!("{:#?}", results);
//! # Ok(()) }
//! ```
//! Get a series by ID:
//! ```no_run
//! # use thetvdb::error::Result;
//! # use thetvdb::Client;
//! #
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! # let client = Client::new("KEY").await?;
//! let series = client.series(318408).await?;
//!
//! assert_eq!(
//!     series.series_name,
//!     Some("Planet Earth II".to_string())
//! );
//! # Ok(()) }
//! ```
//! Use a custom struct to deserialize the API response:
//! ```no_run
//! # use thetvdb::error::Result;
//! # use thetvdb::Client;
//! use serde::Deserialize;
//! #
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! # let client = Client::new("KEY").await?;
//!
//! #[derive(Deserialize)]
//! struct MySeries {
//!     #[serde(rename = "seriesName")]
//!     name: String,
//!     overview: Option<String>,
//!     season: String,
//!     banner: Option<String>,
//!     genre: Vec<String>,
//! }
//!
//! let series: MySeries = client.series_into(318408).await?;
//! # Ok(()) }
//! ```
//!
//! For more examples check [`Client`][client].
//!
//! [client]: ./client/struct.Client.html
//! [API Keys page]: https://thetvdb.com/dashboard/account/apikeys
mod serialization;
mod urls;

pub mod client;
pub mod error;
pub mod language;
pub mod params;
pub mod response;

#[doc(inline)]
pub use client::Client;
#[doc(inline)]
pub use error::Error;

#[cfg(test)]
mod test_util {
    use chrono::{DateTime, TimeZone, Utc};

    use crate::error::Error;

    pub fn now_round_seconds() -> DateTime<Utc> {
        Utc.timestamp(Utc::now().timestamp(), 0)
    }

    pub fn wrong_error_kind(expected: Error, got: Error) {
        panic!("Wrong error kind: expected {:?}, got {:?}", expected, got);
    }
}
