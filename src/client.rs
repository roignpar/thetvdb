use std::collections::HashMap;
use std::time::{Duration, Instant};

use futures::lock::Mutex;
use reqwest::{Client as HttpClient, Method, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::{Error, Result};
use crate::params::*;
use crate::response::*;

const BASE_URL: &str = "https://api.thetvdb.com/";

// 23 hours and 59 minutes
const TOKEN_VALID: Duration = Duration::from_secs(60 * 60 * 24 - 60);

#[derive(Debug)]
pub struct Client {
    base_url: Url,
    api_key: String,
    token: Mutex<Option<String>>,
    token_created: Mutex<Option<Instant>>,
    http_client: HttpClient,
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

    pub async fn search<S>(&self, param: SearchBy<S>) -> Result<Vec<SearchSeries>>
    where
        S: Into<String>,
    {
        let param: HashMap<String, String> = param.into();

        let res = self
            .prep_req(Method::GET, self.search_url())
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
        let id = id.into();

        let res = self
            .prep_req(Method::GET, self.series_url(id))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<Series>>().await?.data)
    }

    fn create<S>(api_key: S) -> Self
    where
        S: Into<String>,
    {
        Client {
            base_url: Url::parse(BASE_URL).unwrap(),
            api_key: api_key.into(),
            token: Mutex::new(None),
            token_created: Mutex::new(None),
            http_client: HttpClient::new(),
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

        self.set_token(token).await;

        Ok(())
    }

    async fn ensure_valid_token(&self) -> Result<()> {
        match (
            self.token_created.lock().await.as_ref(),
            self.token.lock().await.as_ref(),
        ) {
            (Some(moment), Some(_)) if moment.elapsed() <= TOKEN_VALID => Ok(()),

            _ => self.login().await,
        }
    }

    async fn set_token(&self, new_token: String) {
        futures::join!(
            async {
                let mut token = self.token.lock().await;
                *token = Some(new_token);
            },
            async {
                let mut token_created = self.token_created.lock().await;
                *token_created = Some(Instant::now());
            },
        );
    }

    async fn prep_req(&self, method: Method, url: Url) -> Result<RequestBuilder> {
        self.ensure_valid_token().await?;
        let req = self
            .http_client
            .request(method, url)
            .header("Content-Type", "application/json")
            .bearer_auth(self.token.lock().await.as_ref().unwrap());

        Ok(req)
    }

    fn login_url(&self) -> Url {
        self.base_url.join("/login").unwrap()
    }

    fn search_url(&self) -> Url {
        self.base_url.join("/search/series").unwrap()
    }

    fn series_url(&self, id: SeriesID) -> Url {
        self.base_url.join(&format!("/series/{}", id)).unwrap()
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

#[derive(Serialize)]
struct AuthBody<'a> {
    apikey: &'a str,
}

#[derive(Deserialize)]
struct TokenResp {
    token: String,
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
    }
}
