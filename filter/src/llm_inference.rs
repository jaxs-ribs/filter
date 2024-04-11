use kinode_process_lib::println;
use llm_interface::api::openai::OpenaiApi;
use llm_interface::openai::ChatImageContent;
use llm_interface::openai::ChatImageMessage;
use llm_interface::openai::ChatImageParams;
use llm_interface::openai::ChatParams;
use llm_interface::openai::Message as OpenaiMessage;

pub fn llm_inference(
    tweet_contents: &[String],
    photo_urls: &[Option<String>],
    rules: &Vec<String>,
    api: &OpenaiApi,
) -> anyhow::Result<Vec<bool>> {
    println!("Llm inference was called.");
    let rules = rules
        .iter()
        .enumerate()
        .map(|(i, rule)| format!("{}. {}", i + 1, rule))
        .collect::<Vec<String>>()
        .join("\n");
    let mut final_message = vec![OpenaiMessage {
        role: "system".into(),
        content: system_prompt_text(tweet_contents.len(), rules),
    }];

    for (tweet, photo_url) in tweet_contents.iter().zip(photo_urls.iter()) {
        let chat_image_content = if let Some(photo_url) = photo_url {
            ChatImageContent::from_pair(tweet, photo_url);
        } else {
            ChatImageContent::from_text(tweet);
        };

        final_message.push(ChatImageMessage {
            role: "user".into(),
            content: chat_image_content,
        });
    }

    let chat_image_params: ChatImageParams = create_chat_params(final_message);
    let result = OpenaiApi::chat_with_image(&api, chat_image_params)?.content;
    println!("Openai result: {:?}", result);
    let bools = parse_response_to_bool_array(&result);
    Ok(bools)
}

fn system_prompt_text(tweet_count: usize, rules: &Vec<String>) -> String {
    format!(
        r###"
You are a helpful assistant that will only answer with 0 or 1.
Each message of the user will represent a tweet, and possibly an image. 
For each of the tweets, respond 0 if the content within the text or image breaks one or more rules, and 1 if no rules are broken. 
Do not answer with anything else but 0 or 1. No part of the answer should contain anything but the symbols 0 or 1. There are a total of {} tweets, meaning your answer should be {} digits long. 
The rules are: 
{}
"###,
        tweet_count, tweet_count, rules
    )
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

fn create_chat_params(messages: Vec<ChatImageMessage>) -> ChatImageParams {
    let chat_params = ChatImageParams {
        model: "gpt-4-turbo".into(),
        messages,
        max_tokens: Some(40),
        ..Default::default()
    };
    chat_params
}
