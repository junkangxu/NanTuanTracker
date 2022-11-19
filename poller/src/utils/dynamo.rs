use aws_sdk_dynamodb::{
    model::AttributeValue, Client, Error, output::GetItemOutput
};

/// Dynamo Client
pub struct DynamoClient {
    pub client: Client
}

impl DynamoClient {

    /// Get item from Dynamo with provided table_name and entry id
    /// 
    /// # Arguments
    /// 
    /// * `table_name` - The table_name of AWS DynamoDB table to get data from
    /// * `id` - The entry id of items in `table_name`
    pub async fn get_item(&self, table_name: &str, id: i64) -> Result<GetItemOutput, Error> {
        let item = self.client.get_item().table_name(table_name)
            .key("id", AttributeValue::N(id.to_string()))
            .send()
            .await?;

        Ok(item)
    }

    /// Put an item to Dynamo for given table_name, entry id and match_id
    /// 
    /// # Arguments
    /// 
    /// * `table_name` - The name of the AWS DynamoDB table to put data to
    /// * `id` - The id of entry of the AWS DynamoDB table
    /// * `match_id` - The value binded to the `id` to be put into AWS DynamoDB table
    pub async fn put_item(&self, table_name: &str, id: i64, match_id: i64) -> Result<(), Error> {
        let request = self.client.put_item().table_name(table_name)
            .item("id", AttributeValue::N(id.to_string()))
            .item("match_id", AttributeValue::N(match_id.to_string()));
        
        request.send().await?;

        Ok(())
    }

}
