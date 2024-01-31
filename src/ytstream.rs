use std::sync::Arc;

use reqwest::{Client, header};
use reqwest::cookie::Jar;
use reqwest::header::HeaderMap;
use serde::Serialize;

use crate::error::Error;
use crate::response_data::PlayerResponseData;
use crate::video_data::VideoData;

const USER_AGENT: &str = "com.google.android.youtube/17.31.35 (Linux; U; Android 11) gzip";
const CLIENT_NAME: &str = "ANDROID";
const CLIENT_VERSION: &str = "17.31.35";
const CLIENT_KEY: &str = "AIzaSyA8eiZmM1FaDVjRy-df2KTyQ_vz_yYM39w";
const ANDROID_SDK_VERSION: u32 = 30;
const PLAYER_PARAMS: &str = "CgIQBg==";

pub struct YTStream {
    client: Client,
}

impl YTStream {
    pub fn new() -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(header::USER_AGENT, USER_AGENT.parse().unwrap());
        headers.insert(header::ORIGIN, "https://youtube.com".parse().unwrap());
        headers.insert("Sec-Fetch-Mode", "navigate".parse().unwrap());
        headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".parse().unwrap());
        headers.insert("X-Youtube-Client-Name", "3".parse().unwrap());
        headers.insert("X-Youtube-Client-Version", CLIENT_VERSION.parse().unwrap());

        let cookie = "CONSENT=YES+; Path=/; Domain=.youtube.com;";
        let cookie_jar = Jar::default();
        cookie_jar.add_cookie_str(cookie, &"https://youtube.com".parse().unwrap());

        let client = Client::builder()
            .default_headers(headers)
            .cookie_provider(Arc::new(cookie_jar))
            .user_agent(USER_AGENT)
            .build()
            .map_err(|e| Error::Request(e))?;

        Ok(Self { client })
    }

    pub async fn extract(self, id: String) -> Result<VideoData, Error> {
        let player_response = self.video_data_by_innertube(id).await?;
        VideoData::from_player_response_data(player_response)
    }

    pub async fn video_data_by_innertube(&self, id: String) -> Result<PlayerResponseData, Error> {
        let request = InnertubeRequest::video_data_request(id);

        self.client.post("https://www.youtube.com/youtubei/v1/player?key=".to_owned() + CLIENT_KEY)
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::Request(e))?
            .json::<PlayerResponseData>()
            .await
            .map_err(|e| Error::Request(e))
    }

    pub async fn video_data_by_innertube_raw(&self, id: String) -> Result<String, Error> {
        let request = InnertubeRequest::video_data_request(id);

        self.client.post("https://www.youtube.com/youtubei/v1/player?key=".to_owned() + CLIENT_KEY)
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::Request(e))?
            .text()
            .await
            .map_err(|e| Error::Request(e))
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
    fn video_data_request(video_id: String) -> Self {
        InnertubeRequest {
            video_id: Some(video_id),
            browse_id: None,
            continuation: None,
            context: InnertubeContext::default(),
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

impl Default for InnertubeContext {
    fn default() -> Self {
        InnertubeContext {
            client: InnertubeClient::default()
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InnertubeClient {
    hl: String,
    gl: String,
    client_name: String,
    client_version: String,
    android_sdk_version: u32,
    user_agent: String,
    time_zone: String,
}

impl Default for InnertubeClient {
    fn default() -> Self {
        InnertubeClient {
            hl: "en".to_string(),
            gl: "US".to_string(),
            time_zone: "UTC".to_string(),
            client_name: CLIENT_NAME.to_string(),
            client_version: CLIENT_VERSION.to_string(),
            android_sdk_version: ANDROID_SDK_VERSION,
            user_agent: USER_AGENT.to_string(),
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
                html5preference: "HTML5_PREF_WANTS".to_string()
            }
        }
    }
}

#[derive(Serialize)]
struct ContentPlaybackContext {
    #[serde(rename = "html5Preference")]
    html5preference: String,
}