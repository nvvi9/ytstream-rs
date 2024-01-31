use std::time::Duration;

use chrono::NaiveDate;

use crate::error::Error;
use crate::response_data::PlayerResponseData;

pub struct VideoData {
    pub id: String,
    pub title: String,
    pub description: String,
    pub channel: String,
    pub channel_id: String,
    pub duration: Duration,
    pub view_count: u64,
    pub publish_date: Option<NaiveDate>,
    pub keywords: Vec<String>,
    pub thumbnails: Vec<Thumbnail>,
}

pub struct Thumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

impl VideoData {
    pub(crate) fn from_player_response_data(player_response_data: PlayerResponseData) -> Result<Self, Error> {
        if let Err(e) = player_response_data.playability_status.is_video_downloadable() {
            return Err(e);
        }

        let id = player_response_data.video_details.video_id;
        let title = player_response_data.video_details.title;
        let description = player_response_data.video_details.short_description;
        let channel = player_response_data.video_details.author;
        let channel_id = player_response_data.video_details.channel_id;
        let view_count = player_response_data.video_details.view_count;

        let player_microformat_renderer = player_response_data.microformat.map(|m| m.player_microformat_renderer);

        let length_seconds = match player_response_data.video_details.length_seconds {
            Some(v) => v,
            None => player_microformat_renderer.as_ref().map_or(0, |renderer| renderer.length_seconds)
        };

        let duration = Duration::from_secs(length_seconds);

        let publish_date = player_microformat_renderer.map(|renderer| renderer.publish_date);

        let keywords = player_response_data.video_details.keywords;
        let thumbnails = player_response_data.video_details.thumbnail.thumbnails.iter().map(|t| Thumbnail { url: t.url.to_string(), width: t.width, height: t.height })
            .collect::<Vec<Thumbnail>>();

        Ok(Self {
            id,
            title,
            description,
            channel,
            channel_id,
            duration,
            view_count,
            publish_date,
            keywords,
            thumbnails,
        })
    }
}
