use error::Error;
use video_data::VideoData;
use ytstream::YTStream;

mod client_type;
pub mod error;
mod request_data;
mod response_data;
mod serde_impl;
pub mod video_data;
mod youtube_client;
pub mod ytstream;

pub async fn extract_video_data(video_id: &str) -> Result<VideoData, Error> {
    YTStream::new()?.extract(video_id).await
}
