use serde::Deserialize;

pub type LanguageID = u16;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    id: LanguageID,
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
