use kinode_process_lib::{
    our_capabilities, spawn, Address, OnExit, ProcessId, Request,
};
use std::str::FromStr;
use serde::{Serialize, Deserialize};

use crate::llm_types::openai::{ChatParams, ChatRequest, LLMRequest, LLMResponse, Message, Provider};

pub fn spawn_groq_pkg(our: Address, groq_key: &str) -> anyhow::Result<GroqApi> {
    let groq_pkg_path = format!("{}/pkg/groq.wasm", our.package_id());
    let our_caps = our_capabilities();
    let http_client = ProcessId::from_str("http_client:distro:sys").unwrap();

    let process_id = spawn(
        None,
        &groq_pkg_path,
        OnExit::None,
        our_caps,
        vec![http_client],
        false,
    )?;

    let worker_address = Address {
        node: our.node.clone(),
        process: process_id.clone(),
    };

    Ok(GroqApi::new(groq_key.to_string(), worker_address))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroqApi {
    groq_key: String,
    groq_worker: Address,
}

impl GroqApi {
    pub fn new(groq_key: String, groq_worker: Address) -> Self {
        Self {
            groq_key,
            groq_worker,
        }
    }

    pub fn chat(&self, chat_params: ChatParams, provider: Provider) -> anyhow::Result<Message> {
        let chat_request = ChatRequest {
            params: chat_params,
            api_key: self.groq_key.clone(),
            provider,
        };
        let request = LLMRequest::Chat(chat_request);
        let msg = Request::new()
            .target(self.groq_worker.clone())
            .body(request.to_bytes())
            .send_and_await_response(10)??;

        let response = LLMResponse::parse(msg.body())?;
        if let LLMResponse::Chat(chat) = response {
            Ok(chat.to_message_response())
        } else {
            return Err(anyhow::Error::msg("Error querying groq: wrong result"));
        }
    }
}
