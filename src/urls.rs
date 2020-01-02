// This module and type exists to avoid parsing base urls
// every time a `*_url` method is called.

use lazy_static::lazy_static;
use url::Url;

use crate::error::{Error, Result};

const SERIES_BASE_URL: &str = "https://www.thetvdb.com/series/";
const BANNER_BASE_URL: &str = "https://www.thetvdb.com/banners/";
const GENRE_BASE_URL: &str = "https://www.thetvdb.com/genres/";

pub(crate) struct BaseUrls {
    pub(crate) series: Url,
    pub(crate) banner: Url,
    pub(crate) genre: Url,
}

impl BaseUrls {
    pub(crate) fn image(&self, file_name: &str) -> Result<Url> {
        // some of the image paths returned by the API start with "/banners"
        let path = trimmed(file_name.trim_start_matches("/banners"));

        Ok(self.banner.join(path)?)
    }

    pub(crate) fn opt_image(&self, file_name: &Option<String>) -> Result<Url> {
        match file_name {
            None => Err(Error::MissingImage),
            Some(f) => self.image(&f),
        }
    }

    pub(crate) fn series_website(&self, slug: &str) -> Result<Url> {
        Ok(self.series.join(trimmed(slug))?)
    }

    pub(crate) fn genre_page(&self, genre: &str) -> Result<Url> {
        Ok(self.genre.join(trimmed(genre))?)
    }
}

fn trimmed(s: &str) -> &str {
    // joining paths with starting slashes removes any
    // existing paths from the url (e.g. "/banner")
    s.trim_start_matches('/')
}

lazy_static! {
    pub(crate) static ref URLS: BaseUrls = BaseUrls {
        series: Url::parse(SERIES_BASE_URL).expect("could not parse series base url"),
        banner: Url::parse(BANNER_BASE_URL).expect("could not parse banner base url"),
        genre: Url::parse(GENRE_BASE_URL).expect("could not parse genres base url"),
    };
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
