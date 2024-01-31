use chrono::NaiveDate;
use serde::Deserialize;
use serde_with::{json::JsonString, serde_as};

use crate::response_data::Text;
use crate::response_data::video_details::Thumbnail;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Microformat {
    pub(crate) player_microformat_renderer: PlayerMicroformatRenderer,
}

#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMicroformatRenderer {
    pub(crate) thumbnail: Thumbnail,
    pub(crate) title: Text,
    pub(crate) description: Text,
    #[serde_as(as = "JsonString")]
    pub(crate) length_seconds: u64,
    pub(crate) owner_profile_url: String,
    pub(crate) external_channel_id: String,
    pub(crate) is_family_safe: bool,
    pub(crate) available_countries: Vec<String>,
    pub(crate) is_unlisted: bool,
    pub(crate) has_ypc_metadata: bool,
    pub(crate) view_count: String,
    pub(crate) category: String,
    #[serde(with = "crate::serde_impl::naive_date")]
    pub(crate) publish_date: NaiveDate,
    pub(crate) owner_channel_name: String,
    pub(crate) upload_data: String,
}

