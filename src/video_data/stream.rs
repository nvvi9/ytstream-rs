use crate::response_data::streaming_data::Format;
use mime::Mime;
use serde::Deserialize;
use url::Url;

#[derive(Debug)]
pub struct Stream {
    pub itag: i32,
    pub url: Url,
    pub mime_type: MimeType,
    pub quality: Quality,
    pub bitrate: i32,
    pub fps: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub audio_quality: Option<AudioQuality>,
}

impl Stream {
    pub(crate) fn from_format(format: Format) -> Self {
        Self {
            itag: format.itag,
            url: format.url,
            mime_type: format.mime_type,
            quality: format.quality,
            bitrate: format.bitrate,
            fps: format.fps,
            width: format.width,
            height: format.height,
            audio_quality: format.audio_quality,
        }
    }
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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

#[derive(Debug)]
pub struct MimeType {
    pub mime: Mime,
    pub codecs: Vec<Codec>,
}

#[derive(Debug)]
pub enum Codec {
    MP4A,
    AVC1,
    VP9,
    AV1,
    OPUS,
    Unknown,
}

impl Codec {
    pub(crate) fn from_str(s: &str) -> Self {
        match s {
            str if str.starts_with("mp4a") => Self::MP4A,
            str if str.starts_with("avc1") => Self::AVC1,
            str if str.starts_with("vp9") => Self::VP9,
            str if str.starts_with("av01") => Self::AV1,
            str if str.starts_with("opus") => Self::OPUS,
            _ => Self::Unknown,
        }
    }
}
