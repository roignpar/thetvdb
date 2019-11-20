use std::collections::HashMap;

use chrono::{DateTime, Duration, Utc};
use futures::lock::Mutex;
use reqwest::{header::HeaderValue, Client as HttpClient, Method, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::{Error, Result};
use crate::language::*;
use crate::params::*;
use crate::response::*;

const BASE_URL: &str = "https://api.thetvdb.com/";

#[derive(Debug)]
pub struct Client {
    base_url: Url,
    api_key: String,
    token: Mutex<Option<String>>,
    token_created: Mutex<Option<DateTime<Utc>>>,
    token_expires: Mutex<Option<DateTime<Utc>>>,
    http_client: HttpClient,
    lang_abbr: String,
}

impl Client {
    pub async fn new<S>(api_key: S) -> Result<Self>
    where
        S: Into<String>,
    {
        let client = Self::create(api_key);

        client.login().await?;

        Ok(client)
    }

    pub fn set_language(&mut self, language: Language) {
        self.lang_abbr = language.abbr().into();
    }

    pub fn set_language_abbr<S>(&mut self, abbr: S)
    where
        S: Into<String>,
    {
        self.lang_abbr = abbr.into();
    }

    pub async fn search<S>(&self, param: SearchBy<S>) -> Result<Vec<SearchSeries>>
    where
        S: Into<String>,
    {
        let param: HashMap<String, String> = param.into();

        let res = self
            .prep_lang_req(Method::GET, self.search_url())
            .await?
            .query(&param)
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<Vec<SearchSeries>>>().await?.data)
    }

    pub async fn series<I>(&self, id: I) -> Result<Series>
    where
        I: Into<SeriesID>,
    {
        let res = self
            .prep_lang_req(Method::GET, self.series_url(id.into()))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<Series>>().await?.data)
    }

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

    pub async fn series_actors<I>(&self, id: I) -> Result<Vec<Actor>>
    where
        I: Into<SeriesID>,
    {
        let res = self
            .prep_req(Method::GET, self.series_actors_url(id.into()))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<Vec<Actor>>>().await?.data)
    }

    pub async fn series_episodes(&self, params: EpisodeParams) -> Result<EpisodePage> {
        let res = self
            .prep_req(Method::GET, self.series_episodes_url(params.series_id))
            .await?
            .query(&[("page", params.page)])
            .send()
            .await?;

        api_errors(&res)?;

        let mut page: EpisodePage = res.json().await?;
        page.series_id = params.series_id;

        Ok(page)
    }

    pub async fn series_episodes_query(
        &self,
        query_params: EpisodeQueryParams,
    ) -> Result<EpisodeQueryPage> {
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

        let mut page: EpisodeQueryPage = res.json().await?;
        page.series_id = query_params.params.series_id;
        page.query = query_params.query;

        Ok(page)
    }

    pub async fn series_episodes_summary<I>(&self, id: I) -> Result<EpisodeSummary>
    where
        I: Into<SeriesID>,
    {
        let res = self
            .prep_req(Method::GET, self.series_episodes_summary_url(id.into()))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<EpisodeSummary>>().await?.data)
    }

    pub async fn series_filter<I>(
        &self,
        id: I,
        filter_keys: SeriesFilterKeys,
    ) -> Result<FilteredSeries>
    where
        I: Into<SeriesID>,
    {
        if filter_keys.is_empty() {
            return Err(Error::MissingSeriesFilterKeys);
        }

        let res = self
            .prep_lang_req(Method::GET, self.series_filter_url(id.into()))
            .await?
            .query(&[("keys", filter_keys.keys_query)])
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<FilteredSeries>>().await?.data)
    }

    pub async fn series_images<I>(&self, id: I) -> Result<SeriesImages>
    where
        I: Into<SeriesID>,
    {
        let res = self
            .prep_lang_req(Method::GET, self.series_images_url(id.into()))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<SeriesImages>>().await?.data)
    }

    pub async fn series_images_query<I>(
        &self,
        id: I,
        params: ImageQueryParams,
    ) -> Result<Vec<Image>>
    where
        I: Into<SeriesID>,
    {
        let res = self
            .prep_lang_req(Method::GET, self.series_images_query_url(id.into()))
            .await?
            .query(&params)
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<Vec<Image>>>().await?.data)
    }

    pub async fn series_images_query_params<I>(&self, id: I) -> Result<Vec<ImageQueryKey>>
    where
        I: Into<SeriesID>,
    {
        let res = self
            .prep_lang_req(Method::GET, self.series_images_query_params_url(id.into()))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<Vec<ImageQueryKey>>>().await?.data)
    }

    pub async fn episode<I>(&self, id: I) -> Result<Episode>
    where
        I: Into<EpisodeID>,
    {
        let id = id.into();

        let res = self
            .prep_lang_req(Method::GET, self.episodes_url(id))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        // the API will return an empty episode if id is not found
        let episode = res.json::<ResponseData<Episode>>().await?.data;
        if episode.id != id {
            return Err(Error::NotFound);
        }

        Ok(episode)
    }

    pub async fn languages(&self) -> Result<Vec<Language>> {
        let res = self
            .prep_req(Method::GET, self.languages_url())
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<Vec<Language>>>().await?.data)
    }

    pub async fn language<I>(&self, id: I) -> Result<Language>
    where
        I: Into<LanguageID>,
    {
        let res = self
            .prep_req(Method::GET, self.language_url(id.into()))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<Language>>().await?.data)
    }

    pub async fn updated(&self, params: UpdatedParams) -> Result<Vec<SeriesUpdate>> {
        let res = self
            .prep_lang_req(Method::GET, self.updated_url())
            .await?
            .query(&params)
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<Vec<SeriesUpdate>>>().await?.data)
    }

    fn create<S>(api_key: S) -> Self
    where
        S: Into<String>,
    {
        Client {
            base_url: Url::parse(BASE_URL).expect("could not parse BASE_URL"),
            api_key: api_key.into(),
            token: Mutex::new(None),
            token_created: Mutex::new(None),
            token_expires: Mutex::new(None),
            http_client: HttpClient::new(),
            lang_abbr: "en".to_string(),
        }
    }

    async fn login(&self) -> Result<()> {
        let res = self
            .http_client
            .post(self.login_url())
            .json(&AuthBody {
                apikey: &self.api_key,
            })
            .send()
            .await?;

        api_errors(&res)?;

        let TokenResp { token } = res.json().await?;

        self.set_token(token).await?;

        Ok(())
    }

    async fn ensure_valid_token(&self) -> Result<()> {
        match self.token_expires.lock().await.as_ref() {
            Some(moment) if *moment - Duration::minutes(1) >= Utc::now() => Ok(()),

            _ => self.login().await,
        }
    }

    async fn set_token(&self, new_token: String) -> Result<()> {
        // TheTVDB API JWT public key is not available,
        // thus the use of `dangerous_unsafe_decode`
        let payload = jsonwebtoken::dangerous_unsafe_decode::<TokenPayload>(&new_token)?.claims;

        futures::join!(
            async {
                let mut token = self.token.lock().await;
                *token = Some(new_token);
            },
            async {
                let mut token_created = self.token_created.lock().await;
                *token_created = Some(payload.orig_iat);
            },
            async {
                let mut token_expires = self.token_expires.lock().await;
                *token_expires = Some(payload.exp)
            }
        );

        Ok(())
    }

    async fn prep_req(&self, method: Method, url: Url) -> Result<RequestBuilder> {
        self.ensure_valid_token().await?;
        let req = self
            .http_client
            .request(method, url)
            .header("Content-Type", "application/json")
            .bearer_auth(
                self.token
                    .lock()
                    .await
                    .as_ref()
                    .expect("missing token although ensured valid"),
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
struct TokenResp {
    token: String,
}

#[derive(Debug, Deserialize)]
struct TokenPayload {
    #[serde(deserialize_with = "chrono::serde::ts_seconds::deserialize")]
    orig_iat: DateTime<Utc>,
    #[serde(deserialize_with = "chrono::serde::ts_seconds::deserialize")]
    exp: DateTime<Utc>,
}

#[cfg(test)]
mod test {
    use url::Url;

    use super::*;

    #[test]
    fn urls_must_parse() {
        Url::parse(BASE_URL).unwrap();
        let client = Client::create("key");

        client.login_url();
        client.search_url();
        client.series_url(1);
        client.series_actors_url(1);
        client.series_episodes_url(1);
        client.series_episodes_query_url(1);
        client.series_episodes_summary_url(1);
        client.series_filter_url(1);
        client.series_images_url(1);
        client.series_images_query_url(1);
        client.series_images_query_params_url(1);
        client.episodes_url(1);
        client.languages_url();
        client.language_url(1);
        client.updated_url();
    }
}
