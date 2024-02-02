use std::time::Duration;

use chrono::NaiveDate;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

use crate::{error::Error, response_data::PlayerResponseData};

use self::stream::Stream;

pub mod stream;

#[derive(Debug)]
pub struct VideoData {
    pub id: String,
    pub title: String,
    pub description: String,
    pub channel: String,
    pub channel_id: String,
    pub duration: Duration,
    pub view_count: u64,
    pub publish_date: Option<NaiveDate>,
    pub keywords: Option<Vec<String>>,
    pub thumbnails: Vec<Thumbnail>,
    pub streams: Vec<Stream>,
}

#[derive(Deserialize, Debug)]
pub struct Thumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

impl VideoData {
    pub(crate) fn from_video_page(body: &str) -> Result<VideoData, Error> {
        static PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new(r#"\bvar ytInitialPlayerResponse\s*=\s*(\{.+?\});"#).unwrap());

        let initial_player_response = PATTERN
            .captures(body)
            .and_then(|c| c.get(1))
            .ok_or(Error::InitialPlayerResponseNotFound)?;

        let player_response_data =
            serde_json::from_str::<PlayerResponseData>(initial_player_response.as_str())
                .map_err(|_| Error::PlayerResponseDataParse)?;

        match player_response_data.is_video_from_page_downloadable() {
            Ok(()) => Ok(Self::from_player_response_data(player_response_data)),
            Err(e) => Err(e),
        }
    }

    pub(crate) fn from_video_player(
        player_response_data: PlayerResponseData,
    ) -> Result<Self, Error> {
        return match player_response_data.is_video_downloadable() {
            Err(e) => Err(e),
            Ok(()) => Ok(Self::from_player_response_data(player_response_data)),
        };
    }

    fn from_player_response_data(player_response_data: PlayerResponseData) -> Self {
        let id = player_response_data.video_details.video_id;
        let title = player_response_data.video_details.title;
        let description = player_response_data.video_details.short_description;
        let channel = player_response_data.video_details.author;
        let channel_id = player_response_data.video_details.channel_id;
        let view_count = player_response_data.video_details.view_count;

        let player_microformat_renderer = player_response_data
            .microformat
            .map(|m| m.player_microformat_renderer);

        let length_seconds = match player_response_data.video_details.length_seconds {
            Some(v) => v,
            None => player_microformat_renderer
                .as_ref()
                .map_or(0, |renderer| renderer.length_seconds),
        };

        let duration = Duration::from_secs(length_seconds);

        let publish_date = player_microformat_renderer.map(|renderer| renderer.publish_date);

        let keywords = player_response_data.video_details.keywords;
        let thumbnails = player_response_data
            .video_details
            .thumbnail
            .thumbnails
            .iter()
            .map(|t| Thumbnail {
                url: t.url.to_string(),
                width: t.width,
                height: t.height,
            })
            .collect::<Vec<Thumbnail>>();

        let mut formats = Vec::from(player_response_data.streaming_data.formats);
        let mut adaptive_formats = Vec::from(player_response_data.streaming_data.adaptive_formats);
        formats.append(&mut adaptive_formats);

        let streams = formats.into_iter().map(Stream::from_format).collect();

        Self {
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
            streams,
        }
    }
}
