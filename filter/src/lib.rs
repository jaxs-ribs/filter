use rand::Rng;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use kinode_process_lib::{
    await_message, call_init, println, Address, ProcessId, Request, Response, http, Message, get_blob 
};
use std::collections::HashMap;

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

const PROCESS_ID: &str = "filter:filter:template.os";

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
    let headers = HashMap::from([
        ("Content-Type".to_string(), "application/json".to_string()),
        ("Access-Control-Allow-Origin".to_string(), "*".to_string()),
        // Allow Content-Type and other necessary headers
        ("Access-Control-Allow-Headers".to_string(), "Content-Type".to_string()),
        // Optionally, specify the methods allowed
        ("Access-Control-Allow-Methods".to_string(), "GET, POST, OPTIONS".to_string()),
    ]);
    println!("sending response: {}", response);
    let _ = http::send_response(http::StatusCode::OK, Some(headers), response.as_bytes().to_vec());
    None
}

call_init!(init);
fn init(our: Address) {
    println!("filter: begin");
    // TODO: Zena: Here's the problem, you need to serve to the extension. 
    let _ = http::serve_index_html(&our, "ui", false, false, vec!["/", "/status"]);
    // TODO: Zena: Local only to true

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
