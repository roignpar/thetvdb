use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Serialize)]
pub struct Timestamp(
    #[serde(serialize_with = "chrono::serde::ts_seconds::serialize")] pub DateTime<Utc>,
);

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

pub mod optional_naive_time {
    use super::*;

    const FORMAT: &str = "%l:%M %p";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match opt_string(deserializer)? {
            Some(s) if !s.is_empty() => {
                let t = NaiveTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;

                Ok(Some(t))
            }
            _ => Ok(None),
        }
    }

    pub fn serialize<S>(ont: &Option<NaiveTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match ont {
            None => serializer.serialize_none(),
            Some(nt) => {
                let f = nt.format(FORMAT);
                serializer.collect_str(&f)
            }
        }
    }
}

pub mod optional_naive_date {
    use super::*;

    const FORMAT: &str = "%Y-%m-%d";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match opt_string(deserializer)? {
            Some(s) if !s.is_empty() => {
                let nd = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;

                Ok(Some(nd))
            }
            _ => Ok(None),
        }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(ond: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match ond {
            None => serializer.serialize_none(),
            Some(nd) => {
                let f = nd.format(FORMAT);
                serializer.collect_str(&f)
            }
        }
    }
}

pub mod optional_date_time {
    use super::*;

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match opt_string(deserializer)? {
            Some(s) if !s.is_empty() && !is_zero_date_time_str(&s) => {
                let ndt =
                    NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;

                Ok(Some(Utc.from_utc_datetime(&ndt)))
            }
            _ => Ok(None),
        }
    }

    pub fn serialize<S>(odt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match odt {
            None => serializer.serialize_none(),
            Some(dt) => {
                let f = dt.format(FORMAT);
                serializer.collect_str(&f)
            }
        }
    }
}

pub mod u32_string {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(u: &u32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&u.to_string())
    }
}

pub mod optional_ts_seconds_date_time {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt_secs: Option<i64> = Option::deserialize(deserializer)?;

        match opt_secs {
            Some(secs) if secs != 0 => Ok(Some(Utc.timestamp(secs, 0))),
            _ => Ok(None),
        }
    }

    pub fn serialize<S>(odt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match odt {
            None => serializer.serialize_none(),
            Some(dt) => serializer.serialize_i64(dt.timestamp()),
        }
    }
}

pub mod int_bool {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        match i32::deserialize(deserializer)? {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(b: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(if *b { 1 } else { 0 })
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
