#![deny(missing_docs)]

//! This crate provides an async [client](./client/struct.Client.html)
//! as well as helpful types to request and interact with data from
//! **TheTVDB API V3**.
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
//!
//! For more examples check [Client](./client/struct.Client.html).

mod deserialize;
mod serialize;
mod urls;

pub mod client;
pub mod error;
pub mod language;
pub mod params;
pub mod response;

pub use client::Client;
