use chrono::{Duration, Utc};
use futures::executor::block_on;
use jsonwebtoken as jwt;
use mockito::{
    mock,
    Matcher::{self, AllOf, UrlEncoded},
    Mock,
};
use serde::Serialize;
use serde_json::json;
use url::Url;

use super::*;
use crate::test_util::*;

const POST: &str = "POST";
const GET: &str = "GET";
const HEAD: &str = "HEAD";

const LOGIN_PATH: &str = "/login";
const SEARCH_PATH: &str = "/search/series";

const SERIES_ID: u32 = 32167;
const EPISODE_ID: u32 = 76123;
const LANGUAGE_ID: u16 = 5;
const MOVIE_ID: u32 = 9181;

const API_KEY: &str = "TEST_API_KEY";

const RSA_KEY: &[u8] = b"-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEA1lQpUqmD6BXZHCwcQOuKIZK6HPmhgv8SgTHvrYpRq7K6sd+4
lfKLKfBDXkGGvmwKA3mTd0myFFx0/KTBqHbx5v4AMNvlWJZsZM2KQFNHVaiFu/4g
vpuK/n3a2EANtoFo3qxK27t4ZnoJ0ZYnZKtH6SCq6VRH8yB3ntGm2InRsHdSEbqU
ccxuE9JITOSXNPK99reGE+4a++nfWta3i8OLFk+8n+Pm7hqLISmSrZlJRA9qrQzR
P8a+i/eWgb8hTn4tJfyye2hrQc6CkKbUdm4z+RGQn1ZyMGhUaoqL1CAuIGZLRPlI
leVSkJkIUrBrQdOOREDq+JerV1efZTC7NmA4rwIDAQABAoIBAEEEjPiXlf1My20B
cJy9F00x/qWVkMp1aH6q0GObx0TH1lzpWkyapF7XlQg95otTqQH+2p5gS9ZjadXO
gTUCyEVjnlk0C/IAx+cYdy0mVkiE8/Tglbxc4SBsu3vIiqBnx5FtQNRBnbewo9Ph
sheW4hy1nUNlHuKBYeha/ztHK8ZVDfxcIxBTAWhHlAkpzaCtdvciCvroxR4nLLNI
gcr/qs1ji7e9bYmWICNbwLoUEa++MMrpZla3wyWqKCVbOXekbuOa/nrc4mBf9m8P
iAh8Ju2Yh537dnf1jo2G/d050kYVAhCEhNhRcOfJKIMvCIzc4zPdqCJcKbO3j5Go
95b7knECgYEA7iF8nbCvKPgUkE6AmT6uHZuHkHAdY/IUgkV5RZK/zwNUVdF5Cmh2
ZRiR8stB1ZAmwKuDCH74WqqGeTarl1H/e4CM13HuOZWUp3ZwZ0vXP+05q1IaksM7
aqkWuFsweE0IpLs28uy20UoQc7hwhsQQ7tXvfW1AUmpjqmidMNqWUFcCgYEA5mlv
Z5YRIXmf6w2FzwiNOrafxAbkJTzLmuKkMDfcA0tC4NjiXZ9v+d+YM/EP7OZftuGd
Vy4M1KK6k/UiDaArvGXQqTGIIuZio3mg92I9cVAvwAe5kYfHOEoAf4BOS6IpH/vM
v0Obc36fjGvmTOwEXrQPRJIY/DCT2uicVPxRw2kCgYEAwc4rN8PGqxdM1S2u6Anl
d19Pc7Rrlx48ptGYI5GN3hOdncLAOkJOqXTXw7y9Nez+PSzZlh6QTmoey6Q8/wjT
X1Z+ShIFioQ82hZA4TEOXI7bsp0911hhkStQvHYq+p3nfPqJfCg/mhInVac0ndoQ
1fDz2ycBhJHlz5kY5dvhHj0CgYArS+a2RVnEu5EUXz8kr0uym7yb3luir7Dm0u3d
bbG3IL5ZeAfZZetpvd1g7Ux7zLJxsCWbyzh4AHgk82xJuS5ewZcbJAab4ETqySE5
O0mmcFX4tLrnIOeLqUQ8sUSFK9ykePF5g1/DrSiX/3KQQ81yak1wKi/tu8cc3Jb7
1Uob+QKBgBglxXA8qjXKFq8lT1DieB9KG95Z16XVuWJt0P5CqnRGRACVcrQfIV67
3yzdwaM24iRG90uS2clFl0QnMjwRZ38Q4sm1gHs07B0ikhaYrCKlSI3FSTgI4eRs
kgbrwQ/fvhtsiXpld02tqnLRYzFCv7jKRYwEJs73kYT3m5tRJgoe
-----END RSA PRIVATE KEY-----";

#[test]
fn urls_must_parse() {
    Url::parse(BASE_URL).unwrap();
    let client = Client::create("key");

    client.login_url();
    client.search_url();
    client.series_url(SeriesID(1));
    client.series_actors_url(SeriesID(1));
    client.series_episodes_url(SeriesID(1));
    client.series_episodes_query_url(SeriesID(1));
    client.series_episodes_summary_url(SeriesID(1));
    client.series_filter_url(SeriesID(1));
    client.series_images_url(SeriesID(1));
    client.series_images_query_url(SeriesID(1));
    client.series_images_query_params_url(SeriesID(1));
    client.episodes_url(EpisodeID(1));
    client.languages_url();
    client.language_url(LanguageID(1));
    client.updated_url();
    client.movies_url(MovieID(1));
}

#[tokio::test]
async fn client_login() -> Result<()> {
    let client = test_client();

    let req_body = json!({ "apikey": API_KEY });

    let now = now_round_seconds();
    let later = now + Duration::days(1);
    let token = create_jwt(&TokenPayload {
        orig_iat: now,
        exp: later,
    });

    let res_body = json!({ "token": token });

    let login_mock = mock(POST, LOGIN_PATH)
        .match_body(Matcher::Json(req_body))
        .with_body(serde_json::to_string(&res_body)?)
        .create();

    let _ = client.login_set_token().await;

    login_mock.assert();

    let guard = client.token.lock().await;
    let cl_token = guard.as_ref().unwrap();

    assert_eq!(cl_token.token, token);
    assert_eq!(cl_token.created, now);
    assert_eq!(cl_token.exp, later);

    Ok(())
}

#[tokio::test]
async fn client_relogin_on_token_exp() -> Result<()> {
    let client = test_client();

    let now = now_round_seconds();
    let exp = now + Duration::seconds(TOKEN_EXP_LIMIT - TOKEN_EXP_LIMIT / 2);
    let token = create_jwt(&TokenPayload { orig_iat: now, exp });

    let req_body = json!({ "apikey": API_KEY });
    let res_body = serde_json::to_string(&json!({ "token": token }))?;

    let login_mock = mock(POST, LOGIN_PATH).with_body(res_body.clone()).create();

    let _ = client.login_set_token().await;

    login_mock.assert();

    let relogin_mock = mock(POST, LOGIN_PATH)
        .match_body(Matcher::Json(req_body))
        .with_body(res_body)
        .create();

    let series_mock = auth_lang_mock(&client, GET, series_url().as_str()).create();

    let _ = client.series(SERIES_ID).await;

    relogin_mock.assert();
    series_mock.assert();

    Ok(())
}

#[test]
fn client_set_language() {
    let mut client = test_client();

    assert_eq!(client.lang_abbr, "en".to_string());

    let mut abbr = "ts".to_string();

    let language = &Language {
        id: LanguageID(1),
        abbreviation: abbr.clone(),
        name: "Test".to_string(),
        english_name: "Test".to_string(),
    };

    client.set_language(language);

    assert_eq!(client.lang_abbr, abbr);

    abbr = "st".to_string();

    client.set_language_abbr(abbr.clone());

    assert_eq!(client.lang_abbr, abbr);
}

#[tokio::test]
async fn client_search() {
    let client = authenticated_test_client().await;

    let name = "test+series".to_string();
    let imdb_id = "tttest".to_string();
    let zap2it_id = "1234567".to_string();
    let slug = "slug-test".to_string();

    let cases = vec![
        ("name", name.clone(), SearchBy::Name(&name)),
        ("imdbId", imdb_id.clone(), SearchBy::IMDbID(&imdb_id)),
        (
            "zap2itId",
            zap2it_id.clone(),
            SearchBy::Zap2itID(&zap2it_id),
        ),
        ("slug", slug.clone(), SearchBy::Slug(&slug)),
    ];

    for (query_key, query_value, search_by) in cases.into_iter() {
        let mock = auth_lang_mock(&client, GET, SEARCH_PATH)
            .match_query(UrlEncoded(query_key.to_string(), query_value))
            .create();

        let _ = client.search(search_by).await;

        mock.assert();
    }
}

#[tokio::test]
async fn client_series() {
    let client = authenticated_test_client().await;

    let series_mock = auth_lang_mock(&client, GET, series_url().as_str()).create();

    let _ = client.series(SERIES_ID).await;

    series_mock.assert();
}

#[tokio::test]
async fn client_series_last_modified() {
    let client = authenticated_test_client().await;

    let url = format!("/series/{}", SERIES_ID);

    let last_modified_mock = auth_mock(&client, HEAD, url.as_str()).create();

    let _ = client.series_last_modified(SERIES_ID).await;

    last_modified_mock.assert();
}

#[tokio::test]
async fn client_series_actors() {
    let client = authenticated_test_client().await;

    let url = format!("/series/{}/actors", SERIES_ID);

    let actors_mock = auth_mock(&client, GET, url.as_str()).create();

    let _ = client.series_actors(SERIES_ID).await;

    actors_mock.assert();
}

#[tokio::test]
async fn client_series_episodes() {
    let client = authenticated_test_client().await;

    let url = format!("/series/{}/episodes", SERIES_ID);

    let cases = &[
        EpisodeParams::new(SERIES_ID),
        EpisodeParams::with_page(SERIES_ID, 32),
    ];

    for params in cases {
        let mock = auth_mock(&client, GET, url.as_str())
            .match_query(UrlEncoded("page".to_string(), params.page.to_string()))
            .create();

        let _ = client.series_episodes(&params).await;

        mock.assert();
    }
}

#[tokio::test]
async fn client_series_episodes_query() {
    let client = authenticated_test_client().await;

    let url = format!("/series/{}/episodes/query", SERIES_ID);

    let (page, absolute_number, aired_season, aired_episode, dvd_season, dvd_episode, imdb_id) =
        (3, 210, 5, 36, 6, 12, "tt12345678");

    let params = EpisodeQueryParams::new(SERIES_ID)
        .page(page)
        .absolute_number(absolute_number)
        .aired_season(aired_season)
        .aired_episode(aired_episode)
        .dvd_season(dvd_season)
        .dvd_episode(dvd_episode)
        .imdb_id(imdb_id);

    let episodes_query_mock = auth_lang_mock(&client, GET, url.as_str())
        .match_query(AllOf(vec![
            UrlEncoded("page".to_string(), page.to_string()),
            UrlEncoded("absoluteNumber".to_string(), absolute_number.to_string()),
            UrlEncoded("airedSeason".to_string(), aired_season.to_string()),
            UrlEncoded("airedEpisode".to_string(), aired_episode.to_string()),
            UrlEncoded("dvdSeason".to_string(), dvd_season.to_string()),
            UrlEncoded("dvdEpisode".to_string(), dvd_episode.to_string()),
            UrlEncoded("imdbId".to_string(), imdb_id.to_string()),
        ]))
        .create();

    let _ = client.series_episodes_query(&params).await;

    episodes_query_mock.assert();
}

#[tokio::test]
async fn client_episodes_summary() {
    let client = authenticated_test_client().await;

    let url = format!("/series/{}/episodes/summary", SERIES_ID);

    let episodes_summary_mock = auth_mock(&client, GET, url.as_str()).create();

    let _ = client.series_episodes_summary(SERIES_ID).await;

    episodes_summary_mock.assert();
}

#[tokio::test]
async fn client_series_filter() {
    let client = authenticated_test_client().await;

    let url = format!("/series/{}/filter", SERIES_ID);

    let params = SeriesFilterKeys::new()
        .network_id()
        .airs_time()
        .site_rating()
        .series_name()
        .first_aired()
        .runtime()
        .overview()
        .banner()
        .genre()
        .airs_day_of_week()
        .imdb_id()
        .added_by()
        .site_rating_count()
        .id()
        .status()
        .network()
        .rating()
        .zap2it_id()
        .added()
        .slug()
        .aliases()
        .season()
        .poster()
        .fanart()
        .language();

    assert!(params.is_at_full_capacity());

    let series_filter_mock = auth_lang_mock(&client, GET, url.as_str())
        .match_query(UrlEncoded("keys".to_string(), params.keys_query.clone()))
        .create();

    let _ = client.series_filter(SERIES_ID, &params).await;

    series_filter_mock.assert();
}

#[tokio::test]
async fn client_series_images() {
    let client = authenticated_test_client().await;

    let url = format!("/series/{}/images", SERIES_ID);

    let images_mock = auth_lang_mock(&client, GET, url.as_str()).create();

    let _ = client.series_images(SERIES_ID).await;

    images_mock.assert();
}

#[tokio::test]
async fn client_series_images_query() {
    let client = authenticated_test_client().await;

    let url = format!("/series/{}/images/query", SERIES_ID);

    let (key_type, resolution, sub_key) = ("test_key", "123x321", "test_sub_key");

    let images_query_mock = auth_lang_mock(&client, GET, url.as_str())
        .match_query(AllOf(vec![
            UrlEncoded("keyType".to_string(), key_type.to_string()),
            UrlEncoded("resolution".to_string(), resolution.to_string()),
            UrlEncoded("subKey".to_string(), sub_key.to_string()),
        ]))
        .create();

    let params = ImageQueryParams::with_key_type(key_type)
        .resolution(resolution)
        .sub_key(sub_key);

    let _ = client.series_images_query(SERIES_ID, &params).await;

    images_query_mock.assert();
}

#[tokio::test]
async fn client_series_images_query_params() {
    let client = authenticated_test_client().await;

    let url = format!("/series/{}/images/query/params", SERIES_ID);

    let images_query_params_mock = auth_mock(&client, GET, url.as_str()).create();

    let _ = client.series_images_query_params(SERIES_ID).await;

    images_query_params_mock.assert();
}

#[tokio::test]
async fn client_episode() {
    let client = authenticated_test_client().await;

    let url = format!("/episodes/{}", EPISODE_ID);

    let episode_mock = auth_lang_mock(&client, GET, url.as_str()).create();

    let _ = client.episode(EPISODE_ID).await;

    episode_mock.assert();
}

#[tokio::test]
async fn client_languages() {
    let client = authenticated_test_client().await;

    let languages_mock = auth_mock(&client, GET, "/languages").create();

    let _ = client.languages().await;

    languages_mock.assert();
}

#[tokio::test]
async fn client_language() {
    let client = authenticated_test_client().await;

    let url = format!("/languages/{}", LANGUAGE_ID);

    let language_mock = auth_mock(&client, GET, url.as_str()).create();

    let _ = client.language(LANGUAGE_ID).await;

    language_mock.assert();
}

#[tokio::test]
async fn client_updated() {
    let client = authenticated_test_client().await;

    let (from, to) = (
        Utc::now() - Duration::days(9),
        Utc::now() - Duration::days(4),
    );

    let updated_mock = auth_lang_mock(&client, GET, "/updated/query")
        .match_query(AllOf(vec![
            UrlEncoded("fromTime".to_string(), from.timestamp().to_string()),
            UrlEncoded("toTime".to_string(), to.timestamp().to_string()),
        ]))
        .create();

    let params = UpdatedParams::with_to_time(from, to);

    let _ = client.updated(&params).await;

    updated_mock.assert();
}

#[tokio::test]
async fn client_movie() {
    let client = authenticated_test_client().await;

    let url = format!("/movies/{}", MOVIE_ID);

    let movie_mock = auth_lang_mock(&client, GET, url.as_str()).create();

    let _ = client.movie(MOVIE_ID).await;

    movie_mock.assert();
}

#[test]
fn client_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<Client>();
}

fn test_client() -> Client {
    Client {
        base_url: Url::parse(&mockito::server_url()).unwrap(),
        ..Client::create(API_KEY)
    }
}

async fn authenticated_test_client() -> Client {
    let client = test_client();

    let token = create_jwt(&TokenPayload {
        orig_iat: Utc::now(),
        exp: Utc::now() + Duration::days(1),
    });

    let login_mock = mock(POST, LOGIN_PATH)
        .with_body(serde_json::to_string(&json!({ "token": token })).unwrap())
        .create();

    client.login_set_token().await.unwrap();

    login_mock.assert();

    client
}

fn create_jwt<C>(claims: &C) -> String
where
    C: Serialize,
{
    let header = &jwt::Header::new(jwt::Algorithm::RS256);
    let key = jwt::EncodingKey::from_rsa_pem(RSA_KEY).unwrap();

    jwt::encode(header, claims, &key).unwrap()
}

/// HTTP mock with authorization header.
fn auth_mock<P>(client: &Client, method: &str, path: P) -> Mock
where
    P: Into<Matcher>,
{
    let guard = block_on(client.token.lock());
    let token = guard
        .as_ref()
        .expect("missing test client token")
        .token
        .clone();
    let bearer = format!("Bearer {}", token);

    mock(method, path).match_header("authorization", bearer.as_str())
}

/// HTTP mock with authorization and language headers.
fn auth_lang_mock<P>(client: &Client, method: &str, path: P) -> Mock
where
    P: Into<Matcher>,
{
    auth_mock(&client, method, path).match_header("accept-language", client.lang_abbr.as_str())
}

fn series_url() -> String {
    format!("/series/{}", SERIES_ID)
}
