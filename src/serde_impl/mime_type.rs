use crate::video_data::stream::{Codec, MimeType};
use mime::Mime;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

pub fn deserialize<'de, D>(deserializer: D) -> Result<MimeType, <D as Deserializer<'de>>::Error>
where
    D: Deserializer<'de>,
{
    static PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r#"(\w+/\w+);\scodecs="([a-zA-Z-0-9.,\s]*)""#).unwrap());

    let s = String::deserialize(deserializer)?;
    let captures = PATTERN.captures(&s).ok_or(D::Error::invalid_value(
        Unexpected::Str(&s),
        &"a valid mime type with the format <TYPE>/<SUBTYPE>",
    ))?;

    let mut matches = captures.iter().skip(1);

    let mime_type_with_codecs = match (matches.next(), matches.next(), matches.next()) {
        (Some(item1), Some(item2), None) => Some((item1, item2)),
        _ => None,
    };

    let (mime_type, codecs) = mime_type_with_codecs
        .and_then(|(m, c)| m.map(|m| c.map(|c| (m.as_str(), c.as_str()))))
        .flatten()
        .ok_or(D::Error::invalid_value(
            Unexpected::Str(&s),
            &"a valid mime type with the format <TYPE>/<SUBTYPE>",
        ))?;

    let mime = Mime::from_str(mime_type).map_err(|_| {
        D::Error::invalid_value(
            Unexpected::Str(mime_type),
            &r#"a valid mime type with the format `(\w+/\w+);\scodecs="([a-zA-Z-0-9.,\s]*)"`"#,
        )
    })?;

    let codecs = codecs.split(", ").map(Codec::from_str).collect();

    Ok(MimeType { mime, codecs })
}
