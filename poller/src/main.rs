#[macro_use] extern crate log;

mod config;
mod provider;
mod errors;
mod utils;
mod publisher;

use publisher::publisher::Publisher;
use utils::dynamo;
use lambda_runtime::LambdaEvent;
use lambda_runtime::{Error, service_fn};
use aws_sdk_dynamodb;
use serde_json::{json, Value};

use crate::errors::PollerError;
use crate::errors::PublisherError;
use crate::errors::ParserError;
use crate::errors::ProviderError;
use crate::provider::stratz;

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
    let http_client = reqwest::Client::new();
    let response = stratz::api::fetch_matches(http_client, GUILD_ID, TAKE).await.map_err(|_e| PollerError::Provider(ProviderError::Stratz))?;
    if let Some(errors) = response.errors {
        error!("Error in SRATZ response: {:#?}", errors);
        return Err(Box::new(PollerError::Provider(ProviderError::Stratz)));
    }

    let data: stratz::api::ResponseData = response.data.ok_or_else(|| PollerError::Parser(ParserError::Guild))?;
    let guild: stratz::api::Guild = data.guild.ok_or_else(|| PollerError::Parser(ParserError::Guild))?;
    let guild_id: i64 = guild.id.ok_or_else(|| PollerError::Parser(ParserError::Guild))?;
    let guild_name: String = guild.name.ok_or_else(|| PollerError::Parser(ParserError::Guild))?;
    let guild_logo: String = guild.logo.ok_or_else(|| PollerError::Parser(ParserError::Guild))?;
    let guild_matches: Vec<Option<stratz::api::Match>> = guild.matches.ok_or_else(|| PollerError::Parser(ParserError::Guild))?;

    let shared_config = aws_config::load_from_env().await;
    let dynamo_client = aws_sdk_dynamodb::Client::new(&shared_config);
    let current_match_id = get_current_match_id(&dynamo_client).await?;
    let mut latest_match_id = current_match_id;
    
    for guild_match in guild_matches.into_iter().rev() {
        let guild_match = guild_match.ok_or_else(|| PollerError::Parser(ParserError::Guild))?;
        let match_id = guild_match.id.ok_or_else(|| PollerError::Parser(ParserError::Match))?;
        if match_id <= current_match_id {
            continue;
        } else {
            Publisher::publish(guild_id, &guild_name, &guild_logo, guild_match).await?;
            if match_id > latest_match_id {
                latest_match_id = match_id;
            }
        }
    }

    if latest_match_id > current_match_id {
        save_new_current_match_id(&dynamo_client, latest_match_id).await.map_err(|_| PollerError::Provider(ProviderError::Dynamo))?;
    }

    Ok(())
}

/// The wrapper of AWS DynamoDB GetItem operation to get the latest match id we already processed.
async fn get_current_match_id(client: &aws_sdk_dynamodb::Client) -> Result<i64, Error> {
    let item = dynamo::get_item(client, GUILD_TABLE_NAME, GUILD_ID).await?;
    Ok(item.item().unwrap().get("match_id").unwrap().as_n().unwrap().parse::<i64>().unwrap())
}

/// The wrapper of AWS DynamoDB PutItem operation to put the latest match id we just process
async fn save_new_current_match_id(client: &aws_sdk_dynamodb::Client, match_id: i64) -> Result<(), Error> {
    dynamo::put_item(client, GUILD_TABLE_NAME, GUILD_ID, match_id).await?;
    Ok(())
}
