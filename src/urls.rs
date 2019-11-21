// This module and type exists to avoid parsing base urls
// every time a `*_url` method is called.

use lazy_static::lazy_static;
use url::Url;

pub(crate) struct BaseUrls {
    pub(crate) series: Url,
    pub(crate) banner: Url,
}

lazy_static! {
    pub(crate) static ref URLS: BaseUrls = BaseUrls {
        series: Url::parse("https://www.thetvdb.com/series/")
            .expect("could not parse series base url"),
        banner: Url::parse("https://www.thetvdb.com/banners/")
            .expect("could not parse banner base url")
    };
}
