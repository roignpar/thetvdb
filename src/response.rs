//! Types used to deserialize and work with data received from the API.

use std::fmt;

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::*;
use crate::params::{EpisodeParams, EpisodeQuery, EpisodeQueryParams};
use crate::serialization as ser;
use crate::urls::URLS;

mod movie;

pub use movie::*;

#[derive(Debug, Deserialize)]
pub(crate) struct ResponseData<T> {
    pub(crate) data: T,
}

/// Custom type used for [`Series`] ids.
///
/// [`Series`]: struct.Series.html
#[derive(
    Clone, Copy, Debug, Default, Hash, PartialEq, PartialOrd, Ord, Eq, Deserialize, Serialize,
)]
pub struct SeriesID(pub u32);

impl fmt::Display for SeriesID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for SeriesID {
    fn from(i: u32) -> Self {
        Self(i)
    }
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

impl From<&SeriesUpdate> for SeriesID {
    fn from(s: &SeriesUpdate) -> SeriesID {
        s.id
    }
}

/// Custom type used for [`Episode`] ids.
///
/// [`Episode`]: struct.Episode.html
#[derive(
    Clone, Copy, Debug, Default, Hash, PartialEq, PartialOrd, Ord, Eq, Deserialize, Serialize,
)]
pub struct EpisodeID(pub u32);

impl fmt::Display for EpisodeID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for EpisodeID {
    fn from(i: u32) -> Self {
        Self(i)
    }
}

impl From<&Episode> for EpisodeID {
    fn from(e: &Episode) -> EpisodeID {
        e.id
    }
}

/// Series data returned by [`Client::search`].
///
/// Contains less information than `Series`, but can be used
/// to get all the data.
///
/// See [`Client::search`] and [`Client::series`] for more info.
///
/// [`Client::search`]: ../client/struct.Client.html#method.search
/// [`Client::series`]: ../client/struct.Client.html#method.series
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(test, derive(Default))]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct SearchSeries {
    /// Series aliases.
    pub aliases: Vec<String>,
    /// Path to the series' banner.
    ///
    /// Use [`banner_url`](#method.banner_url) for a full URL.
    #[serde(deserialize_with = "ser::optional_string")]
    pub banner: Option<String>,
    /// Date when series was first aired.
    #[serde(with = "ser::optional_naive_date")]
    pub first_aired: Option<NaiveDate>,
    /// ID of the series.
    pub id: SeriesID,
    /// The series' network.
    #[serde(deserialize_with = "ser::optional_string")]
    pub network: Option<String>,
    /// Short description of the series.
    pub overview: Option<String>,
    /// Name of the series.
    pub series_name: Option<String>,
    /// Slug used to create the full [`website_url`](#method.website_url) for
    /// this series.
    pub slug: String,
    /// Status of the series.
    ///
    /// See [`SeriesStatus`](./enum.SeriesStatus.html) for more info.
    pub status: SeriesStatus,
}

macro_rules! series_banner_url_method {
    () => {
        /// Returns the full URL to the series' banner.
        ///
        /// # Errors
        /// Will fail if series `banner` is `None`.
        pub fn banner_url(&self) -> Result<Url> {
            URLS.opt_image(&self.banner)
        }
    }
}

macro_rules! series_website_url_method {
    () => {
        /// Returns the full `thetvdb.com` website series URL.
        ///
        /// # Errors
        /// Will fail if the series `slug` is somehow malformed
        /// and cannot be parsed into an `Url`.
        pub fn website_url(&self) -> Result<Url> {
            URLS.series_website(&self.slug)
        }
    }
}

impl SearchSeries {
    series_banner_url_method!();

    series_website_url_method!();
}

/// Full series data returned by [`Client::series`].
///
/// See linked method for more info.
///
/// [`Client::series`]: ../client/struct.Client.html#method.series
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(test, derive(Default))]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Series {
    /// ID of the series.
    pub id: SeriesID,
    /// Name of the series.
    #[serde(deserialize_with = "ser::optional_string")]
    pub series_name: Option<String>,
    /// The date and time when the series was added to TheTVDB.
    #[serde(with = "ser::optional_date_time")]
    pub added: Option<DateTime<Utc>>,
    // although not in the official docs,
    // `added_by` is returned by the API
    /// ID of the user that added the series to TheTVDB.
    pub added_by: Option<u32>,
    /// Day or days of week when series airs.
    #[serde(deserialize_with = "ser::optional_string")]
    pub airs_day_of_week: Option<String>,
    /// Time of day when the episodes air.
    #[serde(with = "ser::optional_naive_time")]
    pub airs_time: Option<NaiveTime>,
    /// Series aliases.
    pub aliases: Vec<String>,
    /// Series current season.
    pub season: String,
    /// Path to the series' banner.
    ///
    /// Use [`banner_url`](#method.banner_url) for a full URL.
    #[serde(deserialize_with = "ser::optional_string")]
    pub banner: Option<String>,
    /// Path to the series' poster.
    ///
    /// Use [`poster_url`](#method.poster_url) for a full URL.
    #[serde(deserialize_with = "ser::optional_string")]
    pub poster: Option<String>,
    /// Path to the series' fanart.
    ///
    /// Use [`fanart_url`](#method.fanart_url) for a full URL.
    #[serde(deserialize_with = "ser::optional_string")]
    pub fanart: Option<String>,
    /// Date when series was first aired.
    #[serde(with = "ser::optional_naive_date")]
    pub first_aired: Option<NaiveDate>,
    /// List of the series' genres.
    pub genre: Vec<String>,
    /// IMDb ID of the series.
    #[serde(deserialize_with = "ser::optional_string")]
    pub imdb_id: Option<String>,
    /// Time and date when series was last updated.
    #[serde(with = "ser::optional_ts_seconds_date_time")]
    pub last_updated: Option<DateTime<Utc>>,
    /// The series' network.
    pub network: Option<String>,
    /// The series' network ID.
    #[serde(deserialize_with = "ser::optional_string")]
    pub network_id: Option<String>,
    /// Short description of the series.
    #[serde(deserialize_with = "ser::optional_string")]
    pub overview: Option<String>,
    /// Series parental guide rating.
    #[serde(deserialize_with = "ser::optional_string")]
    pub rating: Option<String>,
    /// Series episode runtime.
    pub runtime: String,
    /// Series language abbreviation.
    pub language: String,
    /// Series rating.
    #[serde(deserialize_with = "ser::optional_float")]
    pub site_rating: Option<f32>,
    /// Number of rating votes.
    pub site_rating_count: u32,
    /// Series website slug.
    ///
    /// Use [`website_url`](#method.website_url) for the full URL.
    pub slug: String,
    /// Status of the series.
    ///
    /// See [`SeriesStatus`](./enum.SeriesStatus.html) for more info.
    pub status: SeriesStatus,
    /// Zap2it ID of the series.
    #[serde(deserialize_with = "ser::optional_string")]
    pub zap2it_id: Option<String>,
}

macro_rules! series_url_methods {
    () => {
        series_banner_url_method!();

        /// Returns the full URL to the series' poster.
        ///
        /// # Errors
        /// Will fail if series `poster` is `None`.
        pub fn poster_url(&self) -> Result<Url> {
            URLS.opt_image(&self.poster)
        }

        /// Returns the full URL to the series' fanart.
        ///
        /// # Errors
        /// Will fail if series `fanart` is `None`.
        pub fn fanart_url(&self) -> Result<Url> {
            URLS.opt_image(&self.fanart)
        }
    }
}

impl Series {
    series_url_methods!();

    series_website_url_method!();
}

/// Series data returned by [`Client::series_filter`].
///
/// Contains the same fields as [`Series`], but all values are optional.
///
/// Will only contain values of the selected fields that the API returned.
///
/// For more info see [`Client::series_filter`].
///
/// [`Client::series_filter`]: ../client/struct.Client.html#method.series_filter
/// [`Series`]: struct.Series.html
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(test, derive(Default))]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct FilteredSeries {
    /// ID of the series.
    #[serde(default)]
    pub id: Option<SeriesID>,
    /// Name of the series.
    #[serde(default, deserialize_with = "ser::optional_string")]
    pub series_name: Option<String>,
    /// The date and time when the series was added to TheTVDB.
    #[serde(default, with = "ser::optional_date_time")]
    pub added: Option<DateTime<Utc>>,
    /// ID of the user that added the series to TheTVDB.
    #[serde(default)]
    pub added_by: Option<u32>,
    /// Day or days of week when series airs.
    #[serde(default, deserialize_with = "ser::optional_string")]
    pub airs_day_of_week: Option<String>,
    /// Time of day when the episodes air.
    #[serde(default, with = "ser::optional_naive_time")]
    pub airs_time: Option<NaiveTime>,
    /// Series aliases.
    #[serde(default)]
    pub aliases: Option<Vec<String>>,
    /// Series current season.
    #[serde(default)]
    pub season: Option<String>,
    /// Path to the series' banner.
    ///
    /// Use [`banner_url`](#method.banner_url) for a full URL.
    #[serde(default, deserialize_with = "ser::optional_string")]
    pub banner: Option<String>,
    /// Path to the series' poster.
    ///
    /// Use [`poster_url`](#method.poster_url) for a full URL.
    #[serde(default, deserialize_with = "ser::optional_string")]
    pub poster: Option<String>,
    /// Path to the series' fanart.
    ///
    /// Use [`fanart_url`](#method.fanart_url) for a full URL.
    #[serde(default, deserialize_with = "ser::optional_string")]
    pub fanart: Option<String>,
    /// Date when series was first aired.
    #[serde(default, with = "ser::optional_naive_date")]
    pub first_aired: Option<NaiveDate>,
    /// List of the series' genres.
    #[serde(default)]
    pub genre: Option<Vec<String>>,
    /// IMDb ID of the series.
    #[serde(default)]
    pub imdb_id: Option<String>,
    /// Time and date when series was last updated.
    #[serde(default, with = "ser::optional_ts_seconds_date_time")]
    pub last_updated: Option<DateTime<Utc>>,
    /// The series' network.
    #[serde(default)]
    pub network: Option<String>,
    /// The series' network ID.
    #[serde(default, deserialize_with = "ser::optional_string")]
    pub network_id: Option<String>,
    /// Short description of the series.
    #[serde(default, deserialize_with = "ser::optional_string")]
    pub overview: Option<String>,
    /// Series parental guide rating.
    #[serde(default, deserialize_with = "ser::optional_string")]
    pub rating: Option<String>,
    /// Series episode runtime.
    #[serde(default)]
    pub runtime: Option<String>,
    /// Series language abbreviation.
    #[serde(default)]
    pub language: Option<String>,
    /// Series rating.
    #[serde(default, deserialize_with = "ser::optional_float")]
    pub site_rating: Option<f32>,
    /// Number of rating votes.
    #[serde(default)]
    pub site_rating_count: Option<u32>,
    /// Series website slug.
    ///
    /// Use [`website_url`](#method.website_url) for the full URL.
    #[serde(default)]
    pub slug: Option<String>,
    /// Status of the series.
    ///
    /// See [`SeriesStatus`](./enum.SeriesStatus.html) for more info.
    #[serde(default)]
    pub status: Option<SeriesStatus>,
    /// Zap2it ID of the series.
    #[serde(default, deserialize_with = "ser::optional_string")]
    pub zap2it_id: Option<String>,
}

impl FilteredSeries {
    series_url_methods!();

    /// Returns the full `thetvdb.com` website series URL.
    ///
    /// # Errors
    /// Will fail if the series `slug` is somehow malformed
    /// and cannot be parsed into an `Url`.
    pub fn website_url(&self) -> Result<Url> {
        match self.slug.as_ref() {
            Some(s) => URLS.series_website(&s),
            None => Err(Error::MissingSeriesSlug),
        }
    }
}

/// Possible series status.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[non_exhaustive]
pub enum SeriesStatus {
    /// Series has ended and no more episodes will be aired.
    Ended,
    /// Series is continuing and more episodes will air.
    Continuing,
    /// Series is upcoming and no episodes have aired so far.
    Upcoming,
    /// Status is unknown. Means that the API didn't return a status.
    #[serde(rename = "")]
    Unknown,
}

#[cfg(test)]
impl Default for SeriesStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Actor data returned by [`Client::series_actors`].
///
/// See linked method for more info.
///
/// [`Client::series_actors`]: ../client/struct.Client.html#method.series_actors
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(test, derive(Default))]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    /// ID of the actor.
    pub id: u32,
    /// ID of the series the actor played this role in.
    pub series_id: SeriesID,
    /// Actor's name.
    pub name: String,
    /// Role played by actor in this series.
    pub role: String,
    /// Sort order as returned by the API.
    pub sort_order: u32,
    /// Actor's image path for this series.
    ///
    /// Use [`image_url`](#method.image_url) for a full URL.
    #[serde(deserialize_with = "ser::optional_string")]
    pub image: Option<String>,
    /// Image author.
    pub image_author: Option<u32>,
    #[serde(with = "ser::optional_date_time")]
    /// Date and time when the image was added.
    pub image_added: Option<DateTime<Utc>>,
    /// Date and time when this actor/role was last updated.
    #[serde(with = "ser::optional_date_time")]
    pub last_updated: Option<DateTime<Utc>>,
}

impl Actor {
    /// Returns the full URL of an actor's image.
    ///
    /// # Errors
    /// Will fail if series `image` is `None`.
    pub fn image_url(&self) -> Result<Url> {
        URLS.opt_image(&self.image)
    }
}

/// Episode data returned by [`Client::series_episodes`],
/// [`Client::series_episodes_query`] and [`Client::episode`].
///
/// See linked methods for more info.
///
/// [`Client::series_episodes`]: ../client/struct.Client.html#method.series_episodes
/// [`Client::series_episodes_query`]: ../client/struct.Client.html#method.series_episodes_query
/// [`Client::episode`]: ../client/struct.Client.html#method.episode
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(test, derive(Default))]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    /// ID of the episode.
    pub id: EpisodeID,
    /// Season that episode is part of.
    pub aired_season: Option<u16>,
    /// Episode season ID.
    #[serde(rename = "airedSeasonID")]
    pub aired_season_id: Option<u32>,
    /// Episode number in season.
    pub aired_episode_number: u16,
    /// Name of the episode.
    #[serde(deserialize_with = "ser::optional_string")]
    pub episode_name: Option<String>,
    /// Date when episode was first aired.
    #[serde(with = "ser::optional_naive_date")]
    pub first_aired: Option<NaiveDate>,
    /// List of guest stars playing in this episode.
    pub guest_stars: Vec<String>,
    /// List of this episode's directors.
    pub directors: Vec<String>,
    /// List of this episode's writers.
    pub writers: Vec<String>,
    /// Short description of this episode.
    #[serde(deserialize_with = "ser::optional_string")]
    pub overview: Option<String>,
    /// Language info of episode data.
    ///
    /// See [`EpisodeLanguage`](./struct.EpisodeLanguage.html)
    /// for more info.
    pub language: EpisodeLanguage,
    /// Episode production code.
    #[serde(deserialize_with = "ser::optional_string")]
    pub production_code: Option<String>,
    /// Show URL.
    #[serde(deserialize_with = "ser::optional_string")]
    pub show_url: Option<String>,
    /// Date and time when episode was last updated.
    #[serde(with = "ser::optional_ts_seconds_date_time")]
    pub last_updated: Option<DateTime<Utc>>,
    /// Episode DVD ID.
    #[serde(deserialize_with = "ser::optional_string")]
    pub dvd_discid: Option<String>,
    /// DVD season.
    pub dvd_season: Option<u16>,
    /// Episode's number on DVD.
    pub dvd_episode_number: Option<u16>,
    /// DVD chapter.
    pub dvd_chapter: Option<u16>,
    /// Episode's absolute number.
    pub absolute_number: Option<u16>,
    /// Path to episode's image.
    ///
    /// For the full URL use [`filename_url`](#method.filename_url).
    #[serde(deserialize_with = "ser::optional_string")]
    pub filename: Option<String>,
    /// ID of series that episode is part of.
    pub series_id: SeriesID,
    /// User ID that last updated this episode.
    pub last_updated_by: Option<u32>,
    /// Season this episode airs after.
    pub airs_after_season: Option<u16>,
    /// Season thie episode airs before.
    pub airs_before_season: Option<u16>,
    /// Episode this episode airs before.
    pub airs_before_episode: Option<u16>,
    /// Author of episode image.
    pub thumb_author: Option<u32>,
    /// Date and time image was added.
    #[serde(with = "ser::optional_date_time")]
    pub thumb_added: Option<DateTime<Utc>>,
    /// Image width.
    pub thumb_width: Option<String>,
    /// Image height.
    pub thumb_height: Option<String>,
    /// Episode's IMDb ID.
    #[serde(deserialize_with = "ser::optional_string")]
    pub imdb_id: Option<String>,
    /// Episode parental guideline rating.
    pub content_rating: Option<String>,
    /// Episode rating.
    #[serde(deserialize_with = "ser::optional_float")]
    pub site_rating: Option<f32>,
    /// Number of rating votes.
    pub site_rating_count: u32,
    /// Is this episode a movie?
    #[serde(with = "ser::int_bool")]
    pub is_movie: bool,
}

impl Episode {
    /// Returns the full URL of the episode's image.
    ///
    /// # Errors
    /// Will fail if episode `filename` is `None`.
    pub fn filename_url(&self) -> Result<Url> {
        URLS.opt_image(&self.filename)
    }
}

/// Episode language info.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(test, derive(Default))]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct EpisodeLanguage {
    /// Abbreviation of the episode name language.
    pub episode_name: String,
    /// Abbreviation of the episode overview language.
    pub overview: String,
}

/// Struct used for episode pagination returned by [`Client::series_episodes`].
///
/// Can be used to generate params for querying the next or previous pages.
///
/// See [`Client::series_episodes`] for more info.
///
/// [`Client::series_episodes`]: ../client/struct.Client.html#method.series_episodes
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct EpisodePage<E = Episode> {
    /// The episodes on this page.
    #[serde(rename = "data")]
    pub episodes: Vec<E>,
    #[serde(skip)]
    pub(crate) series_id: SeriesID,
    links: PageLinks,
}

impl<E> EpisodePage<E> {
    /// Generate `EpisodeParams` to fetch the next page with
    /// [`Client::series_episodes`].
    ///
    /// Will return `None` if there is no next page.
    ///
    /// [`Client::series_episodes`]: ../client/struct.Client.html#method.series_episodes
    pub fn next_page_params(&self) -> Option<EpisodeParams> {
        self.next_page()
            .map(|n| EpisodeParams::with_page(self.series_id, n))
    }

    /// Generate `EpisodeParams` to fetch the previous page with
    /// [`Client::series_episodes`].
    ///
    /// Will return `None` if there is no previous page.
    ///
    /// [`Client::series_episodes`]: ../client/struct.Client.html#method.series_episodes
    pub fn prev_page_params(&self) -> Option<EpisodeParams> {
        self.prev_page()
            .map(|p| EpisodeParams::with_page(self.series_id, p))
    }

    /// Generate `EpisodeParams` to fetch the first page with
    /// [`Client::series_episodes`](../client/struct.Client.html#method.series_episodes)
    pub fn first_page_params(&self) -> EpisodeParams {
        EpisodeParams::with_page(self.series_id, self.first_page())
    }

    /// Generate `EpisodeParams` to fetch the last page with
    /// [`Client::series_episodes`](../client/struct.Client.html#method.series_episodes)
    pub fn last_page_params(&self) -> EpisodeParams {
        EpisodeParams::with_page(self.series_id, self.last_page())
    }
}

/// Struct used for queried episode pagination returned by
/// [`Client::series_episodes_query`].
///
/// Works the same as [`EpisodePage`].
///
/// See [`Client::series_episodes_query`] and [`Client::series_episodes`] for more
/// info.
///
/// [`Client::series_episodes_query`]: ../client/struct.Client.html#method.series_episodes_query
/// [`Client::series_episodes`]: ../client/struct.Client.html#method.series_episodes
/// [`EpisodePage`]: struct.EpisodePage.html
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct EpisodeQueryPage<E = Episode> {
    /// The episodes on this page.
    #[serde(rename = "data")]
    pub episodes: Vec<E>,
    #[serde(skip)]
    pub(crate) series_id: SeriesID,
    #[serde(skip)]
    pub(crate) query: EpisodeQuery,
    links: PageLinks,
}

impl<E> EpisodeQueryPage<E> {
    /// Generate `EpisodeQueryParams` to fetch the next page of query results
    /// with [`Client::series_episodes_query`].
    ///
    /// Will return `None` if there is no next page.
    ///
    /// [`Client::series_episodes_query`]: ../client/struct.Client.html#method.series_episodes_query
    pub fn next_page_query_params(&self) -> Option<EpisodeQueryParams> {
        self.next_page()
            .map(|n| EpisodeQueryParams::with_page_query(self.series_id, n, self.query.clone()))
    }

    /// Generate `EpisodeQueryParams` to fetch the previous page of query
    /// results with [`Client::series_episodes_query`].
    ///
    /// Will return `None` if there is no previous page.
    ///
    /// [`Client::series_episodes_query`]: ../client/struct.Client.html#method.series_episodes_query
    pub fn prev_page_query_params(&self) -> Option<EpisodeQueryParams> {
        self.prev_page()
            .map(|p| EpisodeQueryParams::with_page_query(self.series_id, p, self.query.clone()))
    }

    /// Generate `EpisodeQueryParams` to fetch the first page of query results
    /// with [`Client::series_episodes_query`].
    ///
    /// [`Client::series_episodes_query`]: ../client/struct.Client.html#method.series_episodes_query
    pub fn first_page_query_params(&self) -> EpisodeQueryParams {
        EpisodeQueryParams::with_page_query(self.series_id, self.first_page(), self.query.clone())
    }

    /// Generate `EpisodeQueryParams` to fetch the last page of query results
    /// with [`Client::series_episodes_query`].
    ///
    /// [`Client::series_episodes_query`]: ../client/struct.Client.html#method.series_episodes_query
    pub fn last_page_query_params(&self) -> EpisodeQueryParams {
        EpisodeQueryParams::with_page_query(self.series_id, self.last_page(), self.query.clone())
    }
}

/// Struct used for page links in paginated API results.
#[derive(Clone, Debug, PartialEq, Deserialize)]
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

/// Used for pagination related methods.
pub trait Pagination {
    /// Method to get a reference to the page links.
    ///
    /// Used by all the other methods in this trait.
    fn links(&self) -> &PageLinks;

    /// The current page.
    fn current_page(&self) -> u16 {
        self.links().current_page()
    }

    /// The first page.
    fn first_page(&self) -> u16 {
        self.links().first
    }

    /// The last page.
    fn last_page(&self) -> u16 {
        self.links().last
    }

    /// The next page, if available.
    fn next_page(&self) -> Option<u16> {
        self.links().next
    }

    /// The previous page, if available.
    fn prev_page(&self) -> Option<u16> {
        self.links().prev
    }
}

impl<E> Pagination for EpisodePage<E> {
    fn links(&self) -> &PageLinks {
        &self.links
    }
}

impl<E> Pagination for EpisodeQueryPage<E> {
    fn links(&self) -> &PageLinks {
        &self.links
    }
}

/// Episode summary data returned by [`Client::series_episodes_summary`].
///
/// See linked method for more info.
///
/// [`Client::series_episodes_summary`]: ../client/struct.Client.html#method.series_episodes_summary
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct EpisodeSummary {
    /// Number of aired seasons.
    pub aired_seasons: Vec<String>,
    /// Number of aired episodes.
    #[serde(with = "ser::u16_string")]
    pub aired_episodes: u16,
    /// Number of seasons on DVD.
    pub dvd_seasons: Vec<String>,
    /// Number of episodes on DVD.
    #[serde(with = "ser::u16_string")]
    pub dvd_episodes: u16,
}

/// Series image count data returned by [`Client::series_images`].
///
/// See linked method for more info.
///
/// [`Client::series_images`]: ../client/struct.Client.html#method.series_images
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct SeriesImages {
    /// Number of fan art images.
    pub fanart: Option<u32>,
    /// Number of poster images.
    pub poster: Option<u32>,
    /// Number of season images.
    pub season: Option<u32>,
    /// Number of wide season images.
    pub seasonwide: Option<u32>,
    /// Number of series images.
    pub series: Option<u32>,
}

/// Image data returned by [`Client::series_images_query`].
///
/// [`Client::series_images_query`]: ../client/struct.Client.html#method.series_images_query
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(test, derive(Default))]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Image {
    /// ID of the image.
    pub id: u32,
    /// Image key type (season, series, poster, etc...).
    pub key_type: String,
    /// Image subkey.
    #[serde(deserialize_with = "ser::optional_string")]
    pub sub_key: Option<String>,
    /// Image file path.
    ///
    /// For the full URL use [`file_name_url`](#method.file_name_url).
    pub file_name: String,
    /// ID of image's language.
    pub language_id: u16,
    /// Image language abbreviation.
    pub language: String,
    /// Image resolution.
    #[serde(deserialize_with = "ser::optional_string")]
    pub resolution: Option<String>,
    /// Image ratings data.
    ///
    /// See [`ImageRatingsInfo`](./struct.ImageRatingsInfo.html) for more info.
    pub ratings_info: ImageRatingsInfo,
    /// Image thumbnail file path.
    ///
    /// For the full URL use [`thumbnail_url`](#method.thumbnail_url).
    pub thumbnail: String,
}

impl Image {
    /// Returns the full URL of the image file.
    pub fn file_name_url(&self) -> Result<Url> {
        URLS.image(&self.file_name)
    }

    /// Returns the full URL of the image's thumbnail.
    pub fn thumbnail_url(&self) -> Result<Url> {
        URLS.image(&self.thumbnail)
    }
}

/// Image ratings data.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(test, derive(Default))]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct ImageRatingsInfo {
    /// Average rating.
    pub average: f32,
    /// Number of rating votes.
    pub count: u32,
}

/// Image query key data returned by [`Client::series_images_query_params`].
///
/// Can be used to see what types of images can be queried for a series.
///
/// See [`Client::series_images_query_params`] and [`Client::series_images_query`]
/// for more info.
///
/// [`Client::series_images_query_params`]: ../client/struct.Client.html#method.series_images_query_params
/// [`Client::series_images_query`]: ../client/struct.Client.html#method.series_images_query
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct ImageQueryKey {
    /// Key type name.
    pub key_type: String,
    /// Key language ID.
    #[serde(default, deserialize_with = "ser::optional_string")]
    pub language_id: Option<String>,
    /// Available resolutions.
    pub resolution: Vec<String>,
    /// Available subkeys.
    pub sub_key: Vec<String>,
}

/// Series update data returned by [`Client::updated`].
///
/// See linked method for more info.
///
/// [`Client::updated`]: ../client/struct.Client.html#method.updated
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct SeriesUpdate {
    /// ID of the series.
    pub id: SeriesID,
    /// Date and time that series was last updated.
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_updated: DateTime<Utc>,
}

#[cfg(test)]
mod tests;
