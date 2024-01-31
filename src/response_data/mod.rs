use serde::Deserialize;

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


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub(crate) simple_text: String,
}
