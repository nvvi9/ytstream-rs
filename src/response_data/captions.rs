use serde::Deserialize;

use crate::response_data::Text;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Captions {
    pub(crate) player_captions_tracklist_renderer: PlayerCaptionsTracklistRenderer,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCaptionsTracklistRenderer {
    pub(crate) caption_tracks: Vec<CaptionTrack>,
    pub(crate) audio_tracks: Vec<AudioTrack>,
    pub(crate) translation_languages: Vec<TranslationLanguage>,
    pub(crate) default_audio_track_index: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptionTrack {
    pub(crate) base_url: String,
    pub(crate) name: Text,
    pub(crate) vss_id: String,
    pub(crate) language_code: String,
    pub(crate) kind: String,
    pub(crate) is_translatable: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrack {
    pub(crate) caption_track_indices: Vec<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationLanguage {
    pub(crate) language_code: String,
    pub(crate) language_name: Text,
}
