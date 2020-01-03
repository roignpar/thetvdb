use chrono::{Duration, TimeZone, Utc};
use lazy_static::lazy_static;
use tokio::sync::{Mutex, MutexGuard};

use thetvdb::{params::*, Client};

mod data;

use data::*;

const ENV_APIKEY: &str = "THETVDB_APIKEY";

lazy_static! {
    static ref CLIENT: Mutex<Option<Client>> = Mutex::new(None);
}

#[tokio::test]
async fn search() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let cases = vec![
        ("name", SearchBy::Name(&PEII.series_name)),
        ("IMDb ID", SearchBy::IMDbID(&PEII.imdb_id)),
        ("Zap2it ID", SearchBy::Zap2itID(&PEII.zap2it_id)),
        ("slug", SearchBy::Slug(&PEII.slug)),
    ];

    for (case_name, search_by) in cases.into_iter() {
        let series = client
            .search(search_by)
            .await
            .unwrap()
            .into_iter()
            .find(|s| *s == *PEII);

        if series.is_none() {
            panic!("Expected series missing from {} search results", case_name)
        }
    }
}

#[tokio::test]
async fn series() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let peii = client.series(PEII.id).await.expect("Error fetching series");

    assert_eq!(peii, *PEII);
}

#[tokio::test]
async fn series_last_modified() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    client
        .series_last_modified(PEII.id)
        .await
        .expect("Error fetching series last modified");
}

#[tokio::test]
async fn series_actors() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let actor = client
        .series_actors(PEII.id)
        .await
        .expect("Error fetching series actors")
        .into_iter()
        .find(|a| *a == *NARRATOR);

    if actor.is_none() {
        panic!("Expected actor missing from series actor list");
    }
}

#[tokio::test]
async fn series_episodes() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let params = EpisodeParams::new(PEII.id);
    let page = client
        .series_episodes(&params)
        .await
        .expect("Error fetching series episodes");

    assert_eq!(page.episodes.len(), EPISODE_COUNT);

    let episode = page.episodes.into_iter().find(|e| *e == *ISLANDS);

    if episode.is_none() {
        panic!("Expected episode missing from series episode list");
    }
}

#[tokio::test]
async fn series_episodes_query() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let params = EpisodeQueryParams::new(PEII.id)
        .aired_season(ISLANDS.aired_season)
        .aired_episode(ISLANDS.aired_episode_number);

    let page = client
        .series_episodes_query(&params)
        .await
        .expect("Error fetching series episodes query");

    assert_eq!(page.episodes.len(), 1);

    let episode = &page.episodes[0];

    assert_eq!(*episode, *ISLANDS);
}

#[tokio::test]
async fn series_episode_summary() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let summary = client
        .series_episodes_summary(PEII.id)
        .await
        .expect("Error fetching series episode summary");

    assert_eq!(summary, *EPISODE_SUMMARY);
}

#[tokio::test]
async fn series_filter() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let keys = SeriesFilterKeys::new()
        .id()
        .series_name()
        .first_aired()
        .network()
        .slug()
        .status()
        .imdb_id()
        .zap2it_id();

    let series = client
        .series_filter(PEII.id, &keys)
        .await
        .expect("Error fetching filtered series");

    assert_eq!(series, *PEII);
}

#[tokio::test]
async fn series_images() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let images = client.series_images(PEII.id).await;

    if images.is_err() {
        panic!("Error fetching series images: {:?}", images.unwrap_err());
    }
}

#[tokio::test]
async fn series_images_query() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let params = ImageQueryParams::with_key_type("series");

    let images = client.series_images_query(PEII.id, &params).await;

    if images.is_err() {
        panic!(
            "Error fetching series images query: {:?}",
            images.unwrap_err()
        );
    }
}

#[tokio::test]
async fn series_images_query_params() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let image_keys = client.series_images_query_params(PEII.id).await;

    if image_keys.is_err() {
        panic!(
            "Error fetching series images query params: {:?}",
            image_keys.unwrap_err()
        );
    }
}

#[tokio::test]
async fn episode() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let episode = client
        .episode(ISLANDS.id)
        .await
        .expect("Error fetching episode");

    assert_eq!(episode, *ISLANDS);
}

#[tokio::test]
async fn languages() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let language = client
        .languages()
        .await
        .expect("Error fetching languages")
        .into_iter()
        .find(|l| *l == *SAMOAN);

    if language.is_none() {
        panic!("Expected language missing from languages list");
    }
}

#[tokio::test]
async fn language() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let language = client
        .language(SAMOAN.id)
        .await
        .expect("Error fetching language");

    assert_eq!(language, *SAMOAN);
}

#[tokio::test]
async fn updated() {
    let guard = get_client().await;
    let client = guard.as_ref().unwrap();

    let params = UpdatedParams::new(Utc::now() - Duration::days(1));

    let updates = client.updated(&params).await;

    if updates.is_err() {
        panic!("Error fetching updated series: {:?}", updates.unwrap_err());
    }

}

// Because there is no way to use async fns in lazy_static blocks
// CLIENT will be created here.
async fn get_client() -> MutexGuard<'static, Option<Client>> {
    {
        let mut client = CLIENT.lock().await;

        if client.is_none() {
            let api_key = std::env::var(ENV_APIKEY)
                .expect(&format!("Missing or invalid {} env var", ENV_APIKEY));

            *client = Some(
                Client::new(api_key)
                    .await
                    .expect("Could not authenticate test client"),
            );
        }
    }

    CLIENT.lock().await
}
