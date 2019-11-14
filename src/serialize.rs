use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Timestamp(
    #[serde(serialize_with = "chrono::serde::ts_seconds::serialize")] pub DateTime<Utc>,
);
