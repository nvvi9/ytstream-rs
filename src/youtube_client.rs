use std::sync::Arc;

use reqwest::{cookie::Jar, header::{self, HeaderMap}, Client};

use crate::{client_type::ClientType, error::Error, request_data::YoutubePlayerRequest, response_data::PlayerResponseData};

pub struct YoutubeClient {
    client: Client,
    client_type: ClientType,
}

impl YoutubeClient {
    pub fn new(client_type: ClientType) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(header::ORIGIN, "https://youtube.com".parse().unwrap());
        headers.insert("Sec-Fetch-Mode", "navigate".parse().unwrap());
        headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            header::ACCEPT,
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"
                .parse()
                .unwrap(),
        );
        headers.insert("X-Youtube-Client-Name", "3".parse().unwrap());

        let cookie = "CONSENT=YES+; Path=/; Domain=.youtube.com;";
        let cookie_jar = Jar::default();
        cookie_jar.add_cookie_str(cookie, &"https://youtube.com".parse().unwrap());

        let client = Client::builder()
            .default_headers(headers)
            .user_agent(client_type.user_agent())
            .cookie_provider(Arc::new(cookie_jar))
            .build()
            .map_err(|e| Error::Request(e))?;

        Ok(Self {
            client,
            client_type,
        })
    }

    pub async fn player_response_data(
        &self,
        id: &str,
    ) -> Result<PlayerResponseData, reqwest::Error> {
        let request = YoutubePlayerRequest::video_data_request(id, &self.client_type);

        let mut headers = HeaderMap::new();
        headers.append(
            "X-Youtube-Client-Version",
            self.client_type.version().parse().unwrap(),
        );

        self.client
            .post(format!(
                "https://www.youtube.com/youtubei/v1/player?key={}",
                self.client_type.key()
            ))
            .headers(headers)
            .json(&request)
            .send()
            .await?
            .json::<PlayerResponseData>()
            .await
    }

    pub async fn video_page(&self, id: &str) -> Result<String, reqwest::Error> {
        self.client
            .get(format!(
                "https://www.youtube.com/watch?v={id}&bpctr=9999999999&has_verified=1"
            ))
            .send()
            .await?
            .text()
            .await
    }
}
