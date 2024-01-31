use serde::Deserialize;

use crate::error::Error;

#[derive(Deserialize)]
#[serde(tag = "status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayabilityStatus {
    Ok,
    Unplayable {
        reason: String,
        playable_in_embed: bool,
    },
    LoginRequired {
        reason: String
    },
    LiveStreamOffline {
        reason: String,
        playable_in_embed: bool,
    },
    Error {
        reason: String
    },
}

impl PlayabilityStatus {
    pub(crate) fn is_video_downloadable(&self) -> Result<(), Error> {
        match self {
            PlayabilityStatus::Ok => Ok(()),
            PlayabilityStatus::LoginRequired { reason } if reason.starts_with("This video is private") => Err(Error::VideoPrivate),
            PlayabilityStatus::LoginRequired { .. } => Err(Error::LoginRequired),
            PlayabilityStatus::Unplayable { playable_in_embed, .. } if !*playable_in_embed => Err(Error::NotPlayableInEmbed),
            PlayabilityStatus::Unplayable { reason, .. } => Err(Error::PlayabilityStatus { reason: reason.to_string() }),
            PlayabilityStatus::LiveStreamOffline { playable_in_embed, .. } if !*playable_in_embed => Err(Error::NotPlayableInEmbed),
            PlayabilityStatus::LiveStreamOffline { reason, .. } => Err(Error::PlayabilityStatus { reason: reason.to_string() }),
            PlayabilityStatus::Error { reason } => Err(Error::PlayabilityStatus { reason: reason.to_string() })
        }
    }
}
