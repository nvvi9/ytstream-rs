use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};
use serde::de::{Error, Unexpected};

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error> where D: Deserializer<'de> {
    let date = <&str>::deserialize(deserializer)?;
    NaiveDate::parse_from_str(date, "%F")
        .ok()
        .ok_or_else(|| Error::invalid_value(Unexpected::Str(date), &"a yyyy-mm-dd date string"))
}
