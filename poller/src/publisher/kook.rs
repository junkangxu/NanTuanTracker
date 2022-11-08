use std::collections::HashMap;

use lambda_runtime::Error;
use crate::{client::kook::{KookClient, MessageType}, provider::stratz};

use crate::errors::{PollerError, ParserError};

const MINIMUM_PLAYERS: usize = 1;
const RADIANT: &str = "Radiant";
const DIRE: &str = "Dire";

#[derive(Debug, PartialEq, Eq, Hash)]
enum MatchResult {
    None,
    Victory,
    Defeat,
    Both
}

const TARGET_ID: &str = "8464327790344506";

struct KookPublisher;

impl KookPublisher {
    
    pub async fn publish(
        client: &KookClient,
        _guild_id: &i64, 
        guild_name: &str, 
        _guild_logo: &str, 
        guild_match: stratz::api::Match
    ) -> Result<(), Error> {
        let match_id = guild_match.id.ok_or_else(|| PollerError::Parser(ParserError::Match))?;
        let players = guild_match.players.ok_or_else(|| PollerError::Parser(ParserError::Match))?;
        if players.len() < MINIMUM_PLAYERS {
            return Ok(());
        }

        let match_result = get_match_result(players.clone()).await?;
        let lobby_type = guild_match.lobby_type.ok_or_else(|| PollerError::Parser(ParserError::Match))?;
        let game_mode = guild_match.game_mode.ok_or_else(|| PollerError::Parser(ParserError::Match))?;
    
        let duration = guild_match.duration_seconds.ok_or_else(|| PollerError::Parser(ParserError::Match))?;
        let duration = chrono::Duration::seconds(duration);
        let end = guild_match.end_date_time.ok_or_else(|| PollerError::Parser(ParserError::Match))?;
        let end = i64::try_from(end).map_err(|_| PollerError::Parser(ParserError::Match))?;
        let end = chrono::NaiveDateTime::from_timestamp(end, 0);
        let end = chrono::DateTime::<chrono::Utc>::from_utc(end, chrono::Utc);

        let mins = duration.num_seconds() / 60;
        let secs = duration.num_seconds() % 60;
        let duration_field = format!("{}:{:02}", &mins, &secs);

        let players_by_team = get_players_by_team(players.clone()).await?;
        let radiant_players = players_by_team.get(RADIANT).unwrap();
        let mut radiant_field = String::new();
        for player in radiant_players.clone() {
            let line = render_player(player)?;
            radiant_field.push_str(&line);
            radiant_field.push('\n');
        }
        let dire_players = players_by_team.get(DIRE).unwrap();
        let mut dire_field = String::new();
        for player in dire_players.clone() {
            let line = render_player(player)?;
            dire_field.push_str(&line);
            dire_field.push('\n');
        }

        let mut content = format!("https://stratz.com/matches/{}", &match_id);
        let content_author = format!("{}\n", guild_name);
        let content_title = format!(
            "{} - {} - {}\n", 
            match_match_result(&match_result), match_lobby_type(&lobby_type), match_game_mode(&game_mode)
        );
        let mut content_body = String::new();
        if radiant_field.len() > 0 {
            content_body.push_str(radiant_field.as_str());
        }
        if dire_field.len() > 0 {
            content_body.push_str(dire_field.as_str());
        }
        content_body.push_str(format!("{}\n{}\n", "Duration", duration_field).as_str());

        let content_footer = format!("{} - {}", "Powered by STRATZ", &end.to_rfc3339());

        content.push_str(content_author.as_str());
        content.push_str(content_title.as_str());
        content.push_str(content_body.as_str());
        content.push_str(content_footer.as_str());

        client.publish_message(MessageType::KMARKDOWN, TARGET_ID, content.as_str()).await?;

        Ok(())
    }

}


async fn get_match_result(players: Vec<Option<stratz::api::Player>>) -> Result<MatchResult, Error> {
    let mut is_victory = false;
    let mut is_defeat = false;
    for player in players.into_iter() {
        let player = player.ok_or_else(|| PollerError::Parser(ParserError::Player))?;
        let player_result = player.is_victory.ok_or_else(|| PollerError::Parser(ParserError::Player))?;
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

async fn get_players_by_team(players: Vec<Option<stratz::api::Player>>) -> Result<HashMap<&'static str, Vec<stratz::api::Player>>, Error> {
    let mut radiant_players: Vec<stratz::api::Player> = Vec::new();
    let mut dire_players: Vec<stratz::api::Player> = Vec::new();
    for player in players.into_iter() {
        let player = player.ok_or_else(|| PollerError::Parser(ParserError::Player))?;
        let player_team = player.is_radiant.ok_or_else(|| PollerError::Parser(ParserError::Player))?;
        match player_team {
            true => radiant_players.push(player),
            false => dire_players.push(player)
        }
    }

    Ok(HashMap::from(
        [(RADIANT, radiant_players), (DIRE, dire_players)]
    ))
}

fn match_match_result(match_result: &MatchResult) -> &'static str {
    match match_result {
        MatchResult::None => "Cancelled",
        MatchResult::Victory => "Victory",
        MatchResult::Defeat => "Defeat",
        MatchResult::Both => "Clash"
    }
}

fn match_lobby_type(lobby_type: &stratz::api::LobbyType) -> &'static str {
    match lobby_type {
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
    }
}

fn match_game_mode(game_mode: &stratz::api::GameMode) -> &'static str {
    match game_mode {
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
    }
}

fn render_player(player: stratz::api::Player) -> Result<String, PollerError> {
    let steam = player.steam_account.ok_or_else(|| PollerError::Parser(ParserError::Player))?;
    let name = steam.name.ok_or_else(|| PollerError::Parser(ParserError::Player))?;
    let hero = player.hero.ok_or_else(|| PollerError::Parser(ParserError::Player))?;
    let hero_id = hero.id.ok_or_else(|| PollerError::Parser(ParserError::Player))?;
    let kills = player.kills.ok_or_else(|| PollerError::Parser(ParserError::Player))?;
    let deaths = player.deaths.ok_or_else(|| PollerError::Parser(ParserError::Player))?;
    let assists = player.assists.ok_or_else(|| PollerError::Parser(ParserError::Player))?;
    let emoji = match_hero_emoji(hero_id);

    if let Some(imp) = player.imp {
        Ok(format!("{} {} [{}/{}/{}] `{:+}`", &emoji, &name, &kills, &deaths, &assists, &imp))
    } else {
        Ok(format!("{} {} [{}/{}/{}]", &emoji, &name, &kills, &deaths, &assists))
    }
}

fn match_hero_emoji(hero_id: i16) -> &'static str {
    match hero_id {
        1 => "antimage",
        2 => "axe",
        3 => "bane",
        4 => "bloodseeker",
        5 => "crystal_maiden",
        6 => "drow_ranger",
        7 => "earthshaker",
        8 => "juggernaut",
        9 => "mirana",
        10 => "morphling",
        11 => "shadow_fiend",
        12 => "phantom_lancer",
        13 => "puck",
        14 => "pudge",
        15 => "razor",
        16 => "sand_king",
        17 => "storm_spirit",
        18 => "sven",
        19 => "tiny",
        20 => "vengeful_spirit",
        21 => "windranger",
        22 => "zeus",
        23 => "kunkka",
        25 => "lina",
        26 => "lion",
        27 => "shadow_shaman",
        28 => "slardar",
        29 => "tidehunter",
        30 => "witch_doctor",
        31 => "lich",
        32 => "riki",
        33 => "enigma",
        34 => "tinker",
        35 => "sniper",
        36 => "necrophos",
        37 => "warlock",
        38 => "beastmaster",
        39 => "queen_of_pain",
        40 => "venomancer",
        41 => "faceless_void",
        42 => "wraith_king",
        43 => "death_prophet",
        44 => "phantom_assassin",
        45 => "pugna",
        46 => "templar_assassin",
        47 => "viper",
        48 => "luna",
        49 => "dragon_knight",
        50 => "dazzle",
        51 => "clockwerk",
        52 => "leshrac",
        53 => "natures_prophet",
        54 => "lifestealer",
        55 => "dark_seer",
        56 => "clinkz",
        57 => "omniknight",
        58 => "enchantress",
        59 => "huskar",
        60 => "night_stalker",
        61 => "broodmother",
        62 => "bounty_hunter",
        63 => "weaver",
        64 => "jakiro",
        65 => "batrider",
        66 => "chen",
        67 => "spectre",
        69 => "doom",
        68 => "ancient_apparition",
        70 => "ursa",
        71 => "spirit_breaker",
        72 => "gyrocopter",
        73 => "alchemist",
        74 => "invoker",
        75 => "silencer",
        76 => "outworld_destroyer",
        77 => "lycan",
        78 => "brewmaster",
        79 => "shadow_demon",
        80 => "lone_druid",
        81 => "chaos_knight",
        82 => "meepo",
        83 => "treant_protector",
        84 => "ogre_magi",
        85 => "undying",
        86 => "rubick",
        87 => "disruptor",
        88 => "nyx_assassin",
        89 => "naga_siren",
        90 => "keeper_of_the_light",
        91 => "io",
        92 => "visage",
        93 => "slark",
        94 => "medusa",
        95 => "troll_warlord",
        96 => "centaur_warrunner",
        97 => "magnus",
        98 => "timbersaw",
        99 => "bristleback",
        100 => "tusk",
        101 => "skywrath_mage",
        102 => "abaddon",
        103 => "elder_titan",
        104 => "legion_commander",
        105 => "techies",
        106 => "ember_spirit",
        107 => "earth_spirit",
        108 => "underlord",
        109 => "terrorblade",
        110 => "phoenix",
        111 => "oracle",
        112 => "winter_wyvern",
        113 => "arc_warden",
        114 => "monkey_king",
        119 => "dark_willow",
        120 => "pangolier",
        121 => "grimstroke",
        123 => "hoodwink",
        126 => "void_spirit",
        128 => "snapfire",
        129 => "mars",
        135 => "dawnbreaker",
        136 => "marci",
        137 => "primal_beast",
        _ => "?",
    }
}
