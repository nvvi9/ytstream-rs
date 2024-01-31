use serde::Deserialize;
use serde_with::{json::JsonString, serde_as};

#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails {
    pub(crate) video_id: String,
    pub(crate) title: String,
    #[serde_as(as = "JsonString")]
    pub(crate) length_seconds: Option<u64>,
    pub(crate) keywords: Vec<String>,
    pub(crate) channel_id: String,
    pub(crate) is_owner_viewing: bool,
    pub(crate) short_description: String,
    pub(crate) is_crawlable: bool,
    pub(crate) thumbnail: Thumbnails,
    pub(crate) average_rating: Option<f64>,
    pub(crate) allow_ratings: bool,
    #[serde_as(as = "JsonString")]
    pub(crate) view_count: u64,
    pub(crate) author: String,
    pub(crate) is_private: bool,
    pub(crate) is_unplugged_corpus: bool,
    pub(crate) is_live_content: bool,
}

#[derive(Deserialize)]
pub struct Thumbnails {
    pub(crate) thumbnails: Vec<crate::response_data::Thumbnail>,
}

#[derive(Deserialize)]
pub struct Thumbnail {
    pub(crate) url: String,
    pub(crate) width: u32,
    pub(crate) height: u32,
}
