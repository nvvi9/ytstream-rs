use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayabilityStatus {
    Ok,
    Unplayable {
        reason: String,
        playable_in_embed: bool,
    },
    LoginRequired {
        reason: String
    },
    LiveStreamOffline {
        reason: String,
        playable_in_embed: bool,
    },
    Error {
        reason: String
    },
}
