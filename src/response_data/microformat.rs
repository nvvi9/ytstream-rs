use crate::{response_data::Text, video_data::Thumbnail};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_with::{json::JsonString, serde_as};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Microformat {
    pub player_microformat_renderer: PlayerMicroformatRenderer,
}

#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMicroformatRenderer {
    pub thumbnail: Thumbnail,
    pub title: Text,
    pub description: Text,
    #[serde_as(as = "JsonString")]
    pub length_seconds: u64,
    pub owner_profile_url: String,
    pub external_channel_id: String,
    pub is_family_safe: bool,
    pub available_countries: Vec<String>,
    pub is_unlisted: bool,
    pub has_ypc_metadata: bool,
    pub view_count: String,
    pub category: String,
    #[serde(with = "crate::serde_impl::naive_date")]
    pub publish_date: NaiveDate,
    pub owner_channel_name: String,
    pub upload_data: String,
}
