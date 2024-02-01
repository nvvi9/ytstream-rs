use std::sync::Arc;

use reqwest::cookie::Jar;
use reqwest::header::HeaderMap;
use reqwest::{header, Client, Request};
use serde::Serialize;

use crate::error::Error;
use crate::response_data::PlayerResponseData;
use crate::video_data::VideoData;

const PLAYER_PARAMS: &str = "CgIQBg==";

pub struct YTStream {
    client: Client,
}

enum ClientType {
    Android,
    Embedded,
}

impl ClientType {
    fn name(&self) -> &str {
        match self {
            ClientType::Android => "ANDROID",
            ClientType::Embedded => "WEB_EMBEDDED_PLAYER",
        }
    }

    fn version(&self) -> &str {
        match self {
            ClientType::Android => "17.31.35",
            ClientType::Embedded => "1.19700101",
        }
    }

    fn key(&self) -> &str {
        match self {
            ClientType::Android => "AIzaSyA8eiZmM1FaDVjRy-df2KTyQ_vz_yYM39w",
            ClientType::Embedded => "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8",
        }
    }

    fn user_agent(&self) -> &str {
        match self {
            ClientType::Android => "com.google.android.youtube/17.31.35 (Linux; U; Android 11) gzip",
            ClientType::Embedded => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
        }
    }

    fn android_sdk_version(&self) -> Option<u32> {
        match self {
            ClientType::Android => Some(30),
            ClientType::Embedded => None,
        }
    }
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InnertubeRequest {
    video_id: Option<String>,
    browse_id: Option<String>,
    continuation: Option<String>,
    context: InnertubeContext,
    playback_context: Option<PlaybackContext>,
    content_check_ok: Option<bool>,
    racy_check_ok: Option<bool>,
    params: String,
}

impl InnertubeRequest {
    fn video_data_request(video_id: &str, client_type: &ClientType) -> Self {
        InnertubeRequest {
            video_id: Some(video_id.to_string()),
            browse_id: None,
            continuation: None,
            context: InnertubeContext::from_client_type(client_type),
            playback_context: Some(PlaybackContext::default()),
            content_check_ok: Some(true),
            racy_check_ok: Some(true),
            params: PLAYER_PARAMS.to_string(),
        }
    }
}

#[derive(Serialize)]
struct InnertubeContext {
    client: InnertubeClient,
}

impl InnertubeContext {
    fn from_client_type(client_type: &ClientType) -> Self {
        Self {
            client: InnertubeClient::from_client_type(client_type),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InnertubeClient {
    hl: String,
    gl: String,
    time_zone: String,
    client_name: String,
    client_version: String,
    android_sdk_version: Option<u32>,
    user_agent: String,
}

impl InnertubeClient {
    fn from_client_type(client_type: &ClientType) -> Self {
        Self {
            hl: "en".to_string(),
            gl: "US".to_string(),
            time_zone: "UTC".to_string(),
            client_name: client_type.name().to_string(),
            client_version: client_type.version().to_string(),
            android_sdk_version: client_type.android_sdk_version(),
            user_agent: client_type.user_agent().to_string(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PlaybackContext {
    content_playback_context: ContentPlaybackContext,
}

impl Default for PlaybackContext {
    fn default() -> Self {
        PlaybackContext {
            content_playback_context: ContentPlaybackContext {
                html5preference: "HTML5_PREF_WANTS".to_string(),
            },
        }
    }
}

#[derive(Serialize)]
struct ContentPlaybackContext {
    #[serde(rename = "html5Preference")]
    html5preference: String,
}
