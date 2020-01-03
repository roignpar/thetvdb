use super::*;
use crate::error::test_util::*;
use crate::params;

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
        episodes: Vec::new(),
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
        episodes: Vec::new(),
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
