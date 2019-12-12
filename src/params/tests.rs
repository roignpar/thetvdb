use super::*;

#[test]
fn search_by_to_query_param() {
    let name = "name_test";
    let imdb_id = "imdb_test";
    let zap2it_id = "zap2it_test";
    let slug = "slug_test";

    assert_eq!(SearchBy::Name(name).query_param(), [("name", name)]);
    assert_eq!(
        SearchBy::IMDbID(imdb_id).query_param(),
        [("imdbId", imdb_id)]
    );
    assert_eq!(
        SearchBy::Zap2itID(zap2it_id).query_param(),
        [("zap2itId", zap2it_id)]
    );
    assert_eq!(SearchBy::Slug(slug).query_param(), [("slug", slug)]);
}

#[test]
fn series_filter_keys() {
    let mut keys = SeriesFilterKeys::new();

    assert!(keys.is_empty());
    assert_eq!(keys.keys_query.capacity(), SeriesFilterKeys::FULL_CAPACITY);

    assert!(!keys.keys_query.contains("airsTime"));
    keys = keys.airs_time();
    assert!(keys.keys_query.contains("airsTime"));

    assert!(!keys.keys_query.contains("siteRating"));
    keys = keys.site_rating();
    assert!(keys.keys_query.contains(",siteRating"));

    assert!(!keys.keys_query.contains("seriesName"));
    keys = keys.series_name();
    assert!(keys.keys_query.contains(",seriesName"));

    assert!(!keys.keys_query.contains("firstAired"));
    keys = keys.first_aired();
    assert!(keys.keys_query.contains(",firstAired"));

    assert!(!keys.keys_query.contains("runtime"));
    keys = keys.runtime();
    assert!(keys.keys_query.contains(",runtime"));

    assert!(!keys.keys_query.contains("overview"));
    keys = keys.overview();
    assert!(keys.keys_query.contains(",overview"));

    assert!(!keys.keys_query.contains("banner"));
    keys = keys.banner();
    assert!(keys.keys_query.contains(",banner"));

    assert!(!keys.keys_query.contains("genre"));
    keys = keys.genre();
    assert!(keys.keys_query.contains(",genre"));

    assert!(!keys.keys_query.contains("airsDayOfWeek"));
    keys = keys.airs_day_of_week();
    assert!(keys.keys_query.contains(",airsDayOfWeek"));

    assert!(!keys.keys_query.contains("imdbId"));
    keys = keys.imdb_id();
    assert!(keys.keys_query.contains(",imdbId"));

    assert!(!keys.keys_query.contains("added"));
    keys = keys.added();
    assert!(keys.keys_query.contains(",added"));

    assert!(!keys.keys_query.contains("addedBy"));
    keys = keys.added_by();
    assert!(keys.keys_query.contains(",addedBy"));

    assert!(!keys.keys_query.contains("siteRatingCount"));
    keys = keys.site_rating_count();
    assert!(keys.keys_query.contains(",siteRatingCount"));

    assert!(!keys.keys_query.contains("id"));
    keys = keys.id();
    assert!(keys.keys_query.contains(",id"));

    assert!(!keys.keys_query.contains("status"));
    keys = keys.status();
    assert!(keys.keys_query.contains(",status"));

    assert!(!keys.keys_query.contains("network"));
    keys = keys.network();
    assert!(keys.keys_query.contains(",network"));

    assert!(!keys.keys_query.contains("networkId"));
    keys = keys.network_id();
    assert!(keys.keys_query.contains(",networkId"));

    assert!(!keys.keys_query.contains("rating"));
    keys = keys.rating();
    assert!(keys.keys_query.contains(",rating"));

    assert!(!keys.keys_query.contains("zap2itId"));
    keys = keys.zap2it_id();
    assert!(keys.keys_query.contains(",zap2itId"));

    assert!(!keys.keys_query.contains("slug"));
    keys = keys.slug();
    assert!(keys.keys_query.contains(",slug"));

    assert!(!keys.keys_query.contains("aliases"));
    keys = keys.aliases();
    assert!(keys.keys_query.contains(",aliases"));

    assert!(!keys.keys_query.contains("season"));
    keys = keys.season();
    assert!(keys.keys_query.contains(",season"));

    assert!(!keys.keys_query.contains("poster"));
    keys = keys.poster();
    assert!(keys.keys_query.contains(",poster"));

    assert!(!keys.keys_query.contains("fanart"));
    keys = keys.fanart();
    assert!(keys.keys_query.contains(",fanart"));

    assert!(!keys.keys_query.contains("language"));
    keys = keys.language();
    assert!(keys.keys_query.contains(",language"));

    println!("{}", keys.keys_query);

    assert!(keys.is_at_full_capacity());
}
