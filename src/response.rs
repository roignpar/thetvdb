use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc, Weekday};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct ResponseData<T> {
    pub data: T,
}

pub type SeriesID = u32;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSeries {
    pub aliases: Vec<String>,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub banner: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_date")]
    pub first_aired: Option<Date<Utc>>,
    pub id: SeriesID,
    pub network: String,
    pub overview: Option<String>,
    pub series_name: String,
    pub slug: String,
    pub status: SeriesStatus,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    #[serde(deserialize_with = "deserialize_optional_date_time")]
    pub added: Option<DateTime<Utc>>,
    // although not in the official docs,
    // `added_by` is returned by the API
    pub added_by: Option<u32>,
    #[serde(deserialize_with = "deserialize_optional_weekday")]
    pub airs_day_of_week: Option<Weekday>,
    #[serde(deserialize_with = "deserialize_optional_naive_time")]
    pub airs_time: Option<NaiveTime>,
    pub aliases: Vec<String>,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub banner: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_date")]
    pub first_aired: Option<Date<Utc>>,
    pub genre: Vec<String>,
    pub id: SeriesID,
    pub imdb_id: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_updated: DateTime<Utc>,
    pub network: String,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub network_id: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub overview: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub rating: Option<String>,
    pub runtime: String,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub series_id: Option<String>,
    pub series_name: String,
    #[serde(deserialize_with = "deserialize_optional_float")]
    pub site_rating: Option<f32>,
    pub site_rating_count: u32,
    pub slug: String,
    pub status: SeriesStatus,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub zap2it_id: Option<String>,
}

impl From<&SearchSeries> for SeriesID {
    fn from(s: &SearchSeries) -> SeriesID {
        s.id
    }
}

impl From<&Series> for SeriesID {
    fn from(s: &Series) -> SeriesID {
        s.id
    }
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

fn deserialize_optional_float<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let f = f32::deserialize(deserializer)?;
    if f == 0.0 {
        Ok(None)
    } else {
        Ok(Some(f))
    }
}

fn deserialize_optional_weekday<'de, D>(deserializer: D) -> Result<Option<Weekday>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        // because chrono::ParseWeekdayError doesn't impl std::fmt::Display
        // we cannot simply wrap it with serde::de::Error::custom
        let weekday = s
            .parse()
            .map_err(|e| serde::de::Error::custom(format!("failed to parse weekday: {:?}", e)))?;

        Ok(Some(weekday))
    }
}

fn deserialize_optional_naive_time<'de, D>(deserializer: D) -> Result<Option<NaiveTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        let t = NaiveTime::parse_from_str(&s, "%l:%M %p").map_err(serde::de::Error::custom)?;

        Ok(Some(t))
    }
}

fn deserialize_optional_date_time<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        let ndt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)?;

        Ok(Some(Utc.from_utc_datetime(&ndt)))
    }
}
