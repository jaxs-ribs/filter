use serde::{Deserialize, Serialize};
use std::str::FromStr;

use kinode_process_lib::{
    await_message, call_init, println, Address, ProcessId, Request, Response,
};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

type MessageArchive = Vec<(String, String)>;

fn handle_message(our: &Address, message_archive: &mut MessageArchive) -> anyhow::Result<()> {
    let message = await_message()?;

    if !message.is_request() {
        return Err(anyhow::anyhow!("unexpected Response: {:?}", message));
    }
    Ok(())
}

fn handle_http_messages(our: &Address, message: &Message) -> Option<State> {
    match message {
        Message::Response { .. } => {
            return None;
        }
        Message::Request { ref body, .. } => {
            let server_request = http::HttpServerRequest::from_bytes(body).ok()?;
            let http_request = server_request.request()?;

            let body = get_blob()?;
            let bound_path = http_request.bound_path(Some(PROCESS_ID));
            match bound_path {
                "/status" => {
                    return fetch_status(our, message);
                }
                "/config" => {
                    return config(&body.bytes);
                }
                "/addnft" => {
                    return add_nft(&body.bytes);
                }
                "/removenft" => {
                    return remove_nft(&body.bytes);
                }
                "/listnfts" => {
                    println!("listing nfts");
                    return list_nfts();
                }
                _ => {
                    return None;
                }
            }
        }
    }
}

call_init!(init);
fn init(our: Address) {
    println!("filter: begin");
    let _ = http::serve_index_html(&our, "ui", true, false, vec!["/", "/status"]);

    loop {
        let Ok(message) = await_message() else {
            continue;
        };
        if message.source().node != our.node {
            continue;
        }

        if message.source().process == "http_server:distro:sys" {
            let state = handle_http_messages(&our, &message);
            let _ = modify_session(&our, &mut session, state);
        } else {
            match handle_internal_messages(&message, &mut session) {
                Ok(()) => {}
                Err(e) => {
                    println!("auctioneer: error: {:?}", e);
                }
            };
        }
    }
}
