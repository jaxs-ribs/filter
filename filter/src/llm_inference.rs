use kinode_process_lib::println;
use llm_interface::api::openai::OpenaiApi;
use llm_interface::openai::ChatParams;
use llm_interface::openai::Message as OpenaiMessage;

pub fn llm_inference(
    tweets: &[String],
    rules: &Vec<String>,
    api: &OpenaiApi,
) -> anyhow::Result<Vec<bool>> {
    let joined_tweets = tweets
        .iter()
        .enumerate()
        .map(|(i, tweet)| format!("{}. {}", i + 1, tweet))
        .collect::<Vec<String>>()
        .join("\n<ENDOFTWEET>\n");

    let rules = rules
        .iter()
        .enumerate()
        .map(|(i, rule)| format!("{}. {}", i + 1, rule))
        .collect::<Vec<String>>()
        .join("\n");

    let content = format!(
        r###"
I am going to give you a series of tweets, and a series of rules. 

The rules are: 
{}
----------

The tweets are: 
{}

----------
For each of the tweets, respond 0 if they break one or more rules, and 1 if they don't break any rules. 
Do not answer with anything else but 0 or 1. No part of the answer should contain anything but the symbols 0 or 1. There are a total of {} tweets, meaning your answer should be {} digits long. The tweets are separated by <ENDOFTWEET> and are numbered, this is how you can separate them out. 
"###,
        rules,
        joined_tweets,
        tweets.len(),
        tweets.len() 
    );
    // println!("Content: {}", content);
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
    // println!("Openai result: {:?}", result);
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
        max_tokens: Some(40),
        ..Default::default()
    };
    chat_params
}