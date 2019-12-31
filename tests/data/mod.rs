use chrono::{Date, TimeZone, Utc};
use lazy_static::lazy_static;

use thetvdb::{language::*, response::*};

#[derive(Debug)]
pub struct TestSeries {
    pub id: SeriesID,
    pub series_name: String,
    pub first_aired: Date<Utc>,
    pub network: String,
    pub slug: String,
    pub status: SeriesStatus,
    pub imdb_id: String,
    pub zap2it_id: String,
}

#[derive(Debug)]
pub struct TestActor {
    pub id: u32,
    pub series_id: SeriesID,
    pub name: String,
    pub role: String,
}

#[derive(Debug)]
pub struct TestEpisode {
    pub id: EpisodeID,
    pub aired_season: u16,
    pub aired_season_id: u32,
    pub aired_episode_number: u16,
    pub episode_name: String,
    pub first_aired: Date<Utc>,
    pub absolute_number: u16,
    pub series_id: SeriesID,
    pub imdb_id: String,
}

#[derive(Debug)]
pub struct TestLanguage {
    pub id: LanguageID,
    pub abbreviation: String,
    pub name: String,
    pub english_name: String,
}

// All statics and constants refer to the series Planet Earth II.
lazy_static! {
    pub static ref PEII: TestSeries = TestSeries {
        id: SeriesID(318408),
        series_name: "Planet Earth II".to_string(),
        first_aired: Utc.ymd(2016, 11, 6),
        network: "BBC One".to_string(),
        slug: "planet-earth-ii".to_string(),
        status: SeriesStatus::Ended,
        imdb_id: "tt5491994".to_string(),
        zap2it_id: "EP02575095".to_string()
    };
    pub static ref NARRATOR: TestActor = TestActor {
        id: 64183294,
        series_id: PEII.id,
        name: "David Attenborough".to_string(),
        role: "Narrator".to_string(),
    };
    pub static ref ISLANDS: TestEpisode = TestEpisode {
        id: EpisodeID(5812389),
        aired_season: 1,
        aired_season_id: 684701,
        aired_episode_number: 1,
        episode_name: "Islands".to_string(),
        first_aired: Utc.ymd(2016, 11, 6),
        absolute_number: 1,
        series_id: PEII.id,
        imdb_id: "tt5491994".to_string()
    };
    pub static ref EPISODE_SUMMARY: EpisodeSummary = EpisodeSummary {
        aired_seasons: vec!["1".to_string(), "0".to_string()],
        aired_episodes: 18,
        dvd_seasons: vec!["1".to_string(), "0".to_string()],
        dvd_episodes: 7,
    };
    pub static ref SAMOAN: TestLanguage = TestLanguage {
        id: LanguageID(225),
        abbreviation: "sm".to_string(),
        name: "gagana fa'a Samoa".to_string(),
        english_name: "Samoan".to_string(),
    };
}

pub const EPISODE_COUNT: usize = 18;
pub const UPDATED_FROM: i64 = 1575108793;
pub const UPDATED_TO: i64 = 1575109793;
// Number of updated series between UPDATED_FROM and UPDATED_TO
pub const UPDATED_COUNT: usize = 6;

impl PartialEq<SearchSeries> for TestSeries {
    fn eq(&self, ss: &SearchSeries) -> bool {
        self.id == ss.id
            && &self.series_name == ss.series_name.as_ref().unwrap()
            && self.first_aired == ss.first_aired.unwrap()
            && &self.network == ss.network.as_ref().unwrap()
            && self.slug == ss.slug
            && self.status == ss.status
    }
}

impl PartialEq<TestSeries> for SearchSeries {
    fn eq(&self, ts: &TestSeries) -> bool {
        ts == self
    }
}

impl PartialEq<Series> for TestSeries {
    fn eq(&self, s: &Series) -> bool {
        self.id == s.id
            && &self.series_name == s.series_name.as_ref().unwrap()
            && self.first_aired == s.first_aired.unwrap()
            && &self.network == s.network.as_ref().unwrap()
            && self.slug == s.slug
            && self.status == s.status
            && &self.imdb_id == s.imdb_id.as_ref().unwrap()
    }
}

impl PartialEq<TestSeries> for Series {
    fn eq(&self, ts: &TestSeries) -> bool {
        ts == self
    }
}

impl PartialEq<FilteredSeries> for TestSeries {
    fn eq(&self, s: &FilteredSeries) -> bool {
        self.id == s.id.unwrap()
            && &self.series_name == s.series_name.as_ref().unwrap()
            && self.first_aired == s.first_aired.unwrap()
            && &self.network == s.network.as_ref().unwrap()
            && &self.slug == s.slug.as_ref().unwrap()
            && &self.status == s.status.as_ref().unwrap()
            && &self.imdb_id == s.imdb_id.as_ref().unwrap()
    }
}

impl PartialEq<TestSeries> for FilteredSeries {
    fn eq(&self, ts: &TestSeries) -> bool {
        ts == self
    }
}

impl PartialEq<Actor> for TestActor {
    fn eq(&self, a: &Actor) -> bool {
        self.id == a.id
            && self.series_id == a.series_id
            && self.name == a.name
            && self.role == a.role
    }
}

impl PartialEq<TestActor> for Actor {
    fn eq(&self, ta: &TestActor) -> bool {
        ta == self
    }
}

impl PartialEq<Episode> for TestEpisode {
    fn eq(&self, e: &Episode) -> bool {
        self.id == e.id
            && self.aired_season == e.aired_season.unwrap()
            && self.aired_season_id == e.aired_season_id.unwrap()
            && self.aired_episode_number == e.aired_episode_number
            && &self.episode_name == e.episode_name.as_ref().unwrap()
            && self.first_aired == e.first_aired.unwrap()
            && self.absolute_number == e.absolute_number.unwrap()
            && self.series_id == e.series_id
            && &self.imdb_id == e.imdb_id.as_ref().unwrap()
    }
}

impl PartialEq<TestEpisode> for Episode {
    fn eq(&self, te: &TestEpisode) -> bool {
        te == self
    }
}

impl PartialEq<Language> for TestLanguage {
    fn eq(&self, l: &Language) -> bool {
        self.id == l.id
            && self.abbreviation == l.abbr()
            && self.name == l.name
            && self.english_name == l.english_name
    }
}

impl PartialEq<TestLanguage> for Language {
    fn eq(&self, tl: &TestLanguage) -> bool {
        tl == self
    }
}
