use llm_interface::openai::ChatParams;
use llm_interface::openai::Message as OpenaiMessage;
use llm_interface::api::openai::OpenaiApi;

pub fn llm_inference(tweets: &[String], api: &OpenaiApi) -> anyhow::Result<Vec<bool>> {
    let temp_rules: Vec<String> = vec![
        "Nothing related to tech.".into(),
        "Nothing related to finance.".into(),
    ];
    let content = format!(
        r###"
    I am going to give you a series of tweets, and a series of rules. 

    The rules are: 
    {}

    The tweets are: 
    {}

    For each of the tweets, respond 0 if they break one or more rules, and 1 if they don't break any rules. 
    Do not answer with anything else but 0 or 1. No part of the answer should contain anything but the symbols 0 or 1.
    The tweets are delimited by |||.
    "###,
        temp_rules.join("\n"),
        tweets.join("|||\n"),
    );
    let system_prompt = OpenaiMessage {
        role: "system".into(),
        content: "You are a helpful assistant that will only answer with 0 or 1".into(),
    };
    let test_prompt = OpenaiMessage {
        role: "user".into(),
        content: content.into(),
    };
    let chat_params = create_chat_params(vec![system_prompt, test_prompt]);
    let result = OpenaiApi::chat(&api, chat_params)?.content;
    println!("Openai result: {:?}", result);
    let bools = parse_response_to_bool_array(&result);
    Ok(bools)
}

fn parse_response_to_bool_array(response: &str) -> Vec<bool> {
    response
        .chars()
        .filter_map(|c| match c {
            '1' => Some(true),
            '0' => Some(false),
            _ => None,
        })
        .collect()
}

fn create_chat_params(messages: Vec<OpenaiMessage>) -> ChatParams {
    let chat_params = ChatParams {
        model: "gpt-4-turbo-preview".into(),
        messages,
        max_tokens: Some(100),
        // temperature: Some(0.0),
        ..Default::default()
    };
    chat_params
}
