use crate::client_type::ClientType;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InnertubeRequest {
    pub video_id: Option<String>,
    pub browse_id: Option<String>,
    pub continuation: Option<String>,
    pub context: InnertubeContext,
    pub playback_context: Option<PlaybackContext>,
    pub content_check_ok: Option<bool>,
    pub racy_check_ok: Option<bool>,
    pub params: String,
}

impl InnertubeRequest {
    pub fn video_data_request(video_id: &str, client_type: &ClientType) -> Self {
        InnertubeRequest {
            video_id: Some(video_id.to_string()),
            browse_id: None,
            continuation: None,
            context: InnertubeContext::from_client_type(client_type),
            playback_context: Some(PlaybackContext::default()),
            content_check_ok: Some(true),
            racy_check_ok: Some(true),
            params: "CgIQBg==".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct InnertubeContext {
    pub client: InnertubeClient,
}

impl InnertubeContext {
    pub fn from_client_type(client_type: &ClientType) -> Self {
        Self {
            client: InnertubeClient::from_client_type(client_type),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InnertubeClient {
    pub hl: String,
    pub gl: String,
    pub time_zone: String,
    pub client_name: String,
    pub client_version: String,
    pub android_sdk_version: Option<u32>,
    pub user_agent: String,
}

impl InnertubeClient {
    pub fn from_client_type(client_type: &ClientType) -> Self {
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
pub struct PlaybackContext {
    pub content_playback_context: ContentPlaybackContext,
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
pub struct ContentPlaybackContext {
    #[serde(rename = "html5Preference")]
    pub html5preference: String,
}
