pub mod ytstream;
pub mod video_data;
pub mod response_data;
pub mod stream;
mod error;

#[cfg(test)]
mod tests {
    use tokio_test::assert_ok;

    use crate::ytstream::YTStream;

    #[tokio::test]
    async fn it_works() {
        let ytstream = assert_ok!(YTStream::new());
        let response = assert_ok!(ytstream.video_data_by_innertube("34Pl2DTuwoQ".to_owned()).await);
    }
}
