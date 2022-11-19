use aws_sdk_dynamodb::{
    model::AttributeValue, Client, Error, output::GetItemOutput
};

/// Get an item from Dynamo for given table_name and entry id
/// 
/// # Arguments
/// 
/// * `client` - AWS DynamoDB client
/// * `table_name` - The name of the AWS DynamoDB table to get data from
/// * `id` - The id of entry of the AWS DynamoDB table
pub async fn get_item(client: &Client, table_name: &str, id: i64) -> Result<GetItemOutput, Error> {
    let item = client
        .get_item()
        .table_name(table_name)
        .key("id", AttributeValue::N(id.to_string()))
        .send()
        .await?;

    Ok(item)
}

/// Put an item to Dynamo for given table_name, entry id and match_id
/// 
/// # Arguments
/// 
/// * `client` - AWS DynamoDB client
/// * `table_name` - The name of the AWS DynamoDB table to put data to
/// * `id` - The id of entry of the AWS DynamoDB table
/// * `match_id` - The value binded to the `id` to be put into AWS DynamoDB table
pub async fn put_item(client: &Client, table_name: &str, id: i64, match_id: i64) -> Result<(), Error> {
    let request = client
        .put_item()
        .table_name(table_name)
        .item("id", AttributeValue::N(id.to_string()))
        .item("match_id", AttributeValue::N(match_id.to_string())
    );

    request.send().await?;

    Ok(())
}
