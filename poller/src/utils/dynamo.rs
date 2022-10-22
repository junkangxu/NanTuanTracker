use aws_sdk_dynamodb::{
    model::AttributeValue, Client, Error, output::GetItemOutput
};

pub async fn get_item(client: &Client, table_name: &str, id: i64) -> Result<GetItemOutput, Error> {
    let item = client
        .get_item()
        .table_name(table_name)
        .key("id", AttributeValue::N(id.to_string()))
        .send()
        .await?;

    Ok(item)
}

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
