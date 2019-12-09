#![deny(missing_docs, missing_debug_implementations, unsafe_code)]

//! Parameters used by `Client` to send API requests.

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::response::SeriesID;
use crate::serialize;

/// Parameter used to search for series with
/// [`Client.search`](../client/struct.Client.html#method.search).
#[derive(Debug)]
pub enum SearchBy<S> {
    /// Search by (partial) name.
    Name(S),
    /// Search by IMDb ID.
    IMDbID(S),
    /// Search by Zap2it ID.
    Zap2itID(S),
    /// Search by slug.
    Slug(S),
}

impl<S> SearchBy<S>
where
    S: AsRef<str>,
{
    pub(crate) fn query_param(&self) -> [(&str, &str); 1] {
        use SearchBy::*;

        match self {
            Name(name) => [("name", name.as_ref())],
            IMDbID(id) => [("imdbId", id.as_ref())],
            Zap2itID(id) => [("zap2itId", id.as_ref())],
            Slug(slug) => [("slug", slug.as_ref())],
        }
    }
}

/// Parameters used to get a series' episodes with
/// [`Client.series_episodes`](../client/struct.Client.html#method.series_episodes).
#[derive(Debug)]
pub struct EpisodeParams {
    pub(crate) series_id: SeriesID,
    pub(crate) page: u16,
}

impl EpisodeParams {
    /// Create new parameters for the given series.
    pub fn new<I>(series_id: I) -> Self
    where
        I: Into<SeriesID>,
    {
        let series_id = series_id.into();

        Self { series_id, page: 1 }
    }

    /// Create new parameters for the given series with page.
    pub fn with_page<I>(series_id: I, page: u16) -> Self
    where
        I: Into<SeriesID>,
    {
        let series_id = series_id.into();

        Self { series_id, page }
    }

    /// Set the `page` parameter.
    pub fn page(mut self, page: u16) -> Self {
        self.page = page;

        self
    }
}

/// Trait used to create episode parameters.
///
/// Implemented for all types that `impl Into<SeriesID>`.
///
/// # Examples
/// ```no_run
/// # use thetvdb::{Client, error::Result};
/// #
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// # let client = Client::new("KEY").await?;
/// #
/// use thetvdb::params::GetEpisodeParams;
///
/// let series = client.series(318408).await?;
///
/// let params = series.episode_params();
///
/// let episodes_page = client.series_episodes(params).await?;
/// # Ok(()) }
/// ```
pub trait GetEpisodeParams<'a> {
    /// Get the series to create the parameters for.
    fn series_id(&'a self) -> SeriesID;

    /// Create episode params for the series returned by `series_id`.
    fn episode_params(&'a self) -> EpisodeParams {
        EpisodeParams::new(self.series_id())
    }

    /// Create episode params with page for the series returned by `series_id`.
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

/// Parameters used to query for a series episodes with
/// [`Client.series_episodes_query`](../client/struct.Client.html#method.series_episodes_query).
#[derive(Debug)]
pub struct EpisodeQueryParams {
    pub(crate) params: EpisodeParams,
    pub(crate) query: EpisodeQuery,
}

impl EpisodeQueryParams {
    /// Create new parameters for the given series.
    pub fn new<I>(series_id: I) -> Self
    where
        I: Into<SeriesID>,
    {
        Self {
            params: EpisodeParams::new(series_id),
            query: Default::default(),
        }
    }

    /// Create new parameters for given series with page.
    pub fn with_page<I>(series_id: I, page: u16) -> Self
    where
        I: Into<SeriesID>,
    {
        Self {
            params: EpisodeParams::with_page(series_id, page),
            query: Default::default(),
        }
    }

    /// Set the `page` parameter.
    pub fn page(mut self, page: u16) -> Self {
        self.params.page = page;
        self
    }

    /// Set the `absoluteNumber` parameter.
    pub fn absolute_number(mut self, number: u16) -> Self {
        self.query.absolute_number = Some(number);
        self
    }

    /// Set the `airedSeason` parameter.
    pub fn aired_season(mut self, season: u16) -> Self {
        self.query.aired_season = Some(season);
        self
    }

    /// Set the `airedEpisode` parameter.
    pub fn aired_episode(mut self, episode: u16) -> Self {
        self.query.aired_episode = Some(episode);
        self
    }

    /// Set the `dvdSeason` parameter.
    pub fn dvd_season(mut self, season: u16) -> Self {
        self.query.dvd_season = Some(season);
        self
    }

    /// Set the `dvdEpisode` parameter.
    pub fn dvd_episode(mut self, episode: u16) -> Self {
        self.query.dvd_episode = Some(episode);
        self
    }

    /// Set the `imdbId` parameter.
    pub fn imdb_id<S>(mut self, id: S) -> Self
    where
        S: Into<String>,
    {
        self.query.imdb_id = Some(id.into());
        self
    }
}

/// Trait used to create episode query parameters.
///
/// Implemented for all types that `impl Into<SeriesID>`.
///
/// Similar to [`GetEpisodeParams`](./trait.GetEpisodeParams.html). Check for examples.
pub trait GetEpisodeQueryParams<'a> {
    /// Get the series to create the parameters for.
    fn series_id(&'a self) -> SeriesID;

    /// Create episode query params for the series returned by `series_id`.
    fn episode_query_params(&'a self) -> EpisodeQueryParams {
        EpisodeQueryParams::new(self.series_id())
    }

    /// Create episode query params with page for the series returned by `series_id`.
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

/// Parameters used to filter series fields with
/// [`Client.series_filter`](../client/struct.Client.html#method.series_filter).
///
/// The words "key" and "field" are used interchangeably in this context.
#[derive(Debug)]
pub struct SeriesFilterKeys {
    pub(crate) keys_query: String,
}

impl SeriesFilterKeys {
    /// Create a new list of filter keys.
    pub fn new() -> Self {
        Self {
            // if all keys are added, this many bytes would be used
            keys_query: String::with_capacity(226),
        }
    }

    /// Add `network_id` to key list.
    pub fn network_id(self) -> Self {
        self.push_key("networkId")
    }

    // NOTE: V3.0.0 of the API doesn't return the
    // `lastUpdated` field on series filter requests;
    //
    // TODO: enable when API is fixed
    // https://forums.thetvdb.com/viewtopic.php?f=17&t=22325&p=162247#p162247
    //
    ///// Add `last_updated` to key list.
    //pub fn last_updated(self) -> Self {
    //self.push_key("lastUpdated")
    //}

    /// Add `airs_time` to key list.
    pub fn airs_time(self) -> Self {
        self.push_key("airsTime")
    }

    /// Add `site_rating` to key list.
    pub fn site_rating(self) -> Self {
        self.push_key("siteRating")
    }

    /// Add `series_name` to key list.
    pub fn series_name(self) -> Self {
        self.push_key("seriesName")
    }

    /// Add `first_aired` to key list.
    pub fn first_aired(self) -> Self {
        self.push_key("firstAired")
    }

    /// Add `runtime` to key list.
    pub fn runtime(self) -> Self {
        self.push_key("runtime")
    }

    /// Add `overview` to key list.
    pub fn overview(self) -> Self {
        self.push_key("overview")
    }

    /// Add `banner` to key list.
    pub fn banner(self) -> Self {
        self.push_key("banner")
    }

    /// Add `genre` to key list.
    pub fn genre(self) -> Self {
        self.push_key("genre")
    }

    /// Add `airs_day_of_week` to key list.
    pub fn airs_day_of_week(self) -> Self {
        self.push_key("airsDayOfWeek")
    }

    /// Add `imdb_id` to key list.
    pub fn imdb_id(self) -> Self {
        self.push_key("imdbId")
    }

    /// Add `added_by` to key list.
    pub fn added_by(self) -> Self {
        self.push_key("addedBy")
    }

    /// Add `site_rating_count` to key list.
    pub fn site_rating_count(self) -> Self {
        self.push_key("siteRatingCount")
    }

    /// Add `id` to key list.
    pub fn id(self) -> Self {
        self.push_key("id")
    }

    /// Add `status` to key list.
    pub fn status(self) -> Self {
        self.push_key("status")
    }

    /// Add `network` to key list.
    pub fn network(self) -> Self {
        self.push_key("network")
    }

    /// Add `rating` to key list.
    pub fn rating(self) -> Self {
        self.push_key("rating")
    }

    /// Add `zap2it_id` to key list.
    pub fn zap2it_id(self) -> Self {
        self.push_key("zap2itId")
    }

    /// Add `added` to key list.
    pub fn added(self) -> Self {
        self.push_key("added")
    }

    /// Add `slug` to key list.
    pub fn slug(self) -> Self {
        self.push_key("slug")
    }

    /// Add `aliases` to key list.
    pub fn aliases(self) -> Self {
        self.push_key("aliases")
    }

    /// Add `season` to key list.
    pub fn season(self) -> Self {
        self.push_key("season")
    }

    /// Add `poster` to key list.
    pub fn poster(self) -> Self {
        self.push_key("poster")
    }

    /// Add `fanart` to key list.
    pub fn fanart(self) -> Self {
        self.push_key("fanart")
    }

    /// Add `language` to key list.
    pub fn language(self) -> Self {
        self.push_key("language")
    }

    /// Returns `true` if no keys have been added to the list.
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

/// Parameters used to get series images with
/// [`Client.series_images_query`](../client/struct.Client.html#method.series_images_query).
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
    /// Create new parameters with the given key type.
    pub fn with_key_type<S>(key_type: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            key_type: Some(key_type.into()),
            ..Default::default()
        }
    }

    /// Create new parameters with the given resolution.
    pub fn with_resolution<S>(resolution: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            resolution: Some(resolution.into()),
            ..Default::default()
        }
    }

    /// Create new parameters with the given subkey.
    pub fn with_sub_key<S>(sub_key: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            sub_key: Some(sub_key.into()),
            ..Default::default()
        }
    }

    /// Set the key type.
    pub fn key_type<S>(mut self, key_type: S) -> Self
    where
        S: Into<String>,
    {
        self.key_type = Some(key_type.into());

        self
    }

    /// Set the resolution.
    pub fn resolution<S>(mut self, resolution: S) -> Self
    where
        S: Into<String>,
    {
        self.resolution = Some(resolution.into());

        self
    }

    /// Set the subkey.
    pub fn sub_key<S>(mut self, sub_key: S) -> Self
    where
        S: Into<String>,
    {
        self.sub_key = Some(sub_key.into());

        self
    }
}

/// Parameters used to get updated series with
/// [`Client.updated`](../client/struct.Client.html#method.updated).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedParams {
    #[serde(serialize_with = "chrono::serde::ts_seconds::serialize")]
    from_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_time: Option<serialize::Timestamp>,
}

impl UpdatedParams {
    /// Create new parameters with the given `from` time.
    pub fn new<D>(from: D) -> Self
    where
        D: Into<DateTime<Utc>>,
    {
        Self {
            from_time: from.into(),
            to_time: None,
        }
    }

    /// Create new parameters with the given `from` and `to` times.
    pub fn with_to_time<D>(from: D, to: D) -> Self
    where
        D: Into<DateTime<Utc>>,
    {
        Self {
            from_time: from.into(),
            to_time: Some(serialize::Timestamp(to.into())),
        }
    }

    /// Set `to_time` parameter.
    pub fn set_to_time<D>(&mut self, to: D)
    where
        D: Into<DateTime<Utc>>,
    {
        self.to_time = Some(serialize::Timestamp(to.into()));
    }
}
