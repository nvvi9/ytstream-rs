pub mod ytstream;
pub mod video_data;
pub mod response_data;
pub mod stream;
mod error;
mod serde_impl;

#[cfg(test)]
mod tests {
    use tokio_test::assert_ok;

    use crate::ytstream::YTStream;

    #[tokio::test]
    async fn it_works() {
        let ytstream = assert_ok!(YTStream::new());
        let response = assert_ok!(ytstream.video_data_by_innertube_raw("34Pl2DTuwoQ".to_owned()).await);
        println!("{}", response);
    }

    #[tokio::test]
    async fn extract() {
        let ytstream = assert_ok!(YTStream::new());
        let video_data = assert_ok!(ytstream.extract("34Pl2DTuwoQ".to_owned()).await);
    }
}
