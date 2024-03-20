use llm_types::openai::ChatParams;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use kinode_process_lib::{
    await_message, call_init, println, Address, ProcessId, Request, Response, http, Message, get_blob 
};
use std::collections::HashMap;

mod groq_api;
mod llm_types;

use llm_types::openai::Message as GroqMessage;
use groq_api::{GroqApi, spawn_groq_pkg};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

const PROCESS_ID: &str = "filter:filter:template.os";
const GROQ_KEY: &str = include_str!("../../pkg/.groq_key");

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

            let body = get_blob()?;
            let bound_path = http_request.bound_path(Some(PROCESS_ID));
            match bound_path {
                "/status" => {
                    fetch_status(our, message);
                }
                _ => {}
            }
        }
    }
    None
}

fn fetch_status(our: &Address, message: &Message) -> Option<()> {
    let mut rng = rand::thread_rng();
    let status = if rng.gen() { "AAA" } else { "BBB" };
    let response = serde_json::to_string(&status).ok()?;
    // TODO: Zena: Is this OK? 
    let headers = HashMap::from([
        ("Content-Type".to_string(), "application/json".to_string()),
        ("Access-Control-Allow-Origin".to_string(), "*".to_string()),
        ("Access-Control-Allow-Headers".to_string(), "Content-Type".to_string()),
        ("Access-Control-Allow-Methods".to_string(), "GET, POST, OPTIONS".to_string()),
    ]);
    println!("sending response: {}", response);
    let _ = http::send_response(http::StatusCode::OK, Some(headers), response.as_bytes().to_vec());
    None
}

fn make_request(our: &Address) -> anyhow::Result<()> {
    let api = spawn_groq_pkg(our, GROQ_KEY)?;
    let system_prompt = GroqMessage {
        role: "system".into(),
        content: "You are a helpful assistant.".into(),
    };
    let test_prompt = GroqMessage {
        role: "user".into(),
        content: "What is the meaning of life?".into(),
    };
    let chat_params = create_chat_params(vec![system_prompt, test_prompt]);
    let result = GroqApi::chat(&api, chat_params);
    println!("result: {:?}", result);
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
    let _ = http::serve_index_html(&our, "ui", false, true, vec!["/", "/status"]);
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
