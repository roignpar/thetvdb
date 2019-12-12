#![deny(missing_docs, missing_debug_implementations, unsafe_code)]

//! Language related types and impls.

use std::fmt;

use serde::Deserialize;

/// Custom type used for [`Language`](./struct.Language.html) ids.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, PartialOrd, Ord, Eq, Deserialize)]
pub struct LanguageID(pub u16);

impl fmt::Display for LanguageID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u16> for LanguageID {
    fn from(i: u16) -> Self {
        Self(i)
    }
}

/// Language data returned by the API.
///
/// Can be used to [set the client language](../client/struct.Client.html#method.set_language).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    /// ID of the language.
    pub id: LanguageID,
    pub(crate) abbreviation: String,
    /// Original name of the language.
    pub name: String,
    /// English name of the language.
    pub english_name: String,
}

impl Language {
    /// Get the language's abbreviation.
    ///
    /// # Examples
    /// ```no_run
    /// # use thetvdb::{Client, error::Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #
    /// # let client = Client::new("KEY").await?;
    /// #
    /// let japanese = client.language(25).await?;
    ///
    /// assert_eq!(japanese.abbr(), "ja".to_string());
    /// # Ok(()) }
    /// ```
    pub fn abbr(&self) -> &str {
        &self.abbreviation
    }
}

impl From<&Language> for LanguageID {
    fn from(language: &Language) -> LanguageID {
        language.id
    }
}
