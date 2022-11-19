mod config;
mod provider;
mod utils;
mod publisher;

use provider::stratz::api::StratzClient;
use publisher::publisher::Publisher;
use utils::dynamo::DynamoClient;
use lambda_runtime::LambdaEvent;
use lambda_runtime::{Error, service_fn};
use aws_sdk_dynamodb;
use serde_json::{json, Value};

const TAKE: i64 = 5;
const GUILD_ID: i64 = 117311;
const GUILD_TABLE_NAME: &str = "Guilds";

/// The entry point of AWS Lambda Function
#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;

    Ok(())
}

async fn handler(_event: LambdaEvent<Value>) -> Result<Value, Error> {
    match process().await {
        Ok(_) => Ok(json!({"message": format!("Success")})),
        Err(e) => Ok(json!({"message": format!("Failure: {}", e)}))
    }
}

async fn process() -> Result<(), Error> {
    // initialize Stratz client as Dota2 data provider
    let stratz_client = StratzClient {
        client: reqwest::Client::new()
    };

    // Fetch matches using Stratz client
    let response = stratz_client.fetch_matches(GUILD_ID, TAKE).await?;

    // Extract useful information from the matches fetched
    let data = response.data.unwrap();
    let guild = data.guild.unwrap();
    let guild_id = guild.id.unwrap();
    let guild_name = guild.name.unwrap();
    let guild_logo = guild.logo.unwrap();
    let guild_matches = guild.matches.unwrap();

    // Initialize AWS DynamoDB client for matches id validations
    let dynamo_client = DynamoClient {
        client: aws_sdk_dynamodb::Client::new(&aws_config::load_from_env().await)
    };

    // Get the latest matches we already processed
    let current_match_id = get_current_match_id(&dynamo_client).await?;
    let mut latest_match_id = current_match_id;
    
    // Iterate through the fetched matches and check if they are newer compared to the latest matches in
    // our database.
    //
    // If so, we will prepare the data and publish them.
    // Otherwise, we will continue to next match.
    for guild_match in guild_matches.into_iter().rev() {
        let guild_match = guild_match.unwrap();
        let match_id = guild_match.id.unwrap();
        if match_id <= current_match_id {
            continue;
        } else {
            // if match_id > current_match_id, then we publish the data and after the publish succeeds, we update 
            // the latest_match_id
            Publisher::publish(guild_id, &guild_name, &guild_logo, guild_match).await?;
            if match_id > latest_match_id {
                latest_match_id = match_id;
            }
        }
    }

    // Update the latest match id in database to the newest match id we just fetched
    if latest_match_id > current_match_id {
        save_new_current_match_id(&dynamo_client, latest_match_id).await?;
    }

    Ok(())
}

/// The wrapper of AWS DynamoDB GetItem operation to get the latest match id we already processed.
/// 
/// # Arguments
/// 
/// * `client` - AWS DynamoDB client
async fn get_current_match_id(client: &DynamoClient) -> Result<i64, Error> {
    let item = client.get_item(GUILD_TABLE_NAME, GUILD_ID).await?;
    Ok(item.item().unwrap().get("match_id").unwrap().as_n().unwrap().parse::<i64>().unwrap())
}

/// The wrapper of AWS DynamoDB PutItem operation to put the latest match id we just process
/// 
/// # Arguments
/// 
/// * `client` - AWS DynamoDB client
/// * `match_id` - The latest match id to be put in DynamoDB table
async fn save_new_current_match_id(client: &DynamoClient, match_id: i64) -> Result<(), Error> {
    client.put_item(GUILD_TABLE_NAME, GUILD_ID, match_id).await?;
    Ok(())
}
