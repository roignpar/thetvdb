use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::response::SeriesID;
use crate::serialize;

#[derive(Debug)]
pub enum SearchBy<S> {
    Name(S),
    ImdbID(S),
    Zap2itID(S),
    Slug(S),
}

// SearchBy needs to be transformed into a query param and deriving
// Serialize doesn't help:
// "top-level serializer supports only maps and structs";
//
// implementing Into instead of From because clippy incorrectly
// complains: https://github.com/rust-lang/rust-clippy/issues/3899
impl<S, B> Into<HashMap<String, String, B>> for SearchBy<S>
where
    S: Into<String>,
    B: std::hash::BuildHasher + Default,
{
    fn into(self) -> HashMap<String, String, B> {
        use SearchBy::*;

        let mut map = HashMap::default();

        match self {
            Name(name) => map.insert("name".to_string(), name.into()),
            ImdbID(id) => map.insert("imdbId".to_string(), id.into()),
            Zap2itID(id) => map.insert("zap2itId".to_string(), id.into()),
            Slug(slug) => map.insert("slug".to_string(), slug.into()),
        };

        map
    }
}

#[derive(Debug)]
pub struct EpisodeParams {
    pub(crate) series_id: SeriesID,
    pub(crate) page: u16,
}

impl EpisodeParams {
    pub fn new<I>(series_id: I) -> Self
    where
        I: Into<SeriesID>,
    {
        let series_id = series_id.into();

        Self { series_id, page: 1 }
    }

    pub fn with_page<I>(series_id: I, page: u16) -> Self
    where
        I: Into<SeriesID>,
    {
        let series_id = series_id.into();

        Self { series_id, page }
    }

    pub fn set_page(&mut self, page: u16) {
        self.page = page;
    }
}

pub trait GetEpisodeParams<'a> {
    fn series_id(&'a self) -> SeriesID;

    fn episode_params(&'a self) -> EpisodeParams {
        EpisodeParams::new(self.series_id())
    }

    fn episode_params_page(&'a self, page: u16) -> EpisodeParams {
        EpisodeParams::with_page(self.series_id(), page)
    }
}

impl<'a, T> GetEpisodeParams<'a> for T
where
    T: 'a,
    SeriesID: From<&'a T>,
{
    fn series_id(&'a self) -> SeriesID {
        SeriesID::from(self)
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EpisodeQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    absolute_number: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aired_season: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aired_episode: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dvd_season: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dvd_episode: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    imdb_id: Option<String>,
}

#[derive(Debug)]
pub struct EpisodeQueryParams {
    pub(crate) params: EpisodeParams,
    pub(crate) query: EpisodeQuery,
}

impl EpisodeQueryParams {
    pub fn new<I>(series_id: I) -> Self
    where
        I: Into<SeriesID>,
    {
        Self {
            params: EpisodeParams::new(series_id),
            query: Default::default(),
        }
    }

    pub fn with_page<I>(series_id: I, page: u16) -> Self
    where
        I: Into<SeriesID>,
    {
        Self {
            params: EpisodeParams::with_page(series_id, page),
            query: Default::default(),
        }
    }

    pub fn absolute_number(mut self, number: u16) -> Self {
        self.query.absolute_number = Some(number);
        self
    }

    pub fn aired_season(mut self, season: u16) -> Self {
        self.query.aired_season = Some(season);
        self
    }

    pub fn aired_episode(mut self, episode: u16) -> Self {
        self.query.aired_episode = Some(episode);
        self
    }

    pub fn dvd_season(mut self, season: u16) -> Self {
        self.query.dvd_season = Some(season);
        self
    }

    pub fn dvd_episode(mut self, episode: u16) -> Self {
        self.query.dvd_episode = Some(episode);
        self
    }

    pub fn imdb_id<S>(mut self, id: S) -> Self
    where
        S: Into<String>,
    {
        self.query.imdb_id = Some(id.into());
        self
    }
}

pub trait GetEpisodeQueryParams<'a> {
    fn series_id(&'a self) -> SeriesID;

    fn episode_query_params(&'a self) -> EpisodeQueryParams {
        EpisodeQueryParams::new(self.series_id())
    }

    fn episode_query_params_page(&'a self, page: u16) -> EpisodeQueryParams {
        EpisodeQueryParams::with_page(self.series_id(), page)
    }
}

impl<'a, T> GetEpisodeQueryParams<'a> for T
where
    T: 'a,
    SeriesID: From<&'a T>,
{
    fn series_id(&'a self) -> SeriesID {
        self.into()
    }
}

pub struct SeriesFilterKeys {
    pub(crate) keys_query: String,
}

impl SeriesFilterKeys {
    pub fn new() -> Self {
        Self {
            // if all keys are added, 200 bytes would be used
            keys_query: String::with_capacity(200),
        }
    }

    pub fn network_id(self) -> Self {
        self.push_key("networkId")
    }

    // NOTE: V2.2.0 of the API doesn't return the
    // `lastUpdated` field on series filter requests;
    //
    // TODO: enable when API is fixed
    // https://forums.thetvdb.com/viewtopic.php?f=17&t=22325&p=162247#p162247
    //
    //pub fn last_updated(self) -> Self {
    //self.push_key("lastUpdated")
    //}

    pub fn airs_time(self) -> Self {
        self.push_key("airsTime")
    }

    pub fn site_rating(self) -> Self {
        self.push_key("siteRating")
    }

    pub fn series_name(self) -> Self {
        self.push_key("seriesName")
    }

    pub fn first_aired(self) -> Self {
        self.push_key("firstAired")
    }

    pub fn runtime(self) -> Self {
        self.push_key("runtime")
    }

    pub fn overview(self) -> Self {
        self.push_key("overview")
    }

    pub fn banner(self) -> Self {
        self.push_key("banner")
    }

    pub fn series_id(self) -> Self {
        self.push_key("seriesId")
    }

    pub fn genre(self) -> Self {
        self.push_key("genre")
    }

    pub fn airs_day_of_week(self) -> Self {
        self.push_key("airsDayOfWeek")
    }

    pub fn imdb_id(self) -> Self {
        self.push_key("imdbId")
    }

    pub fn added_by(self) -> Self {
        self.push_key("addedBy")
    }

    pub fn site_rating_count(self) -> Self {
        self.push_key("siteRatingCount")
    }

    pub fn id(self) -> Self {
        self.push_key("id")
    }

    pub fn status(self) -> Self {
        self.push_key("status")
    }

    pub fn network(self) -> Self {
        self.push_key("network")
    }

    pub fn rating(self) -> Self {
        self.push_key("rating")
    }

    pub fn zap2it_id(self) -> Self {
        self.push_key("zap2itId")
    }

    pub fn added(self) -> Self {
        self.push_key("added")
    }

    pub fn slug(self) -> Self {
        self.push_key("slug")
    }

    pub fn aliases(self) -> Self {
        self.push_key("aliases")
    }

    pub fn is_empty(&self) -> bool {
        self.keys_query.is_empty()
    }

    fn push_key(mut self, key: &str) -> Self {
        if !self.keys_query.is_empty() {
            self.keys_query.push(',');
        }

        self.keys_query.push_str(key);

        self
    }
}

impl Default for SeriesFilterKeys {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    key_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_key: Option<String>,
}

impl ImageQueryParams {
    pub fn with_key_type<S>(key_type: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            key_type: Some(key_type.into()),
            ..Default::default()
        }
    }

    pub fn with_resolution<S>(resolution: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            resolution: Some(resolution.into()),
            ..Default::default()
        }
    }

    pub fn with_sub_key<S>(sub_key: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            sub_key: Some(sub_key.into()),
            ..Default::default()
        }
    }

    pub fn key_type<S>(mut self, key_type: S) -> Self
    where
        S: Into<String>,
    {
        self.key_type = Some(key_type.into());

        self
    }

    pub fn resolution<S>(mut self, resolution: S) -> Self
    where
        S: Into<String>,
    {
        self.resolution = Some(resolution.into());

        self
    }

    pub fn sub_key<S>(mut self, sub_key: S) -> Self
    where
        S: Into<String>,
    {
        self.sub_key = Some(sub_key.into());

        self
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedParams {
    #[serde(serialize_with = "chrono::serde::ts_seconds::serialize")]
    from_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_time: Option<serialize::Timestamp>,
}

impl UpdatedParams {
    pub fn new<D>(from: D) -> Self
    where
        D: Into<DateTime<Utc>>,
    {
        Self {
            from_time: from.into(),
            to_time: None,
        }
    }

    pub fn with_to_time<D>(from: D, to: D) -> Self
    where
        D: Into<DateTime<Utc>>,
    {
        Self {
            from_time: from.into(),
            to_time: Some(serialize::Timestamp(to.into())),
        }
    }

    pub fn set_to_time<D>(&mut self, to: D)
    where
        D: Into<DateTime<Utc>>,
    {
        self.to_time = Some(serialize::Timestamp(to.into()));
    }
}
