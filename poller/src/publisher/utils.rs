use crate::provider::stratz;

use super::publisher::MatchResult;

pub fn transform_match_result(match_result: &MatchResult) -> String {
    let result = match match_result {
        MatchResult::None => "Cancelled",
        MatchResult::Victory => "Victory",
        MatchResult::Defeat => "Defeat",
        MatchResult::Both => "Clash"
    };

    result.to_string()
}

pub fn transform_lobby_type(lobby_type: &stratz::api::LobbyType) -> String {
    let result = match lobby_type {
        stratz::api::LobbyType::UNRANKED => "Unranked",
        stratz::api::LobbyType::PRACTICE => "Lobby",
        stratz::api::LobbyType::TOURNAMENT => "The International",
        stratz::api::LobbyType::TUTORIAL => "Tutorial",
        stratz::api::LobbyType::COOP_VS_BOTS => "Bots",
        stratz::api::LobbyType::TEAM_MATCH => "Guild",
        stratz::api::LobbyType::SOLO_QUEUE => "Solo Ranked",
        stratz::api::LobbyType::RANKED => "Ranked",
        stratz::api::LobbyType::SOLO_MID => "Duel",
        stratz::api::LobbyType::BATTLE_CUP => "Battle Cup",
        stratz::api::LobbyType::EVENT => "Event",
        _ => "Unknown",
    };

    result.to_string()
}

pub fn transform_game_mode(game_mode: &stratz::api::GameMode) -> String {
    let result = match game_mode {
        stratz::api::GameMode::NONE => "None",
        stratz::api::GameMode::ALL_PICK => "All Pick",
        stratz::api::GameMode::CAPTAINS_MODE => "Captains Mode",
        stratz::api::GameMode::RANDOM_DRAFT => "Random Draft",
        stratz::api::GameMode::SINGLE_DRAFT => "Single Draft",
        stratz::api::GameMode::ALL_RANDOM => "All Random",
        stratz::api::GameMode::INTRO => "Intro",
        stratz::api::GameMode::THE_DIRETIDE => "Diretide",
        stratz::api::GameMode::REVERSE_CAPTAINS_MODE => "Reverse Captains Mode",
        stratz::api::GameMode::THE_GREEVILING => "Greeviling",
        stratz::api::GameMode::TUTORIAL => "Tutorial",
        stratz::api::GameMode::MID_ONLY => "Mid Only",
        stratz::api::GameMode::LEAST_PLAYED => "Least Played",
        stratz::api::GameMode::NEW_PLAYER_POOL => "Limited Heroes",
        stratz::api::GameMode::COMPENDIUM_MATCHMAKING => "Compendium",
        stratz::api::GameMode::CUSTOM => "Custom",
        stratz::api::GameMode::CAPTAINS_DRAFT => "Captains Draft",
        stratz::api::GameMode::BALANCED_DRAFT => "Balanced Draft",
        stratz::api::GameMode::ABILITY_DRAFT => "Ability Draft",
        stratz::api::GameMode::EVENT => "Event",
        stratz::api::GameMode::ALL_RANDOM_DEATH_MATCH => "All Random Deathmatch",
        stratz::api::GameMode::SOLO_MID => "Solo Mid",
        stratz::api::GameMode::ALL_PICK_RANKED => "All Draft",
        stratz::api::GameMode::TURBO => "Turbo",
        stratz::api::GameMode::MUTATION => "Mutation",
        _ => "Unknown",
    };

    result.to_string()
}
