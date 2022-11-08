use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue};

use crate::config;

const BASE_URL: &str = "https://www.kookapp.cn/";
const AUTHORIZATION_HEADER: &str = "Authorization";
const TOKEN_TYPE: &str = "Bot";
const SEND_MESSAGE_ENDPOINT: &str = "/api/v3/message/create";

struct KookClient {
    token: String,
    client: reqwest::Client
}

impl KookClient {

    pub fn new(token: String) -> Self {
        Self { token, client: reqwest::Client::new() }
    }

    pub async fn publish_message(
        &self, 
        option_message_type: Option<i32>, 
        target_id: &str, 
        content: &str
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}{}", BASE_URL, SEND_MESSAGE_ENDPOINT);

        let mut headers = HeaderMap::new();
        let authorization_value = format!("{} {}", TOKEN_TYPE, &config::kook_token());
        headers.insert(AUTHORIZATION_HEADER, HeaderValue::from_str(authorization_value.as_str()).unwrap());

        let mut params = HashMap::new();
        let message_type = match option_message_type {
            Some(x) => x.to_string(),
            None => 1.to_string()
        };
        params.insert("type", message_type.as_str());
        params.insert("target_id", target_id);
        params.insert("content", content);

        let response = self.client.post(url)
            .headers(headers)
            .json(&params)
            .send()
            .await?;
            
        return Ok(());
    }

}
