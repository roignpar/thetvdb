use chrono::NaiveDate;
use lazy_static::lazy_static;

use thetvdb::{language::*, response::*};

#[derive(Debug)]
pub struct TestSeries {
    pub id: SeriesID,
    pub series_name: String,
    pub first_aired: NaiveDate,
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
    pub first_aired: NaiveDate,
    pub absolute_number: u16,
    pub series_id: SeriesID,
    pub imdb_id: String,
}

#[derive(Debug)]
pub struct TestEpisodeSummary {
    pub aired_seasons: Vec<String>,
    pub aired_episodes: u16,
    pub dvd_seasons: Vec<String>,
    pub dvd_episodes: u16,
}

#[derive(Debug)]
pub struct TestLanguage {
    pub id: LanguageID,
    pub abbreviation: String,
    pub name: String,
    pub english_name: String,
}

#[derive(Debug)]
pub struct TestMovie {
    pub id: MovieID,
    pub url: String,
    pub runtime: u16,
}

#[derive(Debug)]
pub struct TestGenre {
    pub url: String,
    pub name: String,
    pub id: u16,
}

#[derive(Debug)]
pub struct TestTranslation {
    pub language_code: String,
    pub name: String,
    pub is_primary: bool,
}

#[derive(Debug)]
pub struct TestReleaseDate {
    pub kind: String,
    pub date: NaiveDate,
    pub country: String,
}

#[derive(Debug)]
pub struct TestRemoteID {
    pub id: String,
    pub source_id: u32,
    pub source_name: String,
    pub url: String,
}

#[derive(Debug)]
pub struct TestPerson {
    pub id: String,
    pub name: String,
    pub role: Option<String>,
    pub is_featured: bool,
    pub people_id: String,
}

// All series statics and constants refer to the series Planet Earth II.
// All movie statics and constants refer to the movie The Shawshank Redemption.
lazy_static! {
    pub static ref PEII: TestSeries = TestSeries {
        id: SeriesID(318408),
        series_name: "Planet Earth II".to_string(),
        first_aired: NaiveDate::from_ymd(2016, 11, 6),
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
        first_aired: NaiveDate::from_ymd(2016, 11, 6),
        absolute_number: 1,
        series_id: PEII.id,
        imdb_id: "tt5491994".to_string()
    };
    pub static ref EPISODE_SUMMARY: TestEpisodeSummary = TestEpisodeSummary {
        aired_seasons: vec!["0".to_string(), "1".to_string()],
        aired_episodes: 18,
        dvd_seasons: vec!["1".to_string()],
        dvd_episodes: 6,
    };
    pub static ref SAMOAN: TestLanguage = TestLanguage {
        id: LanguageID(225),
        abbreviation: "sm".to_string(),
        name: "gagana fa'a Samoa".to_string(),
        english_name: "Samoan".to_string(),
    };
    pub static ref TSR: TestMovie = TestMovie {
        id: MovieID(190),
        url: "https://api.thetvdb.com/the-shawshank-redemption".to_string(),
        runtime: 142,
    };
    pub static ref DRAMA: TestGenre = TestGenre {
        id: 12,
        url: "drama".to_string(),
        name: "Drama".to_string(),
    };
    pub static ref TSR_ENG: TestTranslation = TestTranslation {
        language_code: "eng".to_string(),
        name: "The Shawshank Redemption".to_string(),
        is_primary: true,
    };
    pub static ref RELEASE: TestReleaseDate = TestReleaseDate {
        kind: "release_date".to_string(),
        date: NaiveDate::from_ymd(1995, 2, 9),
        country: "global".to_string()
    };
    pub static ref TSR_IMDB: TestRemoteID = TestRemoteID {
        id: "tt0111161".to_string(),
        source_id: 2,
        source_name: "IMDB".to_string(),
        url: "http://www.imdb.com/title/tt0111161".to_string(),
    };
    pub static ref ANDY: TestPerson = TestPerson {
        id: "12142594".to_string(),
        name: "Tim Robbins".to_string(),
        role: Some("Andy Dufresne".to_string()),
        is_featured: false,
        people_id: "293587".to_string(),
    };
}

pub const EPISODE_COUNT: usize = 18;

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

impl PartialEq<EpisodeSummary> for TestEpisodeSummary {
    fn eq(&self, e: &EpisodeSummary) -> bool {
        for season in &e.aired_seasons {
            if !self.aired_seasons.contains(season) {
                return false;
            }
        }

        for season in &e.dvd_seasons {
            if !self.dvd_seasons.contains(season) {
                return false;
            }
        }

        self.aired_seasons.len() == e.aired_seasons.len()
            && self.aired_episodes == e.aired_episodes
            && self.dvd_seasons.len() == e.dvd_seasons.len()
            && self.dvd_episodes == e.dvd_episodes
    }
}

impl PartialEq<TestEpisodeSummary> for EpisodeSummary {
    fn eq(&self, te: &TestEpisodeSummary) -> bool {
        te == self
    }
}

impl PartialEq<Language> for TestLanguage {
    fn eq(&self, l: &Language) -> bool {
        self.id == l.id
            && self.abbreviation == l.abbreviation
            && self.name == l.name
            && self.english_name == l.english_name
    }
}

impl PartialEq<TestLanguage> for Language {
    fn eq(&self, tl: &TestLanguage) -> bool {
        tl == self
    }
}

impl PartialEq<Movie> for TestMovie {
    fn eq(&self, m: &Movie) -> bool {
        self.id == m.id && self.url == m.url && self.runtime == m.runtime
    }
}

impl PartialEq<TestMovie> for Movie {
    fn eq(&self, tm: &TestMovie) -> bool {
        tm == self
    }
}

impl PartialEq<Genre> for TestGenre {
    fn eq(&self, g: &Genre) -> bool {
        self.id == g.id && self.name == g.name && self.url == g.url
    }
}

impl PartialEq<TestGenre> for Genre {
    fn eq(&self, tg: &TestGenre) -> bool {
        tg == self
    }
}

impl PartialEq<Translation> for TestTranslation {
    fn eq(&self, t: &Translation) -> bool {
        self.language_code == t.language_code
            && self.name == t.name
            && self.is_primary == t.is_primary
    }
}

impl PartialEq<TestTranslation> for Translation {
    fn eq(&self, tt: &TestTranslation) -> bool {
        tt == self
    }
}

impl PartialEq<ReleaseDate> for TestReleaseDate {
    fn eq(&self, r: &ReleaseDate) -> bool {
        self.kind == r.kind && self.date == r.date && self.country == r.country
    }
}

impl PartialEq<TestReleaseDate> for ReleaseDate {
    fn eq(&self, tr: &TestReleaseDate) -> bool {
        tr == self
    }
}

impl PartialEq<RemoteID> for TestRemoteID {
    fn eq(&self, r: &RemoteID) -> bool {
        self.id == r.id
            && self.source_id == r.source_id
            && self.source_name == r.source_name
            && self.url == r.url
    }
}

impl PartialEq<TestRemoteID> for RemoteID {
    fn eq(&self, tr: &TestRemoteID) -> bool {
        tr == self
    }
}

impl PartialEq<Person> for TestPerson {
    fn eq(&self, p: &Person) -> bool {
        self.id == p.id
            && self.name == p.name
            && self.role == p.role
            && self.is_featured == p.is_featured
            && self.people_id == p.people_id
    }
}

impl PartialEq<TestPerson> for Person {
    fn eq(&self, tp: &TestPerson) -> bool {
        tp == self
    }
}
