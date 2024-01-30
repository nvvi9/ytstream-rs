use std::time::{Duration, Instant};
use crate::stream::Stream;

pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub channel: String,
    pub channel_id: String,
    pub view_count: u32,
    pub duration: Duration,
    pub publish_date: Instant,
    pub streams: Vec<Stream>,
}