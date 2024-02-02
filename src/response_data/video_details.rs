use serde::Deserialize;
use serde_with::{json::JsonString, serde_as};

use crate::video_data::Thumbnail;

#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails {
    pub video_id: String,
    pub title: String,
    #[serde_as(as = "JsonString")]
    pub length_seconds: Option<u64>,
    pub keywords: Option<Vec<String>>,
    pub channel_id: String,
    pub is_owner_viewing: bool,
    pub short_description: String,
    pub is_crawlable: bool,
    pub thumbnail: Thumbnails,
    pub average_rating: Option<f64>,
    pub allow_ratings: bool,
    #[serde_as(as = "JsonString")]
    pub view_count: u64,
    pub author: String,
    pub is_private: bool,
    pub is_unplugged_corpus: bool,
    pub is_live_content: bool,
}

#[derive(Deserialize)]
pub struct Thumbnails {
    pub thumbnails: Vec<Thumbnail>,
}
