use std::env;

/**
 * Get the [Stratz API Key] from `STRATZ_JWT` envrionmental variable
 */
pub fn stratz_jwt() -> String {
    let value = env::var("STRATZ_JWT").expect("Missing STRATZ_JWT environmental variable");
    return value;
}

/**
 * Get the Discord webhook URL from `DISCORD_WEBHOOK_URL` environmental variable
 */
pub fn discord_webhook_url() -> String {
    let value = env::var("DISCORD_WEBHOOK_URL").expect("Missing DISCORD_WEBHOOK_URL environmental variable");
    return value;
}