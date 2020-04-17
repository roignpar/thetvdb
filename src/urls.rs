// This module and type exists to avoid parsing base urls
// every time a `*_url` method is called.

use lazy_static::lazy_static;
use url::Url;

use crate::error::{Error, Result};

const SERIES_BASE_URL: &str = "https://www.thetvdb.com/series/";
const BANNER_BASE_URL: &str = "https://www.thetvdb.com/banners/";
const GENRE_BASE_URL: &str = "https://www.thetvdb.com/genres/";

lazy_static! {
    pub(crate) static ref SERIES: Url =
        Url::parse(SERIES_BASE_URL).expect("Could not parse series base URL");
    pub(crate) static ref BANNER: Url =
        Url::parse(BANNER_BASE_URL).expect("Could not parse banner base URL");
    pub(crate) static ref GENRE: Url =
        Url::parse(GENRE_BASE_URL).expect("Could not parse genre base URL");
}

pub(crate) fn image(file_name: &str) -> Result<Url> {
    // some of the image paths returned by the API start with "/banners"
    let path = trimmed(file_name.trim_start_matches("/banners"));

    Ok(BANNER.join(path)?)
}

pub(crate) fn opt_image(file_name: &Option<String>) -> Result<Url> {
    match file_name {
        None => Err(Error::MissingImage),
        Some(f) => image(&f),
    }
}

pub(crate) fn series_website(slug: &str) -> Result<Url> {
    Ok(SERIES.join(trimmed(slug))?)
}

pub(crate) fn genre_page(genre_name: &str) -> Result<Url> {
    Ok(GENRE.join(trimmed(genre_name))?)
}

fn trimmed(s: &str) -> &str {
    // joining paths with starting slashes removes any
    // existing paths from the url (e.g. "/banner")
    s.trim_start_matches('/')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_parse() {
        Url::parse(SERIES_BASE_URL).unwrap();
        Url::parse(BANNER_BASE_URL).unwrap();
        Url::parse(GENRE_BASE_URL).unwrap();
    }
}
