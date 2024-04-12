use serde::{Deserialize, Serialize};
use kinode_process_lib::{get_state, set_state};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub rules: Vec<String>,
    pub is_on: bool,
    pub openai_key: Option<String>,
    // pub filtered_tweets: HashMap<String, bool>,
}

impl State {
    pub fn new() -> Self {
        State {
            rules: vec![
                "This is a test 1".to_string(),
                "This is a test 2".to_string(),
                "This is a test 3".to_string(),
            ],
            is_on: true,
            openai_key: None,
            // filtered_tweets: HashMap::new(),
        }
    }
}

impl State {
    pub fn fetch() -> State {
        if let Some(state_bytes) = get_state() {
            bincode::deserialize(&state_bytes).expect("Failed to deserialize state")
        } else {
            State::new()
        }
    }

    pub fn save(&self) {
        let serialized_state = bincode::serialize(self).expect("Failed to serialize state");
        set_state(&serialized_state);
    }
}

// TODO: Zen: We are making this another struct in case state gets expanded. If it isn't, just merge
#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub rules: Vec<String>,
    pub is_on: bool,
    pub api_key: String,
}

