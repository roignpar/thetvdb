//! TheTVDB API async client.
//!
//! Check [`Client`] documentation for more info.
//!
//! [`Client`]: struct.Client.html

use std::{
    convert::{TryFrom, TryInto},
    fmt,
};

use chrono::{DateTime, Duration, Utc};
use futures::lock::Mutex;
use reqwest::{header::HeaderValue, Client as HttpClient, Method, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use crate::error::{Error, Result};
use crate::language::*;
use crate::params::*;
use crate::response::*;

const BASE_URL: &str = "https://api.thetvdb.com/";
const TOKEN_EXP_LIMIT: i64 = 60;

/// TheTVDB API async client.
///
/// You will need a valid API key to create a new client.
/// To generate a key log in and go to the [API Keys page].
///
/// If you want to use your own types instead of the ones provided by this crate
/// to deserialize API responses, each client method `<method_name>` has an
/// alternative named `<method_name>_into` which can return data deserialized
/// into your types.
///
/// [API Keys page]: https://thetvdb.com/dashboard/account/apikeys
#[derive(Debug)]
pub struct Client {
    base_url: Url,
    api_key: String,
    token: Mutex<Option<TokenData>>,
    http_client: HttpClient,
    lang_abbr: String,
}

impl Client {
    /// Create a new client and authenticate using the given api key.
    ///
    /// # Errors
    /// Will fail if the api key is not valid.
    pub async fn new<S>(api_key: S) -> Result<Self>
    where
        S: Into<String>,
    {
        let client = Self::create(api_key);

        client.login_set_token().await?;

        Ok(client)
    }

    /// Set the language for the client.
    ///
    /// The language abbreviation will be set as the `Accept-Language` header
    /// when sending API requests that support it.
    ///
    /// The default language is **English**.
    /// # Examples
    /// ```no_run
    /// # use thetvdb::{Client, error::Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #
    /// # let mut client = Client::new("KEY").await?;
    /// #
    /// let planet_earth_ii = client.series(318408).await?;
    ///
    /// assert_eq!(
    ///     planet_earth_ii.series_name,
    ///     Some("Planet Earth II".to_string())
    /// );
    ///
    /// let korean = client.language(32).await?;
    ///
    /// client.set_language(&korean);
    ///
    /// let planet_earth_ii_ko = client.series(318408).await?;
    ///
    /// assert_eq!(
    ///     planet_earth_ii_ko.series_name,
    ///     Some("살아있는 지구 II".to_string())
    /// );
    ///
    /// assert_eq!(planet_earth_ii.id, planet_earth_ii_ko.id);
    /// # Ok(()) }
    /// ```
    pub fn set_language(&mut self, language: &Language) {
        self.lang_abbr = language.abbreviation.clone();
    }

    /// Set the language abbreviation directly.
    ///
    /// Read [`set_language`] documentation for more info.
    ///
    /// [`set_language`]: #method.set_language
    ///
    /// # Examples
    /// ```no_run
    /// # use thetvdb::{Client, error::Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #
    /// # let mut client = Client::new("KEY").await?;
    /// #
    /// client.set_language_abbr("ko");
    ///
    /// let planet_earth_ii_ko = client.series(318408).await?;
    ///
    /// assert_eq!(
    ///     planet_earth_ii_ko.series_name,
    ///     Some("살아있는 지구 II".to_string())
    /// );
    /// # Ok(()) }
    /// ```
    pub fn set_language_abbr<S>(&mut self, abbr: S)
    where
        S: Into<String>,
    {
        self.lang_abbr = abbr.into();
    }

    /// Search for series providing either a (partial) name, IMDb id, slug or
    /// Zap2it id.
    ///
    /// Sends a `GET` request to the `/search/series` API endpoint.
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
    /// use thetvdb::params::SearchBy;
    ///
    /// let results = client.search(SearchBy::IMDbID("tt5491994")).await?;
    ///
    /// assert_eq!(
    ///     results[0].series_name,
    ///     Some("Planet Earth II".to_string())
    /// );
    /// # Ok(()) }
    /// ```
    pub async fn search<S>(&self, param: SearchBy<S>) -> Result<Vec<SearchSeries>>
    where
        S: AsRef<str>,
    {
        self.search_into(param).await
    }

    /// Same as [`search`], but allows deserializing the response search series
    /// data into a provided type.
    ///
    /// [`search`]: #method.search
    pub async fn search_into<T, S>(&self, param: SearchBy<S>) -> Result<Vec<T>>
    where
        S: AsRef<str>,
        T: DeserializeOwned,
    {
        let res = self
            .prep_lang_req(Method::GET, self.search_url())
            .await?
            .query(&param.query_param())
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<Vec<T>>>().await?.data)
    }

    /// Get a series by its id.
    ///
    /// Sends a `GET` request to the `/series/{id}` API endpoint.
    ///
    /// References to `SearchSeries`, `Series`, `SeriesUpdate` or any type that
    /// impls `Into<SeriesID>` can also be used for ids.
    ///
    /// # Examples
    /// Use a literal id:
    /// ```no_run
    /// # use thetvdb::{Client, error::Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #
    /// # let client = Client::new("KEY").await?;
    /// #
    /// let series = client.series(318408).await?;
    ///
    /// assert_eq!(
    ///     series.series_name,
    ///     Some("Planet Earth II".to_string())
    /// );
    /// # Ok(()) }
    /// ```
    ///
    /// Use a search result:
    /// ```no_run
    /// # use thetvdb::{Client, error::Result, params::SearchBy};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #
    /// # let client = Client::new("KEY").await?;
    /// #
    /// let results = client.search(SearchBy::IMDbID("tt5491994")).await?;
    ///
    /// let series = client.series(&results[0]).await?;
    ///
    /// assert_eq!(
    ///     series.series_name,
    ///     Some("Planet Earth II".to_string())
    /// );
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    /// Will return an error if the series is not found.
    pub async fn series<I>(&self, id: I) -> Result<Series>
    where
        I: Into<SeriesID>,
    {
        self.series_into(id).await
    }

    /// Same as [`series`], but allows deserializing the response series data
    /// into a provided type.
    ///
    /// [`series`]: #method.series
    pub async fn series_into<T, I>(&self, id: I) -> Result<T>
    where
        I: Into<SeriesID>,
        T: DeserializeOwned,
    {
        let res = self
            .prep_lang_req(Method::GET, self.series_url(id.into()))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<T>>().await?.data)
    }

    /// Get the last modified time of a series.
    ///
    /// Sends a `HEAD` request to the `/series/{id}` API endpoint.
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
    /// use chrono::Utc;
    ///
    /// let last_modified = client.series_last_modified(318408).await?;
    ///
    /// assert!(last_modified < Utc::now());
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    /// Will return an error if the series is not found.
    pub async fn series_last_modified<I>(&self, id: I) -> Result<DateTime<Utc>>
    where
        I: Into<SeriesID>,
    {
        let res = self
            .prep_req(Method::HEAD, self.series_url(id.into()))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        let lm_header = res
            .headers()
            .get("Last-Modified")
            .ok_or(Error::MissingLastModified)
            .map(HeaderValue::to_str)??;

        Ok(DateTime::parse_from_rfc2822(lm_header)?.into())
    }

    /// Get a list of actors playing in a given series.
    ///
    /// Sends a `GET` request to the `/series/{id}/actors` API endpoint.
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
    /// let actors = client.series_actors(318408).await?;
    ///
    /// assert_eq!(&actors[0].name, "David Attenborough");
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    /// Will return an error if the series is not found.
    pub async fn series_actors<I>(&self, id: I) -> Result<Vec<Actor>>
    where
        I: Into<SeriesID>,
    {
        self.series_actors_into(id).await
    }

    /// Same as [`series_actors`], but allows deserializing the response actor
    /// data into a provided type.
    ///
    /// [`series_actors`]: #method.series_actors
    pub async fn series_actors_into<T, I>(&self, id: I) -> Result<Vec<T>>
    where
        I: Into<SeriesID>,
        T: DeserializeOwned,
    {
        let res = self
            .prep_req(Method::GET, self.series_actors_url(id.into()))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<Vec<T>>>().await?.data)
    }

    /// Get a page of a series' episodes.
    ///
    /// Sends a `GET` request to the `/series/{id}/episodes` API endpoint.
    ///
    /// A page contains 100 episodes at most.
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
    /// use thetvdb::params::EpisodeParams;
    ///
    /// // get the first page
    /// let episode_params = EpisodeParams::new(121361);
    /// let episode_page = client.series_episodes(&episode_params).await?;
    ///
    /// // get the next page
    /// let next_page_params = episode_page.next_page_params().unwrap();
    /// let next_page = client.series_episodes(&next_page_params).await?;
    ///
    /// // get the previous page
    /// let prev_page_params = next_page.prev_page_params().unwrap();
    /// let prev_page = client.series_episodes(&prev_page_params).await?;
    ///
    /// // get a custom page
    /// let custom_page_params = EpisodeParams::with_page(121361, 2);
    /// let custom_page = client.series_episodes(&custom_page_params).await?;
    ///
    /// // print an episode
    /// println!("{:#?}", episode_page.episodes[0]);
    /// # Ok(()) }
    /// ```
    /// # Errors
    /// Will return an error if the series is not found.
    pub async fn series_episodes(&self, params: &EpisodeParams) -> Result<EpisodePage> {
        self.series_episodes_into(params).await
    }

    /// Same as [`series_episodes`], but allows deserializing the response
    /// episode data into a provided type.
    ///
    /// [`series_episodes`]: #method.series_episodes
    pub async fn series_episodes_into<T>(&self, params: &EpisodeParams) -> Result<EpisodePage<T>>
    where
        T: DeserializeOwned,
    {
        let res = self
            .prep_req(Method::GET, self.series_episodes_url(params.series_id))
            .await?
            .query(&[("page", params.page)])
            .send()
            .await?;

        api_errors(&res)?;

        let mut page: EpisodePage<T> = res.json().await?;
        page.series_id = params.series_id;

        Ok(page)
    }

    /// Get a page of a series' episodes queried with the given params.
    ///
    /// Sends a `GET` request to the `/series/{id}/episodes/query` API endpoint.
    ///
    /// Check [`series_episodes`] for pagination examples.
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
    /// use thetvdb::params::EpisodeQueryParams;
    ///
    /// let query = EpisodeQueryParams::new(318408)
    ///     .absolute_number(1)
    ///     .aired_season(1)
    ///     .aired_episode(1)
    ///     .dvd_season(1)
    ///     .dvd_episode(1);
    ///
    /// let episode_page = client.series_episodes_query(&query).await?;
    ///
    /// assert_eq!(
    ///     episode_page.episodes[0].episode_name,
    ///     Some("Islands".to_string())
    /// );
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    /// Will return an error if the series is not found or the data set is
    /// empty.
    ///
    /// [`series_episodes`]: #method.series_episodes
    pub async fn series_episodes_query(
        &self,
        query_params: &EpisodeQueryParams,
    ) -> Result<EpisodeQueryPage> {
        self.series_episodes_query_into(query_params).await
    }

    /// Same as [`series_episodes_query`], but allows deserializing the response
    /// episode data into a provided type.
    ///
    /// [`series_episodes_query`]: #method.series_episodes_query
    pub async fn series_episodes_query_into<T>(
        &self,
        query_params: &EpisodeQueryParams,
    ) -> Result<EpisodeQueryPage<T>>
    where
        T: DeserializeOwned,
    {
        let res = self
            .prep_lang_req(
                Method::GET,
                self.series_episodes_query_url(query_params.params.series_id),
            )
            .await?
            .query(&[("page", query_params.params.page)])
            .query(&query_params.query)
            .send()
            .await?;

        api_errors(&res)?;

        let mut page: EpisodeQueryPage<T> = res.json().await?;
        page.series_id = query_params.params.series_id;
        page.query = query_params.query.clone();

        Ok(page)
    }

    /// Get the summary of a series' episodes.
    ///
    /// Sends a `GET` request to the `/series/{id}/episodes/summary` API
    /// endpoint.
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
    /// let summary = client.series_episodes_summary(318408).await?;
    ///
    /// assert_eq!(summary.aired_episodes, 18);
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    /// Will return an error if the series is not found.
    pub async fn series_episodes_summary<I>(&self, id: I) -> Result<EpisodeSummary>
    where
        I: Into<SeriesID>,
    {
        self.series_episodes_summary_into(id).await
    }

    /// Same as [`series_episodes_summary`], but allows deserializing the
    /// response episode summary data into a provided type.
    ///
    /// [`series_episodes_summary`]: #method.series_episodes_summary
    pub async fn series_episodes_summary_into<T, I>(&self, id: I) -> Result<T>
    where
        I: Into<SeriesID>,
        T: DeserializeOwned,
    {
        let res = self
            .prep_req(Method::GET, self.series_episodes_summary_url(id.into()))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<T>>().await?.data)
    }

    /// Get only selected fields of a series.
    ///
    /// Sends a `GET` request to the `/series/{id}/filter` API endpoint.
    ///
    /// This can be more efficient than getting all the fields.
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
    /// use thetvdb::params::SeriesFilterKeys;
    ///
    /// let keys = SeriesFilterKeys::new().series_name();
    ///
    /// let filtered_series = client.series_filter(318408, &keys).await?;
    ///
    /// assert_eq!(
    ///     filtered_series.series_name,
    ///     Some("Planet Earth II".to_string())
    /// );
    ///
    /// assert_eq!(filtered_series.id, None);
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    /// Will return an error if the series is not found.
    pub async fn series_filter<I>(
        &self,
        id: I,
        filter_keys: &SeriesFilterKeys,
    ) -> Result<FilteredSeries>
    where
        I: Into<SeriesID>,
    {
        self.series_filter_into(id, filter_keys).await
    }

    /// Same as [`series_filter`], but allows deserializing the response series
    /// data into a provided type.
    ///
    /// [`series_filter`]: #method.series_filter
    pub async fn series_filter_into<T, I>(&self, id: I, filter_keys: &SeriesFilterKeys) -> Result<T>
    where
        I: Into<SeriesID>,
        T: DeserializeOwned,
    {
        if filter_keys.is_empty() {
            return Err(Error::MissingSeriesFilterKeys);
        }

        let res = self
            .prep_lang_req(Method::GET, self.series_filter_url(id.into()))
            .await?
            .query(&[("keys", &filter_keys.keys_query)])
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<T>>().await?.data)
    }

    /// Get a summary of a series' images.
    ///
    /// Sends a `GET` request to the `/series/{id}/images` API endpoint.
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
    /// let image_summary = client.series_images(318408).await?;
    ///
    /// assert_eq!(image_summary.poster, Some(8));
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    /// Will return an error if the series is not found.
    pub async fn series_images<I>(&self, id: I) -> Result<SeriesImages>
    where
        I: Into<SeriesID>,
    {
        self.series_images_into(id).await
    }

    /// Same as [`series_images`], but allows deserializing the response image
    /// summary data into a provided type.
    ///
    /// [`series_images`]: #method.series_images
    pub async fn series_images_into<T, I>(&self, id: I) -> Result<T>
    where
        I: Into<SeriesID>,
        T: DeserializeOwned,
    {
        let res = self
            .prep_lang_req(Method::GET, self.series_images_url(id.into()))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<T>>().await?.data)
    }

    /// Get a series' images based on query parameters.
    ///
    /// Sends a `GET` request to the `/series/{id}/images/query` API endpoint.
    ///
    /// Each series may have different available image types.
    /// To find out which key types, resolutions and subkeys are available for
    /// a series use the [`series_images_query_params`] method.
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
    /// use thetvdb::params::ImageQueryParams;
    ///
    /// let params = ImageQueryParams::with_key_type("poster");
    ///
    /// let images = client.
    ///     series_images_query(318408, &params)
    ///     .await?;
    ///
    /// assert_eq!(images.len(), 8);
    ///
    /// // print an image's URL
    /// println!("{}", images[0].file_name_url()?);
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    /// Will return an error if the series is not found or the data set is
    /// empty.
    ///
    /// [`series_images_query_params`]: #method.series_images_query_params
    pub async fn series_images_query<I>(
        &self,
        id: I,
        params: &ImageQueryParams,
    ) -> Result<Vec<Image>>
    where
        I: Into<SeriesID>,
    {
        self.series_images_query_into(id, params).await
    }

    /// Same as [`series_images_query`], but allows deserializing the response
    /// image data into a provided type.
    ///
    /// [`series_images_query`]: #method.series_images_query
    pub async fn series_images_query_into<T, I>(
        &self,
        id: I,
        params: &ImageQueryParams,
    ) -> Result<Vec<T>>
    where
        I: Into<SeriesID>,
        T: DeserializeOwned,
    {
        let res = self
            .prep_lang_req(Method::GET, self.series_images_query_url(id.into()))
            .await?
            .query(&params)
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<Vec<T>>>().await?.data)
    }

    /// Get a series' available image key types, resolutions and subkeys.
    ///
    /// Sends a `GET` request to the `/series/{id}/images/query/params` API
    /// endpoint.
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
    /// let image_keys = client
    ///     .series_images_query_params(318408)
    ///     .await?;
    ///
    /// // print resolutions available for the first image key type
    /// println!("{:#?}", image_keys[0].resolution);
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    /// Will return an error if the series is not found.
    pub async fn series_images_query_params<I>(&self, id: I) -> Result<Vec<ImageQueryKey>>
    where
        I: Into<SeriesID>,
    {
        self.series_images_query_params_into(id).await
    }

    /// Same as [`series_images_query_params`], but allows deserializing the
    /// response image key data into a provided type.
    ///
    /// [`series_images_query_params`]: #method.series_images_query_params
    pub async fn series_images_query_params_into<T, I>(&self, id: I) -> Result<Vec<T>>
    where
        I: Into<SeriesID>,
        T: DeserializeOwned,
    {
        let res = self
            .prep_lang_req(Method::GET, self.series_images_query_params_url(id.into()))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<Vec<T>>>().await?.data)
    }

    /// Get an episode by its id.
    ///
    /// Sends a `GET` request to the `/episodes/{id}` API endpoint.
    ///
    /// # Examples
    /// ```no_run
    /// # use thetvdb::{Client, error::Result, response::SeriesID};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #
    /// # let client = Client::new("KEY").await?;
    /// #
    /// let episode = client.episode(5812389).await?;
    ///
    /// assert_eq!(episode.episode_name, Some("Islands".to_string()));
    /// assert_eq!(episode.series_id, SeriesID(318408));
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    /// Will return an error if the episode is not found.
    pub async fn episode<I>(&self, id: I) -> Result<Episode>
    where
        I: Into<EpisodeID>,
    {
        self.episode_into(id).await
    }

    /// Same as [`episode`], but allows deserializing the response episode data
    /// into a provided type.
    ///
    /// [`episode`]: #method.episode
    pub async fn episode_into<T, I>(&self, id: I) -> Result<T>
    where
        I: Into<EpisodeID>,
        T: DeserializeOwned,
    {
        let id = id.into();

        let res = self
            .prep_lang_req(Method::GET, self.episodes_url(id))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<T>>().await?.data)
    }

    /// Get a list of all the available languages.
    ///
    /// Sends a `GET` request to the `/languages` API endpoint.
    ///
    /// Languages can be used to [`set_language`] on the client.
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
    /// let languages = client.languages().await?;
    ///
    /// // print all the languages
    /// println!("{:#?}", languages);
    /// # Ok(()) }
    /// ```
    ///
    /// [`set_language`]: #method.set_language
    pub async fn languages(&self) -> Result<Vec<Language>> {
        self.languages_into().await
    }

    /// Same as [`languages`], but allows deserializing the response language
    /// data into a provided type.
    ///
    /// [`languages`]: #method.languages
    pub async fn languages_into<T>(&self) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let res = self
            .prep_req(Method::GET, self.languages_url())
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<Vec<T>>>().await?.data)
    }

    /// Get a language by its id.
    ///
    /// Sends a `GET` request to the `/languages/{id}` API endpoint.
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
    /// assert_eq!(japanese.abbreviation, "ja".to_string());
    /// assert_eq!(japanese.name, "日本語".to_string());
    /// assert_eq!(japanese.english_name, "Japanese".to_string());
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    /// Will return an error if the language is not found.
    pub async fn language<I>(&self, id: I) -> Result<Language>
    where
        I: Into<LanguageID>,
    {
        self.language_into(id).await
    }

    /// Same as [`language`], but allows deserializing the response language
    /// data into a provided type.
    ///
    /// [`language`]: #method.language
    pub async fn language_into<T, I>(&self, id: I) -> Result<T>
    where
        I: Into<LanguageID>,
        T: DeserializeOwned,
    {
        let res = self
            .prep_req(Method::GET, self.language_url(id.into()))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<T>>().await?.data)
    }

    /// Get a list of series updated within a given time period.
    ///
    /// Sends a `GET` request to the `/updated/query` API endpoint.
    ///
    /// If `to_time` is not set or more than one week after `from_time`, the API
    /// sets the timespan to one week.
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
    /// use thetvdb::params::UpdatedParams;
    /// use chrono::DateTime;
    ///
    /// let from = DateTime::parse_from_rfc3339("2019-11-10T12:00:00-00:00")?;
    /// let to = DateTime::parse_from_rfc3339("2019-11-10T12:10:00-00:00")?;
    ///
    /// let timespan = UpdatedParams::with_to_time(from, to);
    ///
    /// let updates = client.updated(&timespan).await?;
    ///
    /// assert_eq!(updates.len(), 7);
    ///
    /// // results can be used to fetch full series data
    /// let series = client.series(&updates[0]).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    /// Will return an error if there are no updated series within the
    /// given timespan.
    pub async fn updated(&self, params: &UpdatedParams) -> Result<Vec<SeriesUpdate>> {
        self.updated_into(params).await
    }

    /// Same as [`updated`], but allows deserializing the response series
    /// updated data into a provided type.
    ///
    /// [`updated`]: #method.updated
    pub async fn updated_into<T>(&self, params: &UpdatedParams) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let res = self
            .prep_lang_req(Method::GET, self.updated_url())
            .await?
            .query(&params)
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<Vec<T>>>().await?.data)
    }

    /// Get a movie by its id.
    ///
    /// Sends a `GET` request to the `/movies/{id}` API endpoint.
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
    /// let movie = client.movie(190).await?;
    ///
    /// assert_eq!(movie.runtime, 142);
    /// # Ok(()) }
    /// ```
    pub async fn movie<I>(&self, id: I) -> Result<Movie>
    where
        I: Into<MovieID>,
    {
        self.movie_into(id).await
    }

    /// Same as [`movie`], but allows deserializing the response movie data into
    /// a provided type.
    ///
    /// [`movie`]: #method.movie
    pub async fn movie_into<T, I>(&self, id: I) -> Result<T>
    where
        I: Into<MovieID>,
        T: DeserializeOwned,
    {
        let res = self
            .prep_lang_req(Method::GET, self.movies_url(id.into()))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<T>>().await?.data)
    }

    /// Get a list of movies updated since the given time.
    ///
    /// Sends a `GET` request to the `/movieupdates` API endpoint.
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
    /// use chrono::DateTime;
    ///
    /// let since = DateTime::parse_from_rfc3339("2020-01-20T00:00:00-00:00")?;
    ///
    /// let movie_updates = client.movie_updates(since).await?;
    ///
    /// println!("Movie IDs updated since 2020-01-20: {:?}", movie_updates.movies);
    /// # Ok(())}
    /// ```
    pub async fn movie_updates<D>(&self, since: D) -> Result<MovieUpdates>
    where
        D: Into<DateTime<Utc>>,
    {
        self.movie_updates_into(since).await
    }

    /// Same as [`movie_updates`], but allows deserializing the response data
    /// into a provided type.
    ///
    /// [`movie_updates`]: #method.movie_updates
    pub async fn movie_updates_into<T, D>(&self, since: D) -> Result<T>
    where
        D: Into<DateTime<Utc>>,
        T: DeserializeOwned,
    {
        let res = self
            .prep_req(Method::GET, self.movie_updates_url())
            .await?
            .query(&[("since", since.into().timestamp())])
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<T>().await?)
    }

    fn create<S>(api_key: S) -> Self
    where
        S: Into<String>,
    {
        Client {
            base_url: Url::parse(BASE_URL).expect("could not parse BASE_URL"),
            api_key: api_key.into(),
            token: Mutex::new(None),
            http_client: HttpClient::new(),
            lang_abbr: "en".to_string(),
        }
    }

    async fn login_set_token(&self) -> Result<()> {
        self.set_token(self.login().await?).await
    }

    async fn login(&self) -> Result<TokenData> {
        let res = self
            .http_client
            .post(self.login_url())
            .json(&AuthBody {
                apikey: &self.api_key,
            })
            .send()
            .await?;

        api_errors(&res)?;

        let token_res: TokenRes = res.json().await?;

        Ok(token_res.try_into()?)
    }

    async fn ensure_valid_token(&self) -> Result<()> {
        let mut guard = self.token.lock().await;

        match guard.as_ref() {
            Some(t) if t.exp - Duration::seconds(TOKEN_EXP_LIMIT) >= Utc::now() => {}

            _ => *guard = Some(self.login().await?),
        }

        Ok(())
    }

    async fn set_token(&self, new_token: TokenData) -> Result<()> {
        let mut token = self.token.lock().await;

        *token = Some(new_token);

        Ok(())
    }

    async fn prep_req(&self, method: Method, url: Url) -> Result<RequestBuilder> {
        self.ensure_valid_token().await?;
        let req = self
            .http_client
            .request(method, url)
            .header("Content-Type", "application/json")
            .bearer_auth(
                &self
                    .token
                    .lock()
                    .await
                    .as_ref()
                    .expect("missing token although ensured valid")
                    .token,
            );

        Ok(req)
    }

    async fn prep_lang_req(&self, method: Method, url: Url) -> Result<RequestBuilder> {
        self.prep_req(method, url)
            .await
            .map(|r| r.header("Accept-Language", &self.lang_abbr))
    }

    fn login_url(&self) -> Url {
        self.base_url
            .join("/login")
            .expect("could not parse login url")
    }

    fn search_url(&self) -> Url {
        self.base_url
            .join("/search/series")
            .expect("could not parse search url")
    }

    fn series_url(&self, id: SeriesID) -> Url {
        self.base_url
            .join(&format!("/series/{}", id))
            .expect("could not parse series url")
    }

    fn series_actors_url(&self, id: SeriesID) -> Url {
        self.base_url
            .join(&format!("/series/{}/actors", id))
            .expect("could not parse actors url")
    }

    fn series_episodes_url(&self, id: SeriesID) -> Url {
        self.base_url
            .join(&format!("/series/{}/episodes", id))
            .expect("could not parse episodes url")
    }

    fn series_episodes_query_url(&self, id: SeriesID) -> Url {
        self.base_url
            .join(&format!("/series/{}/episodes/query", id))
            .expect("could not parse episodes query url")
    }

    fn series_episodes_summary_url(&self, id: SeriesID) -> Url {
        self.base_url
            .join(&format!("/series/{}/episodes/summary", id))
            .expect("could not parse episodes summary url")
    }

    fn series_filter_url(&self, id: SeriesID) -> Url {
        self.base_url
            .join(&format!("/series/{}/filter", id))
            .expect("could not parse series filter url")
    }

    fn series_images_url(&self, id: SeriesID) -> Url {
        self.base_url
            .join(&format!("/series/{}/images", id))
            .expect("could not parse series images url")
    }

    fn series_images_query_url(&self, id: SeriesID) -> Url {
        self.base_url
            .join(&format!("/series/{}/images/query", id))
            .expect("could not parse series images query url")
    }

    fn series_images_query_params_url(&self, id: SeriesID) -> Url {
        self.base_url
            .join(&format!("/series/{}/images/query/params", id))
            .expect("could not parse series images query params url")
    }

    fn episodes_url(&self, id: EpisodeID) -> Url {
        self.base_url
            .join(&format!("/episodes/{}", id))
            .expect("could not parse episodes url")
    }

    fn languages_url(&self) -> Url {
        self.base_url
            .join("/languages")
            .expect("could not parse languages url")
    }

    fn language_url(&self, id: LanguageID) -> Url {
        self.base_url
            .join(&format!("/languages/{}", id))
            .expect("could not parse language url")
    }

    fn updated_url(&self) -> Url {
        self.base_url
            .join("/updated/query")
            .expect("could not parse updated url")
    }

    fn movies_url(&self, id: MovieID) -> Url {
        self.base_url
            .join(&format!("/movies/{}", id))
            .expect("could not parse movie url")
    }

    fn movie_updates_url(&self) -> Url {
        self.base_url
            .join("/movieupdates")
            .expect("could not parse movie updates url")
    }
}

fn api_errors(res: &Response) -> Result<()> {
    match res.status().into() {
        401 => Err(Error::InvalidAPIKey),
        404 => Err(Error::NotFound),
        500..=599 => Err(Error::ServerError),
        _ => Ok(()),
    }
}

#[derive(Debug, Serialize)]
struct AuthBody<'a> {
    apikey: &'a str,
}

#[derive(Debug, Deserialize)]
struct TokenRes {
    token: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TokenPayload {
    #[serde(with = "chrono::serde::ts_seconds")]
    orig_iat: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    exp: DateTime<Utc>,
}

#[derive(Debug)]
struct TokenData {
    token: String,
    created: DateTime<Utc>,
    exp: DateTime<Utc>,
}

impl fmt::Display for TokenData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<TokenRes> for TokenData {
    type Error = Error;

    fn try_from(res: TokenRes) -> Result<TokenData> {
        // TheTVDB API JWT public key is not available,
        // thus the use of `dangerous_unsafe_decode`
        let payload = jsonwebtoken::dangerous_unsafe_decode::<TokenPayload>(&res.token)?.claims;

        Ok(TokenData {
            token: res.token,
            created: payload.orig_iat,
            exp: payload.exp,
        })
    }
}

#[cfg(test)]
mod tests;
