use serde::Deserialize;
use serde_with::{json::JsonString, serde_as};
use url::Url;

#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    #[serde_as(as = "JsonString")]
    pub(crate) expires_in_seconds: u64,
    pub(crate) formats: Vec<Format>,
    pub(crate) adaptive_formats: Vec<Format>,
    pub(crate) dash_manifest_url: Option<String>,
    pub(crate) hls_manifest_url: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub(crate) itag: i32,
    pub(crate) url: Url,
    pub(crate) mime_type: String,
    pub(crate) quality: Quality,
    pub(crate) signature_cipher: Option<String>,
    pub(crate) bitrate: i32,
    pub(crate) fps: Option<i32>,
    pub(crate) width: Option<i32>,
    pub(crate) height: Option<i32>,
    pub(crate) last_modified: String,
    pub(crate) content_length: Option<String>,
    pub(crate) quality_label: Option<String>,
    pub(crate) projection_type: String,
    pub(crate) average_bitrate: Option<i32>,
    pub(crate) audio_quality: Option<AudioQuality>,
    pub(crate) approx_duration_ms: String,
    pub(crate) audio_sample_rate: Option<String>,
    pub(crate) audio_channels: Option<i32>,
    pub(crate) init_range: Option<Range>,
    pub(crate) index_range: Option<Range>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Quality {
    Tiny,
    Small,
    Medium,
    Large,
    Highres,
    Hd720,
    Hd1080,
    Hd1440,
    Hd2160,
}

#[derive(Deserialize)]
pub enum AudioQuality {
    #[serde(rename = "AUDIO_QUALITY_ULTRALOW")]
    UltraLow,
    #[serde(rename = "AUDIO_QUALITY_LOW")]
    Low,
    #[serde(rename = "AUDIO_QUALITY_MEDIUM")]
    Medium,
    #[serde(rename = "AUDIO_QUALITY_HIGH")]
    High,
}

#[serde_as]
#[derive(Deserialize)]
pub struct Range {
    #[serde_as(as = "JsonString")]
    start: u64,
    #[serde_as(as = "JsonString")]
    end: u64,
}
