use std::collections::HashMap;
use lambda_runtime::Error;
use crate::{provider::stratz, config};

use super::{kook::KookPublisher, webhook::WebhookPublisher};

const MINIMUM_PLAYERS: usize = 1;
const RADIANT: &str = "Radiant";
const DIRE: &str = "Dire";
const KOOK_TARGET_ID: &str = "3193188266865676";

/// Enum to match Match Result
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MatchResult {
    None,
    Victory,
    Defeat,
    Both
}

/// Struct to contain Player stats of a match
pub struct PlayerStats {
    pub hero_id: i16,
    pub hero_display_name: String,
    pub name: String,
    pub kills: u8,
    pub deaths: u8,
    pub assists: u8
}

/// Struct to contain data to be published
pub struct PublishRecord {
    pub match_id: String,
    pub guild_id: String,
    pub guild_name: String,
    pub guild_logo: String,
    pub match_result: MatchResult,
    pub lobby_type: stratz::api::LobbyType,
    pub game_mode: stratz::api::GameMode,
    pub player_stats_radiant: Vec<PlayerStats>,
    pub player_stats_dire: Vec<PlayerStats>,
    pub duration_field: String,
    pub end: chrono::DateTime<chrono::Utc>
}

/// Struct for the entry point of publishers
pub struct Publisher;

impl Publisher {

    /// Extract useful information to PublishRecord and pass it to different publishers, such as
    /// - Discord Webhook
    /// - Kook Bot
    /// 
    /// # Arguments
    /// 
    /// - `guild_id` - The id of Dota2 guild
    /// - `guild_name` - The name of Dota2 guild
    /// - `guild_logo` - The logo url of Dota2 guild
    /// - `guild_match` - The match result of a Dota2 match
    pub async fn publish(
        guild_id: i64,
        guild_name: &str,
        guild_logo: &str,
        guild_match: stratz::api::Match
    ) -> Result<(), Error> {
        let match_id = guild_match.id.unwrap();
        let players = guild_match.players.unwrap();
        if players.len() < MINIMUM_PLAYERS {
            return Ok(());
        }
    
        let match_result = get_match_result(&players).unwrap();
        let lobby_type = guild_match.lobby_type.unwrap();
        let game_mode = guild_match.game_mode.unwrap();
    
        let duration_num = guild_match.duration_seconds.unwrap();
        let duration = chrono::Duration::seconds(duration_num);
    
        let end_num = i64::try_from(guild_match.end_date_time.unwrap()).unwrap();
        let end_naive_date_time = chrono::NaiveDateTime::from_timestamp(end_num, 0);
        let end = chrono::DateTime::<chrono::Utc>::from_utc(end_naive_date_time, chrono::Utc);
        
        let mins = duration.num_seconds() / 60;
        let secs = duration.num_seconds() % 60;
        let duration_field = format!("{}:{:02}", &mins, &secs);
    
        let players_by_team = get_players_by_team(&players).unwrap();
        let radiant_players = players_by_team.get(&RADIANT.to_string()).unwrap();
        let mut radiant_player_stats = Vec::new();
        for player in radiant_players.iter() {
            radiant_player_stats.push(get_player_stats(player).unwrap())
        }
    
        let dire_players = players_by_team.get(&DIRE.to_string()).unwrap();
        let mut dire_player_stats = Vec::new();
        for player in dire_players.iter() {
            dire_player_stats.push(get_player_stats(player).unwrap())
        }
        
        let publish_record = PublishRecord {
            match_id: match_id.to_string(),
            guild_id: guild_id.to_string(),
            guild_name: guild_name.to_string(),
            guild_logo: guild_logo.to_string(),
            match_result,
            lobby_type,
            game_mode,
            player_stats_radiant: radiant_player_stats,
            player_stats_dire: dire_player_stats,
            duration_field,
            end
        };
    
        let kook_publisher = KookPublisher {
            client: reqwest::Client::new()
        };
        kook_publisher.publish(&KOOK_TARGET_ID, &publish_record).await?;
    
        let webhook_publisher = WebhookPublisher {
            client: webhook::client::WebhookClient::new(&config::discord_webhook_url())
        };
        webhook_publisher.publish(&publish_record).await?;

        Ok(())
    }
}

/// Transform Stratz MatchResult to Rust tuples
/// 
/// # Arguments
/// 
/// * `players` - The vector of players in the match
fn get_match_result(players: &Vec<Option<stratz::api::Player>>) -> Result<MatchResult, Error> {
    let mut is_victory = false;
    let mut is_defeat = false;
    for player in players.iter() {
        let player = player.clone().unwrap();
        let player_result = player.is_victory.unwrap();
        match player_result {
            true => is_victory = true,
            false => is_defeat = true,
        }
    }

    match (is_victory, is_defeat) {
        (false, false) => Ok(MatchResult::None),
        (true, false) => Ok(MatchResult::Victory),
        (false, true) => Ok(MatchResult::Defeat),
        (true, true) => Ok(MatchResult::Both)
    }
}

/// Match players based on their team. The return value will be a HashMap with hash key Radiant and Dire. And the
/// corresponding values are players on each side.
/// 
/// # Arguments
/// 
/// * `players` - The vector of players in the match
fn get_players_by_team(players: &Vec<Option<stratz::api::Player>>) -> Result<HashMap<String, Vec<stratz::api::Player>>, Error> {
    let mut radiant_players = Vec::new();
    let mut dire_players = Vec::new();
    for player in players.iter() {
        let player = player.clone().unwrap();
        let player_team = player.is_radiant.unwrap();
        match player_team {
            true => radiant_players.push(player),
            false => dire_players.push(player)
        }
    }

    Ok(HashMap::from(
        [
            (RADIANT.to_string(), radiant_players), 
            (DIRE.to_string(), dire_players)
        ]
    ))
}

/// Extract data from Stratz API player struct and return as a PlayerStats struct.
/// 
/// # Arguments
/// 
/// * `player` - The player struct from Stratz API
fn get_player_stats(player: &stratz::api::Player) -> Result<PlayerStats, Error> {
    let hero = player.hero.as_ref().unwrap();

    Ok(PlayerStats { 
        hero_id: hero.id.unwrap(),
        hero_display_name: hero.display_name.as_ref().unwrap().to_string(),
        name: player.steam_account.as_ref().unwrap().name.as_ref().unwrap().to_string(),
        kills: player.kills.unwrap(),
        deaths: player.deaths.unwrap(),
        assists: player.assists.unwrap()
    })
}
