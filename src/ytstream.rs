use crate::client_type::ClientType;
use crate::error::Error;
use crate::request_data::InnertubeRequest;
use crate::response_data::PlayerResponseData;
use crate::video_data::VideoData;
use reqwest::cookie::Jar;
use reqwest::header::HeaderMap;
use reqwest::{header, Client};
use std::sync::Arc;

pub struct YTStream {
    client: Client,
}

impl YTStream {
    pub fn new() -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(header::ORIGIN, "https://youtube.com".parse().unwrap());
        headers.insert("Sec-Fetch-Mode", "navigate".parse().unwrap());
        headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            header::ACCEPT,
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"
                .parse()
                .unwrap(),
        );
        headers.insert("X-Youtube-Client-Name", "3".parse().unwrap());

        let cookie = "CONSENT=YES+; Path=/; Domain=.youtube.com;";
        let cookie_jar = Jar::default();
        cookie_jar.add_cookie_str(cookie, &"https://youtube.com".parse().unwrap());

        let client = Client::builder()
            .default_headers(headers)
            .cookie_provider(Arc::new(cookie_jar))
            .build()
            .map_err(|e| Error::Request(e))?;

        Ok(Self { client })
    }

    pub async fn extract(&self, id: &str) -> Result<VideoData, Error> {
        let player_response = self
            .video_data_by_innertube(id, &ClientType::Android)
            .await
            .map_err(|e| Error::Request(e))?;

        match VideoData::from_video_player(player_response) {
            Ok(video_data) => Ok(video_data),
            Err(Error::NotPlayableInEmbed) => {
                let page = self
                    .video_page(id, &ClientType::Android)
                    .await
                    .map_err(|e| Error::Request(e))?;
                VideoData::from_video_page(&page)
            }
            Err(Error::LoginRequired) => {
                let embedded_player_response = self
                    .video_data_by_innertube(id, &ClientType::Embedded)
                    .await
                    .map_err(|e| Error::Request(e))?;

                VideoData::from_video_player(embedded_player_response)
            }
            e @ Err(..) => e,
        }
    }

    async fn video_data_by_innertube(
        &self,
        id: &str,
        client_type: &ClientType,
    ) -> Result<PlayerResponseData, reqwest::Error> {
        let request = InnertubeRequest::video_data_request(id, client_type);

        let mut headers = HeaderMap::new();
        headers.append("User-Agent", client_type.user_agent().parse().unwrap());
        headers.append(
            "X-Youtube-Client-Version",
            client_type.version().parse().unwrap(),
        );

        self.client
            .post(format!(
                "https://www.youtube.com/youtubei/v1/player?key={}",
                client_type.key()
            ))
            .headers(headers)
            .json(&request)
            .send()
            .await?
            .json::<PlayerResponseData>()
            .await
    }

    async fn video_page(
        &self,
        id: &str,
        client_type: &ClientType,
    ) -> Result<String, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.append("User-Agent", client_type.user_agent().parse().unwrap());

        self.client
            .get(format!(
                "https://www.youtube.com/watch?v={id}&bpctr=9999999999&has_verified=1"
            ))
            .headers(headers)
            .send()
            .await?
            .text()
            .await
    }
}
