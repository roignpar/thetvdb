use std::collections::HashMap;

use crate::response::SeriesID;

#[derive(Debug)]
pub enum SearchBy<S> {
    Name(S),
    ImdbID(S),
    Zap2itID(S),
    Slug(S),
}

// SearchBy needs to be transformed into a query param and deriving
// Serialize doesn't help:
// "top-level serializer supports only maps and structs";
//
// implementing Into instead of From because clippy incorrectly
// complains: https://github.com/rust-lang/rust-clippy/issues/3899
impl<S, B> Into<HashMap<String, String, B>> for SearchBy<S>
where
    S: Into<String>,
    B: std::hash::BuildHasher + Default,
{
    fn into(self) -> HashMap<String, String, B> {
        use SearchBy::*;

        let mut map = HashMap::default();

        match self {
            Name(name) => map.insert("name".to_string(), name.into()),
            ImdbID(id) => map.insert("imdbId".to_string(), id.into()),
            Zap2itID(id) => map.insert("zap2itId".to_string(), id.into()),
            Slug(slug) => map.insert("slug".to_string(), slug.into()),
        };

        map
    }
}

#[derive(Debug)]
pub struct EpisodeParams {
    pub(crate) series_id: SeriesID,
    pub(crate) page: u16,
}

impl EpisodeParams {
    pub fn new<I>(series_id: I) -> Self
    where
        I: Into<SeriesID>,
    {
        let series_id = series_id.into();

        Self { series_id, page: 1 }
    }

    pub fn with_page<I>(series_id: I, page: u16) -> Self
    where
        I: Into<SeriesID>,
    {
        let series_id = series_id.into();

        Self { series_id, page }
    }

    pub fn set_page(&mut self, page: u16) {
        self.page = page;
    }
}

pub trait GetEpisodeParams<'a> {
    fn series_id(&'a self) -> SeriesID;

    fn episode_params(&'a self) -> EpisodeParams {
        EpisodeParams::new(self.series_id())
    }

    fn episode_params_page(&'a self, page: u16) -> EpisodeParams {
        EpisodeParams::with_page(self.series_id(), page)
    }
}

impl<'a, T> GetEpisodeParams<'a> for T
where
    T: 'a,
    SeriesID: From<&'a T>,
{
    fn series_id(&'a self) -> SeriesID {
        SeriesID::from(self)
    }
}
