use crate::response_data::Text;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Captions {
    pub player_captions_tracklist_renderer: PlayerCaptionsTracklistRenderer,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCaptionsTracklistRenderer {
    pub caption_tracks: Vec<CaptionTrack>,
    pub audio_tracks: Vec<AudioTrack>,
    pub translation_languages: Option<Vec<TranslationLanguage>>,
    pub default_audio_track_index: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptionTrack {
    pub base_url: String,
    pub name: Text,
    pub vss_id: String,
    pub language_code: String,
    pub kind: Option<String>,
    pub is_translatable: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrack {
    pub caption_track_indices: Vec<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationLanguage {
    pub language_code: String,
    pub language_name: Text,
}
