use std::collections::HashMap;

use lambda_runtime::Error;
use reqwest::header::AUTHORIZATION;
use serde::{Serialize, Deserialize};
use crate::config::kook_token;
use super::{publisher::
    {PublishRecord, MatchResult}, 
    utils::{transform_match_result, transform_lobby_type, transform_game_mode}
};

const CREATE_MESSAGE_ENDPOINT: &str = "https://www.kookapp.cn/api/v3/message/create";
const TOKEN_TYPE: &str = "Bot";

/// Struct to serialize and deserialize Element of Kook Module
#[derive(Serialize, Deserialize, Debug)]
struct Element {
    #[serde(rename = "type")]
    element_type: String,
    content: String
}

/// Struct to serialize and deserialize Text of Kook Module
#[derive(Serialize, Deserialize, Debug)]
struct Text {
    #[serde(rename = "type")]
    text_type: String,
    content: String
}

/// Struct to serialize and deserialize Module of Kook Card
#[derive(Serialize, Deserialize, Debug)]
struct Module {
    #[serde(rename = "type")]
    module_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Text>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    elements: Vec<Element>
}

/// Struct to serialize and deserialize Card of Kook CardMessage
#[derive(Serialize, Deserialize, Debug)]
struct Card {
    #[serde(rename = "type")]
    card_type: String,
    theme: String,
    size: String,
    modules: Vec<Module>
}

/// Struct to serialize and deserialize Kook CardMessage
#[derive(Serialize, Deserialize, Debug)]
struct CardMessage {
    pub cards: Vec<Card>
}

/// Kook Publisher
pub struct KookPublisher {
    pub client: reqwest::Client
}

impl KookPublisher {

    /// Format `publish_record` and publish the formatted data to Kook
    ///
    /// # Arguments
    /// 
    /// * `target_id` - The id of target, a.k.a the id of channel
    /// * `publish_record` - The data POJO to be published
    pub async fn publish(&self, target_id: &str, publish_record: &PublishRecord) -> Result<(), Error> {
        let guild_link = format!("https://stratz.com/guilds/{}", publish_record.guild_id);
        let match_link = format!("https://stratz.com/matches/{}", publish_record.match_id);
        let match_result = transform_match_result(&publish_record.match_result);
        let lobby_type = transform_lobby_type(&publish_record.lobby_type);
        let game_mode = transform_game_mode(&publish_record.game_mode);

        let mut radiant_field = String::new();
        for player_stats in publish_record.player_stats_radiant.iter() {
            let line = format!("**{}** *{}* [{}/{}/{}]\n", 
                &player_stats.name, player_stats.hero_display_name, player_stats.kills, player_stats.deaths, player_stats.assists
            );
            radiant_field.push_str(&line);
        }

        let mut dire_field = String::new();
        for player_stats in publish_record.player_stats_dire.iter() {
            let line = format!("**{}** *{}* [{}/{}/{}]\n", 
                &player_stats.name, player_stats.hero_display_name, player_stats.kills, player_stats.deaths, player_stats.assists
            );
            dire_field.push_str(&line);
        }

        let mut content = String::new();
        content.push_str(
            &format!("[{}]({}) - [{}]({})\n",
                publish_record.guild_name, guild_link, publish_record.match_id, match_link
            )
        );
        content.push_str(
            &format!("**{} - {} - {}** *{}*\n",
                match_result, lobby_type, game_mode, publish_record.duration_field
            )
        );

        if radiant_field.len() > 0 {
            content.push_str(&format!("(ins)Radiant(ins)\n{}", radiant_field));
        }

        if dire_field.len() > 0 {
            content.push_str(&format!("(ins)Dire(ins)\n{}", dire_field));
        }

        let text = Text {
            text_type: "kmarkdown".to_string(),
            content: content.clone()
        };

        let first_module = Module {
            module_type: "section".to_string(),
            text: Some(text),
            elements: Vec::new()
        };

        let second_module = Module {
            module_type: "divider".to_string(),
            text: None,
            elements: Vec::new()
        };

        let element = Element {
            element_type: "plain-text".to_string(),
            content: "Powered by STRATZ".to_string()
        };

        let third_module = Module {
            module_type: "context".to_string(),
            text: None,
            elements: vec!(element)
        };

        let theme = match publish_record.match_result {
            MatchResult::Victory => "success".to_string(),
            MatchResult::Defeat => "danger".to_string(),
            MatchResult::Both => "warning".to_string(),
            MatchResult::None => "none".to_string()
        };

        let card = Card {
            card_type: "card".to_string(),
            theme,
            size: "sm".to_string(),
            modules: vec![first_module, second_module, third_module]
        };

        let card_message = CardMessage {
            cards: vec!(card)
        };

        let mut params = HashMap::new();
        let serde_card_message = serde_json::to_string(&card_message.cards).unwrap();
        params.insert("type", "10");
        params.insert("target_id", target_id);
        params.insert("content", &serde_card_message);

        self.client.post(CREATE_MESSAGE_ENDPOINT)
            .header(AUTHORIZATION, format!("{} {}", TOKEN_TYPE, &kook_token()))
            .json(&params)
            .send()
            .await?;

        Ok(())
    }

}

