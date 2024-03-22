use llm_types::openai::ChatParams;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

use kinode_process_lib::{
    await_message, call_init, get_blob, http, println, Address, Message, ProcessId, Request,
    Response,
};
use std::collections::HashMap;

mod groq_api;
mod llm_types;

use groq_api::{spawn_groq_pkg, GroqApi};
use llm_types::openai::Message as GroqMessage;

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

const PROCESS_ID: &str = "filter:filter:template.os";
const GROQ_KEY: &str = include_str!("../../pkg/.groq_key");

// TODO: Zena: Is this OK? Look where it's being used.
fn default_headers() -> HashMap<String, String> {
    HashMap::from([
        ("Content-Type".to_string(), "application/json".to_string()),
        ("Access-Control-Allow-Origin".to_string(), "*".to_string()),
        ("Access-Control-Allow-Headers".to_string(), "Content-Type".to_string()),
        ("Access-Control-Allow-Methods".to_string(), "GET, POST, OPTIONS".to_string()),
    ])
}

fn handle_internal_messages(our: &Address) -> anyhow::Result<()> {
    let message = await_message()?;

    if !message.is_request() {
        return Err(anyhow::anyhow!("unexpected Response: {:?}", message));
    }
    Ok(())
}

fn handle_http_messages(our: &Address, message: &Message) -> Option<()> {
    match message {
        Message::Response { .. } => {}
        Message::Request { ref body, .. } => {
            let server_request = http::HttpServerRequest::from_bytes(body).ok()?;
            let http_request = server_request.request()?;
            if http_request.method().ok()? == http::Method::OPTIONS {
                // Handle OPTIONS request by returning the necessary CORS headers
                let _ =
                    http::send_response(http::StatusCode::OK, Some(default_headers()), Vec::new());
                return None;
            }
            let body = get_blob()?;
            let bound_path = http_request.bound_path(Some(PROCESS_ID));
            match bound_path {
                "/status" => {
                    fetch_status(our);
                }
                "/send" => {
                    send_tweet(our, &body.bytes);
                }
                _ => {}
            }
        }
    }
    None
}

fn fetch_status(our: &Address) -> Option<()> {
    let mut rng = rand::thread_rng();
    let status = if rng.gen() { "AAA" } else { "BBB" };
    let response = serde_json::to_string(&status).ok()?;
    println!("sending response: {}", response);
    let _ = http::send_response(
        http::StatusCode::OK,
        Some(default_headers()),
        response.as_bytes().to_vec(),
    );
    None
}

fn send_tweet(our: &Address, body: &[u8]) -> Option<()> {
    let tweets: Vec<String> =
        match serde_json::from_slice::<serde_json::Value>(body).ok()?["tweets"].as_array() {
            Some(tweets) => tweets
                .iter()
                .filter_map(|tweet| tweet.as_str().map(String::from))
                .collect(),
            None => vec![],
        };
    let mut rng = rand::thread_rng();
    let tweet_results: Vec<bool> = tweets.iter().map(|_| rng.gen()).collect();
    let response_body =
        serde_json::to_string(&serde_json::json!({ "tweet_results": tweet_results })).ok()?;
    println!("sending tweet results: {}", response_body);
    let _ = http::send_response(
        http::StatusCode::OK,
        Some(default_headers()),
        response_body.as_bytes().to_vec(),
    );
    None
}

// TODO: Zena: THe problem is that we're first getting a preflight request!

fn make_request(our: &Address, tweets: &[String]) -> anyhow::Result<()> {
    let api = spawn_groq_pkg(our, GROQ_KEY)?;
    let content = r###"
    You are a helpful assistant that helps to filter out tweets that don't adhere to certain rules. 
    These rules are as follows: 
    - Does not talk about a technical topic
    - Talks about a memecoin
    - Uses profanity
    You will receive a series of tweets. For each of the tweets, you either answer y or n, with y meaning the tweet passes the aforementioned rules and n meaning it does not.
    The tweets are delimited by |||.
    Don't answer with anything else, only y or n. Don't even use a space between anything, the entire output should just be a series of y's and n's.
    "###;
    let system_prompt = GroqMessage {
        role: "system".into(),
        content: content.into(),
    };
    let content = tweets.join("|||\n");
    let test_prompt = GroqMessage {
        role: "user".into(),
        content: "What is the meaning of life?".into(),
    };
    let chat_params = create_chat_params(vec![system_prompt, test_prompt]);
    let result = GroqApi::chat(&api, chat_params);
    println!("Groq result: {:?}", result);
    Ok(())
}

fn create_chat_params(messages: Vec<GroqMessage>) -> ChatParams {
    let chat_params = ChatParams {
        model: "mixtral-8x7b-32768".into(),
        messages,
        max_tokens: Some(1200),
        temperature: Some(0.0),
        ..Default::default()
    };
    chat_params
}

call_init!(init);
fn init(our: Address) {
    println!("filter: begin");
    let _ = http::serve_index_html(&our, "ui", false, true, vec!["/", "/status", "/send"]);
    // let _ = make_request(&our);

    loop {
        let Ok(message) = await_message() else {
            continue;
        };
        println!("Message from: {}", message.source().process);
        if message.source().node != our.node {
            continue;
        }

        if message.source().process == "http_server:distro:sys" {
            let state = handle_http_messages(&our, &message);
            // let _ = modify_session(&our, &mut session, state);
        } else {
            match handle_internal_messages(&our) {
                Ok(()) => {}
                Err(e) => {
                    println!("auctioneer: error: {:?}", e);
                }
            };
        }
    }
}
