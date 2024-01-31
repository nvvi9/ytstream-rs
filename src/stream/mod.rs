use url::Url;

use crate::response_data::streaming_data::{AudioQuality, Format, MimeType, Quality};

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