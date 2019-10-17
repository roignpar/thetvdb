use chrono::{Date, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct ResponseData<T> {
    pub data: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSeries {
    pub aliases: Vec<String>,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub banner: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_date")]
    pub first_aired: Option<Date<Utc>>,
    pub id: u32,
    pub network: String,
    pub overview: Option<String>,
    pub series_name: String,
    pub slug: String,
    pub status: SeriesStatus,
}

#[derive(Debug, Deserialize)]
pub enum SeriesStatus {
    Ended,
    Continuing,
    Upcoming,
    #[serde(rename = "")]
    Unknown,
}

fn deserialize_optional_date<'de, D>(deserializer: D) -> Result<Option<Date<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(None);
    }

    let ndate = NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)?;

    Ok(Some(Utc.from_utc_date(&ndate)))
}

fn deserialize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}
