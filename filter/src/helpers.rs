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