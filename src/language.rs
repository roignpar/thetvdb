use std::fmt;

use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub id: LanguageID,
    abbreviation: String,
    pub name: String,
    pub english_name: String,
}

impl Language {
    pub fn abbr(&self) -> &str {
        &self.abbreviation
    }
}

impl From<&Language> for LanguageID {
    fn from(language: &Language) -> LanguageID {
        language.id
    }
}
