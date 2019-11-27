// This module and type exists to avoid parsing base urls
// every time a `*_url` method is called.

use lazy_static::lazy_static;
use url::Url;

const SERIES_BASE_URL: &str = "https://www.thetvdb.com/series/";
const BANNER_BASE_URL: &str = "https://www.thetvdb.com/banners/";

pub(crate) struct BaseUrls {
    pub(crate) series: Url,
    pub(crate) banner: Url,
}

lazy_static! {
    pub(crate) static ref URLS: BaseUrls = BaseUrls {
        series: Url::parse(SERIES_BASE_URL).expect("could not parse series base url"),
        banner: Url::parse(BANNER_BASE_URL).expect("could not parse banner base url")
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_parse() {
        Url::parse(SERIES_BASE_URL).unwrap();
        Url::parse(BANNER_BASE_URL).unwrap();
    }
}
