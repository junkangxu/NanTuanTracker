use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue};

use crate::config;

const BASE_URL: &str = "https://www.kookapp.cn/";
const AUTHORIZATION_HEADER: &str = "Authorization";
const TOKEN_TYPE: &str = "Bot";
const SEND_MESSAGE_ENDPOINT: &str = "/api/v3/message/create";

pub struct KookClient {
    token: String,
    client: reqwest::Client
}

pub enum MessageType {
    TEXT,
    KMARKDOWN,
    CARD
}

impl MessageType {
    fn to_value(&self) -> &str {
        match self {
            Self::TEXT => "1",
            Self::KMARKDOWN => "9",
            Self::CARD => "10"
        }
    }
}

impl KookClient {

    pub fn new(token: String) -> Self {
        Self { token, client: reqwest::Client::new() }
    }

    pub async fn publish_message(
        &self, 
        message_type: MessageType, 
        target_id: &str, 
        content: &str
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}{}", BASE_URL, SEND_MESSAGE_ENDPOINT);

        let mut headers = HeaderMap::new();
        let authorization_value = format!("{} {}", TOKEN_TYPE, &config::kook_token());
        headers.insert(AUTHORIZATION_HEADER, HeaderValue::from_str(authorization_value.as_str()).unwrap());

        let mut params = HashMap::new();
        let message_type = message_type.to_value();
        params.insert("type", message_type);
        params.insert("target_id", target_id);
        params.insert("content", content);

        let response = self.client.post(url)
            .headers(headers)
            .json(&params)
            .send()
            .await?;
            
        return Ok(response);
    }



}
