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

#[cfg(test)]
mod tests {

    use std::env;
    use super::{stratz_jwt, discord_webhook_url};

    #[test]
    #[should_panic(expected = "Missing STRATZ_JWT environmental variable")]
    fn test_stratz_jwt_without_env() {
        stratz_jwt();
    }
    
    #[test]
    fn test_stratz_jwt() {
        env::set_var("STRATZ_JWT", "TestingJWT");
        assert_eq!(stratz_jwt(), "TestingJWT");
        env::remove_var("STRATZ_JWT");
    }

    #[test]
    #[should_panic(expected = "Missing DISCORD_WEBHOOK_URL environmental variable")]
    fn test_discord_webhook_url_without_env() {
        discord_webhook_url();
    }

    #[test]
    fn test_discord_webhook_url() {
        env::set_var("DISCORD_WEBHOOK_URL", "TestingUrl");
        assert_eq!(discord_webhook_url(), "TestingUrl");
        env::remove_var("DISCORD_WEBHOOK_URL");
    }

}