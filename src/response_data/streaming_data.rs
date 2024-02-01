use crate::video_data::stream::{AudioQuality, MimeType, Quality};
use serde::Deserialize;
use serde_with::{json::JsonString, serde_as};
use url::Url;

#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    #[serde_as(as = "JsonString")]
    pub expires_in_seconds: u64,
    pub formats: Vec<Format>,
    pub adaptive_formats: Vec<Format>,
    pub dash_manifest_url: Option<String>,
    pub hls_manifest_url: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub itag: i32,
    pub url: Url,
    #[serde(with = "crate::serde_impl::mime_type")]
    pub mime_type: MimeType,
    pub quality: Quality,
    pub signature_cipher: Option<String>,
    pub bitrate: i32,
    pub fps: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub last_modified: String,
    pub content_length: Option<String>,
    pub quality_label: Option<String>,
    pub projection_type: String,
    pub average_bitrate: Option<i32>,
    pub audio_quality: Option<AudioQuality>,
    pub approx_duration_ms: String,
    pub audio_sample_rate: Option<String>,
    pub audio_channels: Option<i32>,
    pub init_range: Option<Range>,
    pub index_range: Option<Range>,
}

#[serde_as]
#[derive(Deserialize)]
pub struct Range {
    #[serde_as(as = "JsonString")]
    pub start: u64,
    #[serde_as(as = "JsonString")]
    pub end: u64,
}
