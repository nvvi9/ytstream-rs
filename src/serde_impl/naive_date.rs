use chrono::NaiveDate;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let date = <&str>::deserialize(deserializer)?;
    NaiveDate::parse_from_str(date, "%F")
        .ok()
        .ok_or_else(|| D::Error::invalid_value(Unexpected::Str(date), &"a yyyy-mm-dd date string"))
}
