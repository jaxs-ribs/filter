use kinode_process_lib::{await_message, call_init, get_blob, http, println, Address, Message};
use llm_interface::api::openai::{spawn_openai_pkg, OpenaiApi};

mod llm_inference;

mod helpers;
use helpers::default_headers;
use helpers::extract_tweets;

mod structs;
use structs::State;

// TODO: Zen: Remove this
const PROCESS_ID: &str = "filter:filter:template.os";
const OPENAI_API: &str = include_str!("../../pkg/.openai_key");

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

call_init!(init);

fn handle_http_messages(message: &Message, api: &OpenaiApi, state: &mut State)  {
    if let Message::Request { ref body, .. } = message {
        handle_request(body, api, state);
    }
}

fn handle_request(body: &[u8], api: &OpenaiApi, state: &mut State) -> Option<()> {
    let server_request = http::HttpServerRequest::from_bytes(body).ok()?;
    let http_request = server_request.request()?;
    match http_request.method().ok() {
        Some(http::Method::OPTIONS) => {
            // Handle OPTIONS request by returning the necessary CORS headers
            let _ = http::send_response(http::StatusCode::OK, Some(default_headers()), Vec::new());
            return None;
        }
        Some(http::Method::POST) => {
            let body = get_blob()?;
            let bound_path = http_request.bound_path(Some(PROCESS_ID));
            match bound_path {
                "/send" => {
                    filter_tweets(&body.bytes, api);
                }
                "/fetch_settings" => {
                    fetch_settings(state);
                }
                _ => {}
            }
        }
        _ => {}
    }
    None
}

fn fetch_settings(state: &mut State) -> Option<()> {
    println!("Found fetch settings");
    let response_body = serde_json::to_string(&serde_json::json!({
        "rules": state.rules,
        "is_on": state.is_on
    })).ok()?;

    let _ = http::send_response(
        http::StatusCode::OK,
        Some(default_headers()),
        response_body.as_bytes().to_vec(),
    );
    Some(())
}

fn filter_tweets(body: &[u8], api: &OpenaiApi) -> Option<()> {
    let tweets = extract_tweets(body).ok()?;
    let tweet_results = llm_inference::llm_inference(&tweets, api).ok()?;
    // TODO: Zen: Sometimes the llm response doesn't return enough responses for all the tweets. Maybe we need to separate them and number them? 
    // assert_eq!(tweets.len(), tweet_results.len(), "Tweets and results length mismatch");

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


fn setup(our: &Address) -> OpenaiApi {
    println!("filter: begin");
    if let Err(e) = http::serve_index_html(&our, "ui", false, true, vec!["/", "/send", "/fetch_settings"]) {
        panic!("Error serving index html: {:?}", e);
    }
    let Ok(api) = spawn_openai_pkg(our.clone(), OPENAI_API) else {
        panic!("Failed to spawn openai pkg");
    };
    api
}

fn init(our: Address) {
    let api = setup(&our);
    let mut state = State::fetch();

    while let Ok(message) = await_message() {
        if message.source().node != our.node {
            continue;
        }

        if message.source().process == "http_server:distro:sys" {
            handle_http_messages(&message, &api, &mut state);
            state.save();
        } 
    }
}
