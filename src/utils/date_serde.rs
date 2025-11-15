//! Custom (de)serialization for `chrono::DateTime<Utc>`
//! to/from the ISO-8601 date string used by the API: `"YYYY-MM-DD"`
//!
//! Example:
//! ```json
//! { "date": "2025-11-15" }
//! ```

use chrono::{DateTime, TimeZone, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

/// Format string used by the Open Schedule API for the `date` field.
const FORMAT: &str = "%Y-%m-%d";

/// Serialize a `DateTime<Utc>` into `"YYYY-MM-DD"`.
pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.format(FORMAT).to_string();
    serializer.serialize_str(&s)
}

/// Deserialize a `"YYYY-MM-DD"` string into a `DateTime<Utc>` (time part is set to 00:00:00 UTC).
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    // Parse only the date part; the time will be midnight UTC.
    Utc.datetime_from_str(&format!("{} 00:00:00", s), "%Y-%m-%d %H:%M:%S")
        .map_err(serde::de::Error::custom)
}
