use serde::de::DeserializeOwned;
use serde_json as json;

use super::*;
use crate::params;
use crate::test_util::*;

const BANNER: &str = "path/to/banner.jpg";
const POSTER: &str = "path/to/poster.png";
const FANART: &str = "path/to/fanart.gif";
const THUMB: &str = "path/to/thumbnail.jpeg";
const SLUG: &str = "series-name";
const GENRE: &str = "sci-fi";

#[test]
fn search_series_urls() -> Result<()> {
    let ss = SearchSeries {
        banner: Some(BANNER.to_string()),
        slug: SLUG.to_string(),

        ..Default::default()
    };

    assert_eq!(ss.banner_url()?, URLS.banner.join(BANNER)?);
    assert_eq!(ss.website_url()?, URLS.series.join(SLUG)?);

    Ok(())
}

#[test]
fn search_series_urls_errors() {
    let ss = SearchSeries::default();

    assert_missing_image_err(ss.banner_url());
}

#[test]
fn series_urls() -> Result<()> {
    let s = Series {
        banner: Some(BANNER.to_string()),
        poster: Some(POSTER.to_string()),
        fanart: Some(FANART.to_string()),
        slug: SLUG.to_string(),

        ..Default::default()
    };

    assert_eq!(s.banner_url()?, URLS.banner.join(BANNER)?);
    assert_eq!(s.poster_url()?, URLS.banner.join(POSTER)?);
    assert_eq!(s.fanart_url()?, URLS.banner.join(FANART)?);
    assert_eq!(s.website_url()?, URLS.series.join(SLUG)?);

    Ok(())
}

#[test]
fn series_urls_errors() {
    let s = Series::default();

    let urls = vec![s.banner_url(), s.poster_url(), s.fanart_url()];

    for url in urls {
        assert_missing_image_err(url);
    }
}

#[test]
fn filtered_series_urls() -> Result<()> {
    let fs = FilteredSeries {
        banner: Some(BANNER.to_string()),
        poster: Some(POSTER.to_string()),
        fanart: Some(FANART.to_string()),
        slug: Some(SLUG.to_string()),

        ..Default::default()
    };

    assert_eq!(fs.banner_url()?, URLS.banner.join(BANNER)?);
    assert_eq!(fs.poster_url()?, URLS.banner.join(POSTER)?);
    assert_eq!(fs.fanart_url()?, URLS.banner.join(FANART)?);
    assert_eq!(fs.website_url()?, URLS.series.join(SLUG)?);

    Ok(())
}

#[test]
fn filtered_series_urls_errors() {
    let fs = FilteredSeries::default();

    let urls = vec![fs.banner_url(), fs.poster_url(), fs.fanart_url()];

    for url in urls {
        assert_missing_image_err(url);
    }

    match fs.website_url().unwrap_err() {
        Error::MissingSeriesSlug => {}
        e => wrong_error_kind(Error::MissingSeriesSlug, e),
    }
}

#[test]
fn actor_urls() -> Result<()> {
    let a = Actor {
        image: Some(BANNER.to_string()),

        ..Default::default()
    };

    assert_eq!(a.image_url()?, URLS.banner.join(BANNER)?);

    Ok(())
}

#[test]
fn acrot_urls_errors() {
    let a = Actor::default();

    assert_missing_image_err(a.image_url());
}

#[test]
fn episode_urls() -> Result<()> {
    let e = Episode {
        filename: Some(BANNER.to_string()),

        ..Default::default()
    };

    assert_eq!(e.filename_url()?, URLS.banner.join(BANNER)?);

    Ok(())
}

#[test]
fn episode_urls_errors() {
    let e = Episode::default();

    assert_missing_image_err(e.filename_url());
}

#[test]
fn page_links_current_page() {
    let mut pl = PageLinks {
        first: 1,
        last: 20,
        next: Some(10),
        prev: Some(8),
    };

    assert_eq!(pl.current_page(), 9);

    pl = PageLinks {
        next: Some(2),
        prev: None,
        ..pl
    };

    assert_eq!(pl.current_page(), 1);

    pl = PageLinks {
        next: None,
        prev: Some(19),
        ..pl
    };

    assert_eq!(pl.current_page(), 20);

    pl = PageLinks {
        first: 1,
        last: 1,
        next: None,
        prev: None,
    };

    assert_eq!(pl.current_page(), 1);
}

#[test]
fn episode_page_params_generation() {
    let sid = SeriesID(123);

    let mut ep = EpisodePage {
        episodes: vec![Episode::default()],
        series_id: sid,
        links: PageLinks {
            first: 1,
            last: 10,
            next: Some(5),
            prev: Some(3),
        },
    };

    check_episode_page_params(ep.next_page_params().unwrap(), sid, 5);
    check_episode_page_params(ep.prev_page_params().unwrap(), sid, 3);
    check_episode_page_params(ep.first_page_params(), sid, 1);
    check_episode_page_params(ep.last_page_params(), sid, 10);

    ep.links = PageLinks {
        first: 1,
        last: 1,
        next: None,
        prev: None,
    };

    assert_eq!(ep.next_page_params(), None);
    assert_eq!(ep.prev_page_params(), None);
}

#[test]
fn episode_query_page_params_generation() {
    let sid = SeriesID(321);

    let query = params::EpisodeQuery {
        absolute_number: Some(11),
        aired_season: Some(12),
        aired_episode: Some(13),
        dvd_season: Some(14),
        dvd_episode: Some(15),
        imdb_id: Some("imdb".to_string()),
    };

    let mut eqp = EpisodeQueryPage {
        episodes: vec![Episode::default()],
        series_id: sid,
        query: query.clone(),
        links: PageLinks {
            first: 1,
            last: 15,
            next: Some(3),
            prev: Some(1),
        },
    };

    check_episode_query_page_params(eqp.next_page_query_params().unwrap(), sid, 3, &query);
    check_episode_query_page_params(eqp.prev_page_query_params().unwrap(), sid, 1, &query);
    check_episode_query_page_params(eqp.first_page_query_params(), sid, 1, &query);
    check_episode_query_page_params(eqp.last_page_query_params(), sid, 15, &query);

    eqp.links = PageLinks {
        first: 1,
        last: 1,
        next: None,
        prev: None,
    };

    assert_eq!(eqp.next_page_query_params(), None);
    assert_eq!(eqp.prev_page_query_params(), None);
}

#[test]
fn image_urls() -> Result<()> {
    let i = Image {
        file_name: BANNER.to_string(),
        thumbnail: THUMB.to_string(),

        ..Default::default()
    };

    assert_eq!(i.file_name_url()?, URLS.banner.join(BANNER)?);
    assert_eq!(i.thumbnail_url()?, URLS.banner.join(THUMB)?);

    Ok(())
}

#[test]
fn genre_url() -> Result<()> {
    let g = Genre {
        url: GENRE.to_string(),

        ..Default::default()
    };

    assert_eq!(g.full_url()?, URLS.genre.join(GENRE)?);

    Ok(())
}

#[test]
fn artwork_urls() -> Result<()> {
    let a = Artwork {
        url: BANNER.to_string(),
        thumb_url: THUMB.to_string(),

        ..Default::default()
    };

    assert_eq!(a.full_url()?, URLS.banner.join(BANNER)?);
    assert_eq!(a.full_thumb_url()?, URLS.banner.join(THUMB)?);

    Ok(())
}

#[test]
fn person_urls() -> Result<()> {
    let p = Person {
        people_image: Some(BANNER.to_string()),
        role_image: Some(POSTER.to_string()),

        ..Default::default()
    };

    assert_eq!(p.people_image_url()?, URLS.banner.join(BANNER)?);
    assert_eq!(p.role_image_url()?, URLS.banner.join(POSTER)?);

    Ok(())
}

#[test]
fn person_urls_errors() {
    let p = Person::default();

    let urls = vec![p.people_image_url(), p.role_image_url()];

    for url in urls {
        assert_missing_image_err(url);
    }
}

#[test]
fn types_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<SeriesID>();
    assert_send_sync::<EpisodeID>();
    assert_send_sync::<SearchSeries>();
    assert_send_sync::<Series>();
    assert_send_sync::<FilteredSeries>();
    assert_send_sync::<SeriesStatus>();
    assert_send_sync::<Actor>();
    assert_send_sync::<Episode>();
    assert_send_sync::<EpisodeLanguage>();
    assert_send_sync::<EpisodePage>();
    assert_send_sync::<EpisodeQueryPage>();
    assert_send_sync::<PageLinks>();
    assert_send_sync::<EpisodeSummary>();
    assert_send_sync::<SeriesImages>();
    assert_send_sync::<Image>();
    assert_send_sync::<ImageRatingsInfo>();
    assert_send_sync::<ImageQueryKey>();
    assert_send_sync::<SeriesUpdate>();
    assert_send_sync::<MovieID>();
    assert_send_sync::<Movie>();
    assert_send_sync::<Genre>();
    assert_send_sync::<Translation>();
    assert_send_sync::<ReleaseDate>();
    assert_send_sync::<Artwork>();
    assert_send_sync::<Trailer>();
    assert_send_sync::<RemoteID>();
    assert_send_sync::<People>();
    assert_send_sync::<Person>();
    assert_send_sync::<MovieUpdates>();
}

#[test]
fn serde() -> Result<()> {
    let ss = SearchSeries {
        aliases: vec!["test".to_string(), "testing".to_string()],
        banner: Some("banner".to_string()),
        first_aired: Some(NaiveDate::from_ymd(2000, 10, 30)),
        id: SeriesID(10),
        network: Some("Test".to_string()),
        overview: Some("test".to_string()),
        series_name: Some("Testing".to_string()),
        slug: "test".to_string(),
        status: SeriesStatus::Ended,
    };
    let dss = SearchSeries::default();

    for search_series in &[ss, dss] {
        assert_ser_deser(search_series);
    }

    let s = Series {
        added: Some(now_round_seconds()),
        added_by: Some(10),
        airs_day_of_week: Some("Monday".to_string()),
        airs_time: Some(NaiveTime::from_hms(8, 30, 0)),
        aliases: vec!["test".to_string(), "testing".to_string()],
        season: "2".to_string(),
        banner: Some("banner".to_string()),
        poster: Some("poster".to_string()),
        fanart: Some("fanart".to_string()),
        first_aired: Some(NaiveDate::from_ymd(2000, 7, 21)),
        genre: vec!["Drama".to_string(), "Comedy".to_string()],
        id: SeriesID(20),
        imdb_id: Some("tttest".to_string()),
        last_updated: Some(now_round_seconds()),
        network: Some("Test".to_string()),
        network_id: Some("test".to_string()),
        overview: Some("testing".to_string()),
        rating: Some("TV-TEST".to_string()),
        runtime: "50".to_string(),
        language: "en".to_string(),
        series_name: Some("Test".to_string()),
        site_rating: Some(8.0),
        site_rating_count: 1234,
        slug: "test".to_string(),
        status: SeriesStatus::Continuing,
        zap2it_id: Some("1234567".to_string()),
    };
    let ds = Series::default();

    for series in &[s, ds] {
        assert_ser_deser(series);
    }

    let fs = FilteredSeries {
        added: Some(now_round_seconds()),
        added_by: Some(21),
        airs_day_of_week: Some("Friday".to_string()),
        airs_time: Some(NaiveTime::from_hms(9, 10, 00)),
        aliases: Some(vec!["filtered".to_string(), "test".to_string()]),
        season: Some("3".to_string()),
        banner: Some("filtered_banner".to_string()),
        poster: Some("filtered_poster".to_string()),
        fanart: Some("filtered_fanart".to_string()),
        first_aired: Some(NaiveDate::from_ymd(2004, 1, 6)),
        genre: Some(vec!["Thriller".to_string()]),
        id: Some(SeriesID(321451)),
        imdb_id: Some("testtt".to_string()),
        last_updated: Some(now_round_seconds()),
        network: Some("tesT".to_string()),
        network_id: Some("tesst".to_string()),
        overview: Some("test".to_string()),
        rating: Some("TEST-TV".to_string()),
        runtime: Some("123".to_string()),
        language: Some("en".to_string()),
        series_name: Some("Tester".to_string()),
        site_rating: Some(7.32),
        site_rating_count: Some(123),
        slug: Some("teslug".to_string()),
        status: Some(SeriesStatus::Ended),
        zap2it_id: Some("7654321".to_string()),
    };
    let dfs = FilteredSeries::default();

    for filtered_series in &[fs, dfs] {
        assert_ser_deser(filtered_series);
    }

    let a = Actor {
        id: 123,
        series_id: SeriesID(321),
        name: "Famous Actor".to_string(),
        role: "Bob".to_string(),
        sort_order: 3,
        image: Some("portrait".to_string()),
        image_author: Some(1234),
        image_added: Some(now_round_seconds()),
        last_updated: Some(now_round_seconds()),
    };
    let da = Actor::default();

    for actor in &[a, da] {
        assert_ser_deser(actor);
    }

    let e = Episode {
        id: EpisodeID(1),
        aired_season: Some(2),
        aired_season_id: Some(3),
        aired_episode_number: 4,
        episode_name: Some("Test".to_string()),
        first_aired: Some(NaiveDate::from_ymd(2010, 3, 12)),
        guest_stars: vec!["Sirius".to_string(), "Pollux".to_string()],
        directors: vec!["Director Test".to_string()],
        writers: vec!["Ms. Writer".to_string()],
        overview: Some("test episode".to_string()),
        language: EpisodeLanguage {
            episode_name: "en".to_string(),
            overview: "fr".to_string(),
        },
        production_code: Some("TEST".to_string()),
        show_url: Some("test_url".to_string()),
        last_updated: Some(now_round_seconds()),
        dvd_discid: Some("DVD".to_string()),
        dvd_season: Some(2),
        dvd_episode_number: Some(3),
        dvd_chapter: Some(5),
        absolute_number: Some(43),
        filename: Some("test_file".to_string()),
        series_id: SeriesID(32),
        last_updated_by: Some(432),
        airs_after_season: Some(12),
        airs_before_season: Some(10),
        airs_before_episode: Some(6),
        thumb_author: Some(90),
        thumb_added: Some(now_round_seconds()),
        thumb_width: Some("320".to_string()),
        thumb_height: Some("430".to_string()),
        imdb_id: Some("ttttttt".to_string()),
        content_rating: Some("TEST".to_string()),
        site_rating: Some(5.4),
        site_rating_count: 324,
        is_movie: true,
    };
    let de = Episode::default();

    for episode in &[e, de] {
        assert_ser_deser(episode);
    }

    let es = EpisodeSummary {
        aired_seasons: vec!["1".to_string(), "3".to_string()],
        aired_episodes: 12,
        dvd_seasons: vec!["3".to_string(), "2".to_string()],
        dvd_episodes: 13,
    };

    assert_ser_deser(&es);

    let si = SeriesImages {
        fanart: Some(12),
        poster: Some(13),
        season: Some(14),
        seasonwide: Some(15),
        series: Some(16),
    };

    assert_ser_deser(&si);

    let i = Image {
        id: 789,
        key_type: "test".to_string(),
        sub_key: Some("testing".to_string()),
        file_name: "test_file".to_string(),
        language_id: 3,
        language: "es".to_string(),
        resolution: Some("321x531".to_string()),
        ratings_info: ImageRatingsInfo {
            average: 8.12,
            count: 12,
        },
        thumbnail: "test_thumb".to_string(),
    };

    assert_ser_deser(&i);

    let iqk = ImageQueryKey {
        key_type: "test_key".to_string(),
        language_id: Some("t".to_string()),
        resolution: vec!["100x200".to_string(), "300x600".to_string()],
        sub_key: vec!["t1".to_string(), "t2".to_string()],
    };

    assert_ser_deser(&iqk);

    let su = SeriesUpdate {
        id: SeriesID(12345),
        last_updated: now_round_seconds(),
    };

    assert_ser_deser(&su);

    let m = Movie {
        id: MovieID(412),
        url: "movie_url".to_string(),
        runtime: 121,
        genres: vec![Genre {
            url: "test".to_string(),
            name: "Tester".to_string(),
            id: 90,
        }],
        translations: vec![Translation {
            language_code: "ts".to_string(),
            name: "Test Movie".to_string(),
            overview: Some("test overview".to_string()),
            is_primary: true,
            tagline: Some("testline".to_string()),
        }],
        release_dates: vec![ReleaseDate {
            kind: "testing".to_string(),
            date: NaiveDate::from_ymd(2011, 11, 1),
            country: "Testlandia".to_string(),
        }],
        artworks: vec![Artwork {
            id: "TEST123".to_string(),
            artwork_type: "test".to_string(),
            url: "aw_url".to_string(),
            thumb_url: "aw_thumb".to_string(),
            tags: Some("test,testing,testish".to_string()),
            is_primary: false,
            width: 1000,
            height: 800,
        }],
        trailers: vec![Trailer {
            url: "trailer_url".to_string(),
            name: "Test Trailer".to_string(),
        }],
        remoteids: vec![RemoteID {
            id: "testid".to_string(),
            source_id: 931,
            source_name: "Test".to_string(),
            url: "test.test/testid".to_string(),
        }],
        people: People {
            actors: vec![Person {
                id: "teston".to_string(),
                name: "Tester Testerson".to_string(),
                role: Some("Testara".to_string()),
                people_image: Some("test_face".to_string()),
                role_image: Some("testara_face".to_string()),
                is_featured: true,
                people_id: "TEST".to_string(),
                imdb_id: Some("tttesterson".to_string()),
                people_twitter: Some("twitest".to_string()),
                people_facebook: Some("testbook".to_string()),
                people_instagram: Some("testagram".to_string()),
            }],
            directors: vec![],
            producers: vec![],
            writers: vec![],
        },
    };

    assert_ser_deser(&m);

    Ok(())
}

fn check_episode_page_params(params: EpisodeParams, id: SeriesID, page: u16) {
    assert_eq!(params.series_id, id);
    assert_eq!(params.page, page);
}

fn check_episode_query_page_params(
    query_params: EpisodeQueryParams,
    id: SeriesID,
    page: u16,
    query: &EpisodeQuery,
) {
    assert_eq!(query_params.params.series_id, id);
    assert_eq!(query_params.params.page, page);
    assert_eq!(query_params.query, *query);
}

fn assert_missing_image_err<T>(result: Result<T>)
where
    T: std::fmt::Debug,
{
    match result.unwrap_err() {
        Error::MissingImage => {}
        e => wrong_error_kind(Error::MissingImage, e),
    }
}

fn ser_deser<T>(t: &T) -> Result<T>
where
    T: DeserializeOwned + Serialize,
{
    Ok(json::from_slice(&json::to_vec(t)?)?)
}

fn assert_ser_deser<T>(t: &T)
where
    T: DeserializeOwned + Serialize + std::fmt::Debug + PartialEq,
{
    use std::any::type_name;

    assert_eq!(
        t,
        &ser_deser(t).expect(&format!("failed to ser-deser {}", type_name::<T>()))
    );
}
