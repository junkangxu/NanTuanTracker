use crate::config;
use crate::PublisherError;
use graphql_client::GraphQLQuery;

type Short = i16;
type Long = i64;
type Byte = u8;

#[derive(GraphQLQuery)]
#[graphql(schema_path="src/provider/stratz/assets/stratz_schema.gql", query_path="src/provider/stratz/assets/latest_guild_matches.gql", response_derives="Clone,Debug")]
struct MatchesQuery;
pub type Response = graphql_client::Response<matches_query::ResponseData>;
pub use matches_query::ResponseData;
pub use matches_query::Variables as Variable;
pub use matches_query::LobbyTypeEnum as LobbyType;
pub use matches_query::GameModeEnumType as GameMode;
pub use matches_query::MatchesQueryGuild as Guild;
pub use matches_query::MatchesQueryGuildMatches as Match;
pub use matches_query::MatchesQueryGuildMatchesPlayers as Player;
pub use matches_query::MatchesQueryGuildMatchesPlayersHero as Hero;
pub use matches_query::MatchesQueryGuildMatchesPlayersSteamAccount as Steam;

/// Get the Stratz API url
fn api_url() -> String {
    format!("https://api.stratz.com/graphql?jwt={}", &config::stratz_jwt())
}

/// Fetches Dota2 matchces with given guild_id and take
/// 
/// # Arguments
/// 
/// * `client` - http client.
/// * `guild` - The id of Dota2 guild.
/// * `take` - The number of matches to be fetched
pub async fn fetch_matches(client: reqwest::Client, guild_id: i64, take: i64) -> Result<Response, PublisherError> {
    let vars = Variable {guild_id, take};
    let body = MatchesQuery::build_query(vars);
    let response = client.post(api_url()).json(&body).send().await.map_err(|err| {
        error!("Error while performing request: {:#?}", &err);
        PublisherError::Discord
    })?;

    let data = response.json::<Response>().await.map_err(|err| {
        error!("Error while parsing response: {:#?}", &err);
        PublisherError::Discord
    })?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    
    use std::env;
    use super::api_url;

    #[test]
    #[should_panic(expected = "Missing STRATZ_JWT environmental variable")]
    fn test_api_url_without_jwt() {
        api_url();
    }

    #[test]
    fn test_api_url() {
        env::set_var("STRATZ_JWT", "JWT");
        assert!(api_url().starts_with("https://api.stratz.com/graphql?jwt="));
        env::remove_var("STRATZ_JWT");
    }

}
