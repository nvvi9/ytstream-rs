use serde::Deserialize;

use crate::error::Error;
use crate::response_data::captions::Captions;
use crate::response_data::microformat::Microformat;
use crate::response_data::playability_status::PlayabilityStatus;
use crate::response_data::streaming_data::StreamingData;
use crate::response_data::video_details::{Thumbnail, VideoDetails};

mod video_details;
mod playability_status;
pub(crate) mod streaming_data;
mod microformat;
mod captions;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponseData {
    pub(crate) captions: Option<Captions>,
    pub(crate) playability_status: PlayabilityStatus,
    pub(crate) streaming_data: StreamingData,
    pub(crate) video_details: VideoDetails,
    pub(crate) microformat: Option<Microformat>,
}

impl PlayerResponseData {
    pub(crate) fn is_video_downloadable(&self) -> Result<(), Error> {
        match &self.playability_status {
            PlayabilityStatus::Ok => Ok(()),
            PlayabilityStatus::LoginRequired { reason } if reason.starts_with("This video is private") => Err(Error::VideoPrivate),
            PlayabilityStatus::LoginRequired { .. } => Err(Error::LoginRequired),
            PlayabilityStatus::Unplayable { playable_in_embed, .. } if !playable_in_embed => Err(Error::NotPlayableInEmbed),
            PlayabilityStatus::Unplayable { reason, .. } => Err(Error::PlayabilityStatus { reason: reason.to_string() }),
            PlayabilityStatus::LiveStreamOffline { playable_in_embed, .. } if !playable_in_embed => Err(Error::NotPlayableInEmbed),
            PlayabilityStatus::LiveStreamOffline { reason, .. } => Err(Error::PlayabilityStatus { reason: reason.to_string() }),
            PlayabilityStatus::Error { reason } => Err(Error::PlayabilityStatus { reason: reason.to_string() })
        }
    }

    pub(crate) fn is_video_from_page_downloadable(&self) -> Result<(), Error> {
        match &self.playability_status {
            PlayabilityStatus::Ok => Ok(()),
            PlayabilityStatus::LoginRequired { reason } if reason.starts_with("This video is private") => Err(Error::VideoPrivate),
            PlayabilityStatus::LoginRequired { .. } => Err(Error::LoginRequired),
            PlayabilityStatus::Unplayable { reason, .. } => Err(Error::PlayabilityStatus { reason: reason.to_string() }),
            PlayabilityStatus::LiveStreamOffline { reason, .. } => Err(Error::PlayabilityStatus { reason: reason.to_string() }),
            PlayabilityStatus::Error { reason } => Err(Error::PlayabilityStatus { reason: reason.to_string() })
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub(crate) simple_text: String,
}
