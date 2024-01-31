#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    CipherNotFound,
    SignatureTimestampNotFound,
    NotPlayableInEmbed,
    LoginRequired,
    VideoPrivate,
    PlayabilityStatus { reason: String },
}
