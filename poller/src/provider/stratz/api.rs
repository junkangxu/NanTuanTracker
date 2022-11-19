use lambda_runtime::Error;
use crate::config;
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

/// Stratz API Client as Dota data provider
pub struct StratzClient {
    pub client: reqwest::Client
}

impl StratzClient {

    /// Fetch Dota2 matches based on guild_id and take
    /// 
    /// # Arguments
    /// 
    /// * `guild_id` - The guild_id we will use to get matches from
    /// * `take` - The number of matches to be fetched
    pub async fn fetch_matches(&self, guild_id: i64, take: i64) -> Result<Response, Error> {
        let vars = Variable { guild_id, take };
        let body = MatchesQuery::build_query(vars);
        let response = self.client.post(StratzClient::api_url()).json(&body).send().await?;
        let data = response.json::<Response>().await?;

        Ok(data)
    }

    /// Get the API URL of Stratz API
    fn api_url() -> String {
        format!("https://api.stratz.com/graphql?jwt={}", &config::stratz_jwt())
    }

}
