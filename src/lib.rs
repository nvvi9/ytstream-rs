pub mod error;
pub mod response_data;
pub mod serde_impl;
pub mod stream;
pub mod video_data;
pub mod ytstream;

#[cfg(test)]
mod tests {
    use tokio_test::assert_ok;

    use crate::ytstream::YTStream;

    #[tokio::test]
    async fn extract() {
        let ytstream = assert_ok!(YTStream::new());
        let video_data = assert_ok!(ytstream.extract("GMd13JPiFFA").await);
        println!("{:#?}", video_data);
        let urls = video_data
            .streams
            .iter()
            .map(|s| s.url.as_str())
            .collect::<Vec<&str>>();
        println!("{:#?}", urls);
    }
}
