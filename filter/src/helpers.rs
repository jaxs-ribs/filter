use std::collections::HashMap;

// TODO: Zena: Is this OK? Look where it's being used.
pub fn default_headers() -> HashMap<String, String> {
    HashMap::from([
        ("Content-Type".to_string(), "application/json".to_string()),
        ("Access-Control-Allow-Origin".to_string(), "*".to_string()),
        (
            "Access-Control-Allow-Headers".to_string(),
            "Content-Type".to_string(),
        ),
        (
            "Access-Control-Allow-Methods".to_string(),
            "GET, POST, OPTIONS".to_string(),
        ),
    ])
}

pub fn extract_tweets(body: &[u8]) -> anyhow::Result<Vec<String>> {
    let parsed_body = serde_json::from_slice::<serde_json::Value>(body)?;
    let tweets_array = parsed_body
        .get("tweets")
        .ok_or_else(|| anyhow::anyhow!("'tweets' field is missing"))?
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("'tweets' is not an array"))?;

    let tweets = tweets_array
        .iter()
        .map(|tweet| {
            tweet.as_str().ok_or_else(|| anyhow::anyhow!("tweet is not a string")).map(String::from)
        })
        .collect::<anyhow::Result<Vec<String>>>()?;

    Ok(tweets)
}