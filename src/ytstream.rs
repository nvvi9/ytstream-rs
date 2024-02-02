use crate::{
    client_type::ClientType, error::Error, video_data::VideoData, youtube_client::YoutubeClient,
};

pub struct YTStream {
    android_client: YoutubeClient,
    embedded_client: YoutubeClient,
}

impl YTStream {
    pub fn new() -> Result<Self, Error> {
        let android_client = YoutubeClient::new(ClientType::Android)?;
        let embedded_client = YoutubeClient::new(ClientType::Embedded)?;

        Ok(Self {
            android_client,
            embedded_client,
        })
    }

    pub async fn extract(&self, id: &str) -> Result<VideoData, Error> {
        let player_response = self
            .android_client
            .player_response_data(id)
            .await
            .map_err(|e| Error::Request(e))?;

        match VideoData::from_video_player(player_response) {
            Ok(video_data) => Ok(video_data),
            Err(Error::NotPlayableInEmbed) => {
                let page = self
                    .android_client
                    .video_page(id)
                    .await
                    .map_err(|e| Error::Request(e))?;
                VideoData::from_video_page(&page)
            }
            Err(Error::LoginRequired) => {
                let embedded_player_response = self
                    .embedded_client
                    .player_response_data(id)
                    .await
                    .map_err(|e| Error::Request(e))?;

                VideoData::from_video_player(embedded_player_response)
            }
            e @ Err(..) => e,
        }
    }
}
