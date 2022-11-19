use std::collections::HashMap;

use lambda_runtime::Error;
use reqwest::header::AUTHORIZATION;
use serde::{Serialize, Deserialize};
use crate::config::kook_token;
use super::{publisher::
    {PublishRecord, MatchResult, PlayerStats}, 
    utils::{transform_match_result, transform_lobby_type, transform_game_mode}
};

const CREATE_MESSAGE_ENDPOINT: &str = "https://www.kookapp.cn/api/v3/message/create";
const TOKEN_TYPE: &str = "Bot";

const TEXT_TYPE_KMARKDOWN: &str = "kmarkdown";
const ELEMENT_TYPE_PLAIN_TEXT: &str = "plain-text";
const MODULE_TYPE_SECTION: &str = "section";
const MODULE_TYPE_DIVIDER: &str = "divider";
const MODULE_TYPE_CONTEXT: &str = "context";
const CARD_TYPE_CARD: &str = "card";
const MESSAGE_TYPE_CARD: &str = "10";
const CARD_SIZE_LARGE: &str = "lg";

/// Struct to serialize and deserialize Element of Kook Module
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Element {
    #[serde(rename = "type")]
    element_type: String,
    content: String
}

/// Struct to serialize and deserialize Text of Kook Module
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Text {
    #[serde(rename = "type")]
    text_type: String,
    content: String
}

/// Struct to serialize and deserialize Module of Kook Card
#[derive(Serialize, Deserialize, Debug, Clone)]
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

/// Struct containing data needed to format Header module
pub struct HeaderModuleData<'a> {
    guild_name: &'a String,
    guild_link: &'a String,
    match_id: &'a String,
    match_link: &'a String,
    match_result: &'a String,
    lobby_type: &'a String,
    game_mode: &'a String,
    duration: &'a String
}

/// Struct containing data needed to format Body module
pub struct BodyModuleData<'a> {
    radiant: &'a String,
    dire: &'a String
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
        // Construct HeaderModuleData for data to format Header Module of Kook Card Message
        let header_module_data = HeaderModuleData {
            guild_name: &publish_record.guild_name,
            guild_link: &format!("https://stratz.com/guilds/{}", publish_record.guild_id),
            match_id: &publish_record.match_id,
            match_link: &format!("https://stratz.com/matches/{}", publish_record.match_id),
            match_result: &transform_match_result(&publish_record.match_result),
            lobby_type: &transform_lobby_type(&publish_record.lobby_type),
            game_mode: &transform_game_mode(&publish_record.game_mode),
            duration: &publish_record.duration_field
        };

        // Construct BodyModuleData
        let body_module_data = BodyModuleData {
            radiant: &self.get_players_stats(&publish_record.player_stats_radiant),
            dire: &self.get_players_stats(&publish_record.player_stats_dire)
        };

        // Construct Card of Kook Card Message
        let card = Card {
            card_type: CARD_TYPE_CARD.to_string(),
            theme: self.match_card_theme(&publish_record.match_result),
            size: CARD_SIZE_LARGE.to_string(),
            modules: vec![
                self.get_header_module(&header_module_data), 
                self.get_divider_module(),
                self.get_body_module(&body_module_data),
                self.get_divider_module(),
                self.get_footer_module()
            ]
        };

        // Construct Kook Card Message
        let card_message = CardMessage {
            cards: vec!(card)
        };

        // generate POST request parameters
        let mut params = HashMap::new();
        let serde_card_message = serde_json::to_string(&card_message.cards).unwrap();
        params.insert("type", MESSAGE_TYPE_CARD);
        params.insert("target_id", target_id);
        params.insert("content", &serde_card_message);

        // publish the message
        self.client.post(CREATE_MESSAGE_ENDPOINT)
            .header(AUTHORIZATION, format!("{} {}", TOKEN_TYPE, &kook_token()))
            .json(&params)
            .send()
            .await?;

        Ok(())
    }

    /// Get the content for the Kook Card Message header
    fn get_header_content(&self, data: &HeaderModuleData) -> String {
        let mut header_content = String::new();
        header_content.push_str(
            &format!("[{}]({}) - [{}]({})", data.guild_name, data.guild_link, data.match_id, data.match_link)
        );
        header_content.push_str(
            &format!("**{} - {} - {}** *{}*\n", data.match_result, data.lobby_type, data.game_mode, data.duration) 
        );

        return header_content;
    }

    /// Get the text for the Kook Card Message header
    fn get_header_text(&self, data: &HeaderModuleData) -> Text {
        Text {
            text_type: TEXT_TYPE_KMARKDOWN.to_string(),
            content: self.get_header_content(data)
        }
    }

    /// Get header module from provided data
    /// 
    /// # Arguments
    /// 
    /// * `data` - The data to be formatted inside header module
    /// 
    /// # Examples
    /// 
    /// [SampleGuild](SampleGuildLink) - [SampleMatchName](SampleMatchLink)
    /// **Victory - Unranked - All Pick** *25:51*
    fn get_header_module(
        &self,
        data: &HeaderModuleData
    ) -> Module {
        Module {
            module_type: MODULE_TYPE_SECTION.to_string(),
            text: Some(self.get_header_text(data)),
            elements: Vec::new()
        }
    }

    /// Get the content for the Kook Card Message body
    fn get_body_content(&self, data: &BodyModuleData) -> String {
        let mut body_content = String::new();
        if data.radiant.len() > 0 {
            body_content.push_str(&format!("**Radiant**\n{}", data.radiant));
        }
        if data.dire.len() > 0 {
            body_content.push_str(&format!("**Dire**\n{}", data.dire));
        }

        return body_content;
    }

    // Get the text for the Kook Card Message body
    fn get_body_text(&self, data: &BodyModuleData) -> Text {
        Text {
            text_type: TEXT_TYPE_KMARKDOWN.to_string(),
            content: self.get_body_content(data)
        }
    }

    /// Get body module from provided data
    /// 
    /// # Arguments
    /// 
    /// * `radiant_field` - The radiant players data in String literal
    /// * `dire_field` - The dire players data in String literal
    /// 
    /// # Examples
    /// 
    /// **Dire**
    /// Player1 - Rubick - [6/5/16]
    /// Player2 - Shadow Fiend - [16/4/8]
    fn get_body_module(&self, data: &BodyModuleData) -> Module {
        Module {
            module_type: MODULE_TYPE_SECTION.to_string(),
            text: Some(self.get_body_text(data)),
            elements: Vec::new()
        }
    }

    /// Get a horizontal divider line in Kook Card Message
    fn get_divider_module(&self) -> Module {
        Module {
            module_type: MODULE_TYPE_DIVIDER.to_string(),
            text: None,
            elements: Vec::new()
        }
    }

    /// Get a Stratz appriciation text as a Kook Card Message footer element
    fn get_stratz_appriciation_element(&self) -> Element {
        Element {
            element_type: ELEMENT_TYPE_PLAIN_TEXT.to_string(),
            content: "Powered by Stratz".to_string()
        }
    }

    /// Get footer module
    fn get_footer_module(&self) -> Module {
        Module {
            module_type: MODULE_TYPE_CONTEXT.to_string(),
            text: None,
            elements: vec![self.get_stratz_appriciation_element()]
        }
    }

    /// Match MatchResult Struct to String literal
    /// 
    /// # Arguments
    /// 
    /// * `match_result` - a MatchResult struct to be transformed
    fn match_card_theme(&self, match_result: &MatchResult) -> String {
        let card_theme = match match_result {
            MatchResult::Victory => "success",
            MatchResult::Defeat => "danger",
            MatchResult::Both => "warning",
            MatchResult::None => "none"
        };

        return card_theme.to_string();
    }

    /// Get players stats in string literal
    /// 
    /// # Arguments
    /// 
    /// * `players_stats` - A vector of PlayerStats containing stats of a player
    fn get_players_stats(&self, players_stats: &Vec<PlayerStats>) -> String {
        let mut result = String::new();
        for player_stats in players_stats.iter() {
            let line = format!(
                "{} - {} - [{}/{}/{}]\n",
                &player_stats.name, player_stats.hero_display_name, player_stats.kills, player_stats.deaths, player_stats.assists
            );

            result.push_str(&line);
        }

        return result;
    }

}
