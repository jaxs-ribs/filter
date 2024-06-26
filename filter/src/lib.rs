use kinode_process_lib::{await_message, call_init, get_blob, http, println, Address, Message};
use llm_interface::api::openai::{spawn_openai_pkg, OpenaiApi};
use serde_json::Value;

mod llm_inference;
mod llm_inference_with_image;

mod helpers;
use helpers::default_headers;

mod structs;
use structs::Settings;
use structs::State;

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

call_init!(init);

fn handle_http_messages(our: &Address, message: &Message, api: &mut OpenaiApi, state: &mut State) {
    if let Message::Request { ref body, .. } = message {
        handle_request(our, body, api, state);
    }
}

fn handle_request(
    our: &Address,
    body: &[u8],
    api: &mut OpenaiApi,
    state: &mut State,
) -> Option<()> {
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
            let bound_path = http_request.bound_path(Some(&our.process()));
            match bound_path {
                "/filter" => {
                    filter_tweets(&body.bytes, api, state);
                }
                "/fetch_settings" => {
                    fetch_settings(state);
                }
                "/submit_settings" => {
                    submit_settings(&body.bytes, api, state);
                }
                _ => {}
            }
        }
        _ => {}
    }
    None
}

fn filter_tweets(body: &[u8], api: &OpenaiApi, state: &mut State) -> Option<()> {
    let tweets_data: Value = serde_json::from_slice(body).ok()?;
    let tweets_array = tweets_data["tweets"].as_array()?;
    let debug = tweets_data["debug"].as_bool().unwrap_or(true);
    let with_image = tweets_data["withImage"].as_bool().unwrap_or(false);

    let tweet_ids: Vec<String> = tweets_array
        .iter()
        .filter_map(|tweet| tweet["tweetId"].as_str())
        .map(|id| id.to_string())
        .collect();
    let tweet_contents: Vec<String> = tweets_array
        .iter()
        .filter_map(|tweet| tweet["content"].as_str().map(|content| content.to_string()))
        .collect();
    let photo_urls: Vec<Option<String>> = tweets_array
        .iter()
        .map(|tweet| tweet["photoUrl"].as_str().map(|url| url.to_string()))
        .collect();

    if tweets_array.len() != photo_urls.len() {
        println!(
            "Mismatch in the number of tweets ({}) and photo URLs ({})",
            tweets_array.len(),
            photo_urls.len()
        );
        return None;
    }

    let should_pass_vec = if debug {
        println!("Tweet contents: {:?}", tweet_contents.len());
        tweet_contents.iter().map(|_| rand::random()).collect()
    } else if state.is_on && state.rules.len() > 0 && tweet_contents.len() > 0 {
        if with_image {
            llm_inference_with_image::llm_inference_with_image(
                &tweet_contents,
                &photo_urls,
                &state.rules,
                api,
            )
            .ok()?
        } else {
            llm_inference::llm_inference(&tweet_contents, &state.rules, api).ok()?
        }
    } else {
        vec![true; tweet_contents.len()]
    };

    if should_pass_vec.len() != tweet_ids.len() {
        println!(
            "Tweet results and tweet ids length mismatch, with {} and {}",
            should_pass_vec.len(),
            tweet_ids.len()
        );
        return None;
    }

    let mut tweet_results = Vec::new();
    for (tweet_id, should_pass) in tweet_ids.into_iter().zip(should_pass_vec) {
        tweet_results.push(serde_json::json!({
            "tweetId": tweet_id,
            "filterBool": should_pass,
        }));
    }

    let response_body = serde_json::to_string(&tweet_results).ok()?;
    if tweet_results.len() > 0 {
        println!("Response body is: {}", response_body);
    }

    let _ = http::send_response(
        http::StatusCode::OK,
        Some(default_headers()),
        response_body.as_bytes().to_vec(),
    );

    None
}

fn submit_settings(body: &[u8], api: &mut OpenaiApi, state: &mut State) -> Option<()> {
    println!("Submit settings!");
    let settings = serde_json::from_slice::<Settings>(body).ok()?;
    state.rules = settings.rules;
    state.is_on = settings.is_on;
    state.openai_key = Some(settings.api_key.clone());
    state.save();

    api.openai_key = settings.api_key;
    None
}

fn fetch_settings(state: &mut State) -> Option<()> {
    println!("Fetch settings!");
    let response_body = serde_json::to_string(&serde_json::json!({
        "rules": state.rules,
        "is_on": state.is_on,
    }))
    .ok()?;

    let _ = http::send_response(
        http::StatusCode::OK,
        Some(default_headers()),
        response_body.as_bytes().to_vec(),
    );
    Some(())
}

fn setup(our: &Address, state: &State) -> OpenaiApi {
    if let Err(e) = http::serve_index_html(
        &our,
        "ui",
        false,
        true,
        vec![
            "/",
            "/send",
            "/fetch_settings",
            "/submit_settings",
            "/filter",
        ],
    ) {
        panic!("Error binding https paths: {:?}", e);
    }
    // TODO: Zen: Maybe we shouldn't have a default value in the first place?
    let Ok(api) = spawn_openai_pkg(our.clone(), &state.openai_key.clone().unwrap_or_default())
    else {
        panic!("Failed to spawn openai pkg");
    };
    api
}

fn init(our: Address) {
    let mut state = State::fetch();
    let mut api = setup(&our, &state);

    while let Ok(message) = await_message() {
        if message.source().node != our.node {
            continue;
        }

        if message.source().process == "http_server:distro:sys" {
            handle_http_messages(&our, &message, &mut api, &mut state);
            state.save();
        }
    }
}
