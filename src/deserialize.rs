use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};

pub fn optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    match opt_string(deserializer)? {
        Some(s) if !s.is_empty() => Ok(Some(s)),
        _ => Ok(None),
    }
}

pub fn optional_float<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_f: Option<f32> = Option::deserialize(deserializer)?;

    match opt_f {
        Some(f) if f != 0.0 => Ok(Some(f)),
        _ => Ok(None),
    }
}

pub fn optional_naive_time<'de, D>(deserializer: D) -> Result<Option<NaiveTime>, D::Error>
where
    D: Deserializer<'de>,
{
    match opt_string(deserializer)? {
        Some(s) if !s.is_empty() => {
            let t = NaiveTime::parse_from_str(&s, "%l:%M %p").map_err(serde::de::Error::custom)?;

            Ok(Some(t))
        }
        _ => Ok(None),
    }
}

pub fn optional_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    match opt_string(deserializer)? {
        Some(s) if !s.is_empty() => {
            let nd = NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)?;

            Ok(Some(nd))
        }
        _ => Ok(None),
    }
}

pub fn optional_date_time<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    match opt_string(deserializer)? {
        Some(s) if !s.is_empty() && !is_zero_date_time_str(&s) => {
            let ndt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                .map_err(serde::de::Error::custom)?;

            Ok(Some(Utc.from_utc_datetime(&ndt)))
        }
        _ => Ok(None),
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
    let opt_secs: Option<i64> = Option::deserialize(deserializer)?;

    match opt_secs {
        Some(secs) if secs != 0 => Ok(Some(Utc.timestamp(secs, 0))),
        _ => Ok(None),
    }
}

pub fn int_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match i32::deserialize(deserializer)? {
        0 => Ok(false),
        _ => Ok(true),
    }
}

fn is_zero_date_time_str(s: &str) -> bool {
    s == "0000-00-00 00:00:00"
}

fn opt_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::deserialize(deserializer)
}
