use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub fn optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Value::deserialize(deserializer)?;
    match v {
        Value::String(s) if !s.is_empty() => Ok(Some(s)),
        _ => Ok(None),
    }
}

pub fn optional_float<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let f = f32::deserialize(deserializer)?;
    if f == 0.0 {
        Ok(None)
    } else {
        Ok(Some(f))
    }
}

pub fn optional_naive_time<'de, D>(deserializer: D) -> Result<Option<NaiveTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        let t = NaiveTime::parse_from_str(&s, "%l:%M %p").map_err(serde::de::Error::custom)?;

        Ok(Some(t))
    }
}

pub fn optional_date<'de, D>(deserializer: D) -> Result<Option<Date<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        let nd = NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)?;

        Ok(Some(Utc.from_utc_date(&nd)))
    }
}

pub fn optional_date_time<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() || is_zero_date_time_str(&s) {
        Ok(None)
    } else {
        let ndt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)?;

        Ok(Some(Utc.from_utc_datetime(&ndt)))
    }
}

pub fn u16_string<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse()
        .map_err(serde::de::Error::custom)
}

pub fn optional_ts_seconds_date_time<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let secs = i64::deserialize(deserializer)?;

    if secs == 0 {
        Ok(None)
    } else {
        Ok(Some(Utc.timestamp(secs, 0)))
    }
}

fn is_zero_date_time_str(s: &str) -> bool {
    s == "0000-00-00 00:00:00"
}
