use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::params::{EpisodeParams, EpisodeQuery, EpisodeQueryParams};

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
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub airs_day_of_week: Option<String>,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    pub id: u32,
    pub series_id: SeriesID,
    pub name: String,
    pub role: String,
    pub sort_order: u32,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub image: Option<String>,
    pub image_author: Option<u32>,
    #[serde(deserialize_with = "deserialize_optional_date_time")]
    pub image_added: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_optional_date_time")]
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: u32,
    pub aired_season: u16,
    #[serde(rename = "airedSeasonID")]
    pub aired_season_id: u32,
    pub aired_episode_number: u16,
    pub episode_name: String,
    #[serde(deserialize_with = "deserialize_optional_date")]
    pub first_aired: Option<Date<Utc>>,
    pub guest_stars: Vec<String>,
    pub directors: Vec<String>,
    pub writers: Vec<String>,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub overview: Option<String>,
    pub language: EpisodeLanguage,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub production_code: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub show_url: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_updated: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub dvd_discid: Option<String>,
    pub dvd_season: Option<u16>,
    pub dvd_episode_number: Option<u16>,
    pub dvd_chapter: Option<u16>,
    pub absolute_number: Option<u16>,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub filename: Option<String>,
    pub series_id: SeriesID,
    pub last_updated_by: Option<u32>,
    pub airs_after_season: Option<u16>,
    pub airs_before_season: Option<u16>,
    pub airs_before_episode: Option<u16>,
    pub thumb_author: Option<u32>,
    #[serde(deserialize_with = "deserialize_optional_date_time")]
    pub thumb_added: Option<DateTime<Utc>>,
    pub thumb_width: Option<String>,
    pub thumb_height: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub imdb_id: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_float")]
    pub site_rating: Option<f32>,
    pub site_rating_count: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeLanguage {
    pub episode_name: String,
    pub overview: String,
}

#[derive(Debug, Deserialize)]
pub struct EpisodePage {
    #[serde(rename = "data")]
    pub episodes: Vec<Episode>,
    #[serde(skip)]
    pub(crate) series_id: SeriesID,
    links: PageLinks,
}

impl EpisodePage {
    pub fn next_page_params(&self) -> Option<EpisodeParams> {
        self.next_page()
            .map(|n| EpisodeParams::with_page(self.series_id, n))
    }

    pub fn prev_page_params(&self) -> Option<EpisodeParams> {
        self.prev_page()
            .map(|p| EpisodeParams::with_page(self.series_id, p))
    }
}

#[derive(Debug, Deserialize)]
pub struct EpisodeQueryPage {
    #[serde(rename = "data")]
    pub episodes: Vec<Episode>,
    #[serde(skip)]
    pub(crate) series_id: SeriesID,
    #[serde(skip)]
    pub(crate) query: EpisodeQuery,
    links: PageLinks,
}

impl EpisodeQueryPage {
    pub fn next_page_query_params(&self) -> Option<EpisodeQueryParams> {
        self.next_page()
            .map(|n| EpisodeQueryParams::with_page(self.series_id, n))
    }

    pub fn prev_page_query_params(&self) -> Option<EpisodeQueryParams> {
        self.prev_page()
            .map(|p| EpisodeQueryParams::with_page(self.series_id, p))
    }
}

#[derive(Debug, Deserialize)]
pub struct PageLinks {
    first: u16,
    last: u16,
    next: Option<u16>,
    prev: Option<u16>,
}

impl PageLinks {
    fn current_page(&self) -> u16 {
        match (self.next, self.prev) {
            (Some(n), _) => n - 1,
            (None, Some(p)) => p + 1,
            _ => self.first,
        }
    }
}

pub trait Pagination {
    fn links(&self) -> &PageLinks;

    fn current_page(&self) -> u16 {
        self.links().current_page()
    }

    fn first_page(&self) -> u16 {
        self.links().first
    }

    fn last_page(&self) -> u16 {
        self.links().last
    }

    fn next_page(&self) -> Option<u16> {
        self.links().next
    }

    fn prev_page(&self) -> Option<u16> {
        self.links().prev
    }
}

impl Pagination for EpisodePage {
    fn links(&self) -> &PageLinks {
        &self.links
    }
}

impl Pagination for EpisodeQueryPage {
    fn links(&self) -> &PageLinks {
        &self.links
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeSummary {
    pub aired_seasons: Vec<String>,
    #[serde(deserialize_with = "deserialize_u16_string")]
    pub aired_episodes: u16,
    pub dvd_seasons: Vec<String>,
    #[serde(deserialize_with = "deserialize_u16_string")]
    pub dvd_episodes: u16,
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

fn deserialize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Value::deserialize(deserializer)?;
    match v {
        Value::String(s) if !s.is_empty() => Ok(Some(s)),
        _ => Ok(None),
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

fn deserialize_optional_date<'de, D>(deserializer: D) -> Result<Option<Date<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        let nd = NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)?;

        Ok(Some(Utc.from_utc_date(&nd)))
    }
}

fn deserialize_optional_date_time<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() || is_zero_date_time_str(&s) {
        Ok(None)
    } else {
        let ndt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)?;

        Ok(Some(Utc.from_utc_datetime(&ndt)))
    }
}

fn deserialize_u16_string<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse()
        .map_err(serde::de::Error::custom)
}

fn is_zero_date_time_str(s: &str) -> bool {
    s == "0000-00-00 00:00:00"
}
