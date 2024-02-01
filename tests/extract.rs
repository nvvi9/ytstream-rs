use tokio_test::assert_ok;
use ytstream_rs::extract_video_data;

#[tokio::test]
async fn extract() {
    let video_data = assert_ok!(extract_video_data("GMd13JPiFFA").await);
    println!("{:#?}", video_data);
}
