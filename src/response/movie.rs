#![deny(missing_docs, missing_debug_implementations, unsafe_code)]

use std::fmt;

use chrono::NaiveDate;
use serde::Deserialize;
use url::Url;

use crate::deserialize;
use crate::error::Result;
use crate::urls::URLS;

/// Custom type used for [`Movie`](./struct.Movie.html) ids.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, PartialOrd, Ord, Eq, Deserialize)]
pub struct MovieID(pub u32);

impl fmt::Display for MovieID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for MovieID {
    fn from(i: u32) -> Self {
        MovieID(i)
    }
}

/// Movie data returned by
/// [`Client.movie`](../client/struct.Client.html#method.movie).
#[derive(Debug, Deserialize)]
pub struct Movie {
    /// ID of the movie.
    pub id: MovieID,
    /// Movie URL.
    pub url: String,
    /// Movie runtime, in minutes.
    pub runtime: u16,
    /// Movie genres.
    pub genres: Vec<Genre>,
    /// Movie translations.
    pub translations: Vec<Translation>,
    /// Movie release dates.
    pub release_dates: Vec<ReleaseDate>,
    /// Movie artworks.
    pub artworks: Vec<Artwork>,
    /// Movie trailers.
    pub trailers: Vec<Trailer>,
    /// Movie IDs on other websites.
    pub remoteids: Vec<RemoteID>,
    /// Movie people (actors, writers, directors, producers).
    pub people: People,
}

/// Movie genre data.
#[derive(Debug, Default, Deserialize)]
pub struct Genre {
    /// Genre path.
    ///
    /// For the full URL use [`full_url`](#method.full_url).
    pub url: String,
    /// Genre name.
    pub name: String,
    /// Genre ID.
    pub id: u16,
}

impl Genre {
    /// Returns the full URL of the genre page.
    pub fn full_url(&self) -> Result<Url> {
        URLS.genre_page(&self.url)
    }
}

/// Movie translation data.
#[derive(Debug, Deserialize)]
pub struct Translation {
    /// Translation language code.
    pub language_code: String,
    /// Movie name in this language.
    pub name: String,
    /// Movie overview in this language.
    #[serde(deserialize_with = "deserialize::optional_string")]
    pub overview: Option<String>,
    /// Whether this is the movie's primary translation.
    pub is_primary: bool,
    /// Movie tagline in this language.
    #[serde(deserialize_with = "deserialize::optional_string")]
    pub tagline: Option<String>,
}

/// Movie release date data.
#[derive(Debug, Deserialize)]
pub struct ReleaseDate {
    /// Type of release date.
    ///
    /// In the API this field is named `type`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The release date.
    pub date: NaiveDate,
    /// Country or location release date applies to.
    pub country: String,
}

/// Movie artwork image data.
#[derive(Debug, Default, Deserialize)]
pub struct Artwork {
    /// Artwork's ID.
    pub id: String,
    /// Artwork's type.
    pub artwork_type: String,
    /// Artwork's path.
    ///
    /// For the full URL use [`full_url`](#method.full_url).
    pub url: String,
    /// Artwork thumbnail path.
    ///
    /// For the full URL use [`full_thumb_url`](#method.full_thumb_url).
    pub thumb_url: String,
    /// Artwork's tags.
    #[serde(deserialize_with = "deserialize::optional_string")]
    pub tags: Option<String>,
    /// Whether this is the primary artwork of the movie.
    pub is_primary: bool,
    /// Artwork's width.
    pub width: u16,
    /// Artwork's height.
    pub height: u16,
}

impl Artwork {
    /// Returns the full URL of the artwork image.
    pub fn full_url(&self) -> Result<Url> {
        URLS.image(&self.url)
    }

    /// Returns the full URL of the artwork thumbnail.
    pub fn full_thumb_url(&self) -> Result<Url> {
        URLS.image(&self.thumb_url)
    }
}

/// Movie trailer data.
#[derive(Debug, Deserialize)]
pub struct Trailer {
    /// Trailer full URL.
    pub url: String,
    /// Trailer title.
    pub name: String,
}

/// Movie remote ID data.
#[derive(Debug, Deserialize)]
pub struct RemoteID {
    /// The ID.
    pub id: String,
    /// ID of the remote source.
    pub source_id: u32,
    /// Name of the remote source.
    pub source_name: String,
    /// Remote movie webpage URL.
    pub url: String,
}

/// Movie people data.
#[derive(Debug, Deserialize)]
pub struct People {
    /// List of movie's actors.
    #[serde(default)]
    pub actors: Vec<Person>,
    /// List of movie's directors.
    #[serde(default)]
    pub directors: Vec<Person>,
    /// List of movie's producers.
    #[serde(default)]
    pub producers: Vec<Person>,
    /// List of movie's writers.
    #[serde(default)]
    pub writers: Vec<Person>,
}

/// Movie person (actor, director, etc.) data.
#[derive(Debug, Default, Deserialize)]
pub struct Person {
    /// Person ID for this movie.
    pub id: String,
    /// Person's name.
    pub name: String,
    /// Person's role in this movie.
    #[serde(deserialize_with = "deserialize::optional_string")]
    pub role: Option<String>,
    /// Person's image path.
    #[serde(deserialize_with = "deserialize::optional_string")]
    pub people_image: Option<String>,
    /// Person's movie role image path.
    #[serde(deserialize_with = "deserialize::optional_string")]
    pub role_image: Option<String>,
    /// Whether this person is featured for this movie.
    pub is_featured: bool,
    /// ID of the person.
    pub people_id: String,
    /// Person's IMDb ID.
    #[serde(deserialize_with = "deserialize::optional_string")]
    pub imdb_id: Option<String>,
    /// Person's Twitter.
    #[serde(deserialize_with = "deserialize::optional_string")]
    pub people_twitter: Option<String>,
    /// Person's Facebook.
    #[serde(deserialize_with = "deserialize::optional_string")]
    pub people_facebook: Option<String>,
    /// Person's Instagram.
    #[serde(deserialize_with = "deserialize::optional_string")]
    pub people_instagram: Option<String>,
}

impl Person {
    /// Returns the full URL of the person's image.
    pub fn people_image_url(&self) -> Result<Url> {
        URLS.opt_image(&self.people_image)
    }

    /// Returns the full URL of the person's role image.
    pub fn role_image_url(&self) -> Result<Url> {
        URLS.opt_image(&self.role_image)
    }
}
