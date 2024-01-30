use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponseData {
    captions: Option<Captions>,
    playability_status: PlayabilityStatus,
    streaming_data: StreamingData,
    video_details: VideoDetails,
    microformat: Option<Microformat>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Captions {
    player_captions_tracklist_renderer: PlayerCaptionsTracklistRenderer,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCaptionsTracklistRenderer {
    caption_tracks: Vec<CaptionTrack>,
    audio_tracks: Vec<AudioTrack>,
    translation_languages: Vec<TranslationLanguage>,
    default_audio_track_index: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptionTrack {
    base_url: String,
    name: Text,
    vss_id: String,
    language_code: String,
    kind: String,
    is_translatable: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrack {
    caption_track_indices: Vec<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationLanguage {
    language_code: String,
    language_name: Text,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Microformat {
    player_microformat_renderer: PlayerMicroformatRenderer,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMicroformatRenderer {
    thumbnail: Thumbnail,
    title: Text,
    description: Text,
    length_seconds: String,
    owner_profile_url: String,
    external_channel_id: String,
    is_family_safe: bool,
    available_countries: Vec<String>,
    is_unlisted: bool,
    has_ypc_metadata: bool,
    view_count: String,
    category: String,
    publish_date: String,
    owner_channel_name: String,
    upload_data: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    simple_text: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayabilityStatus {
    status: String,
    reason: Option<String>,
    playable_in_embed: bool,
    miniplayer: Miniplayer,
    context_params: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Miniplayer {
    miniplayer_renderer: MiniplayerRenderer,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniplayerRenderer {
    playback_mode: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails {
    video_id: String,
    title: String,
    length_seconds: String,
    keywords: Vec<String>,
    channel_id: String,
    is_owner_viewing: bool,
    short_description: String,
    is_crawlable: bool,
    thumbnail: Thumbnails,
    average_rating: Option<f64>,
    allow_ratings: bool,
    view_count: String,
    author: String,
    is_private: bool,
    is_unplugged_corpus: bool,
    is_live_content: bool,
}

#[derive(Deserialize)]
pub struct Thumbnails {
    thumbnails: Vec<Thumbnail>,
}

#[derive(Deserialize)]
pub struct Thumbnail {
    url: String,
    width: u32,
    height: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    expires_in_seconds: String,
    formats: Vec<Format>,
    adaptive_formats: Vec<Format>,
    dash_manifest_url: Option<String>,
    hls_manifest_url: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    itag: i32,
    url: String,
    mime_type: String,
    quality: String,
    signature_cipher: Option<String>,
    bitrate: i32,
    fps: Option<i32>,
    width: Option<i32>,
    height: Option<i32>,
    last_modified: String,
    content_length: Option<String>,
    quality_label: Option<String>,
    projection_type: String,
    average_bitrate: Option<i32>,
    audio_quality: Option<String>,
    approx_duration_ms: String,
    audio_sample_rate: Option<String>,
    audio_channels: Option<i32>,
    init_range: Option<InitRange>,
    index_range: Option<IndexRange>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitRange {
    start: String,
    end: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexRange {
    start: String,
    end: String,
}