use kinode_process_lib::println;
use llm_interface::api::openai::OpenaiApi;
use llm_interface::openai::ChatImageContent;
use llm_interface::openai::ChatImageMessage;
use llm_interface::openai::ChatImageParams;

pub fn llm_inference_with_image(
    tweet_contents: &[String],
    photo_urls: &[Option<String>],
    rules: &Vec<String>,
    api: &OpenaiApi,
) -> anyhow::Result<Vec<bool>> {
    let rules_string = rules
        .iter()
        .enumerate()
        .map(|(i, rule)| format!("{}. {}", i + 1, rule))
        .collect::<Vec<String>>()
        .join("\n");
    let mut final_message = vec![ChatImageMessage {
        role: "system".into(),
        content: ChatImageContent::from_text(&format!("Tweet: {}", &system_prompt_text(
            // tweet_contents.len(),
            &rules_string,
        ))),
    }];

    for (tweet, photo_url) in tweet_contents.iter().zip(photo_urls.iter()) {
        let chat_image_content = if let Some(photo_url) = photo_url {
            ChatImageContent::from_pair(tweet, photo_url)
        } else {
            ChatImageContent::from_text(tweet)
        };

        final_message.push(ChatImageMessage {
            role: "user".into(),
            content: chat_image_content,
        });
    }

    let chat_image_params: ChatImageParams = create_chat_params(final_message);
    let result = match OpenaiApi::chat_with_image(&api, chat_image_params) {
        Ok(response) => response.content,
        Err(e) => {
            println!("Error calling OpenAI API: {:?}", e);
            return Err(e.into());
        }
    };
    println!("Openai result: {:?}", result);
    let bools = parse_response_to_bool_array(&result);
    Ok(bools)
}

fn system_prompt_text(rules_string: &str) -> String {
    format!(
        "For each tweet provided, which may contain text and/or an image, evaluate it against these rules: {}. \
         Each tweet must result in exactly one digit in your output: '1' for compliance and '0' for violation. \
         Assemble a binary sequence reflecting the evaluation of each tweet sequentially. \
         Ensure the binary sequence directly corresponds to the number of tweets evaluated. \
         Do not describe the images, provide additional information, or output anything other than the binary sequence. \
         Focus solely on the evaluation of compliance based on the provided rules.",
        rules_string
    )
}

// fn system_prompt_text(tweet_count: usize, rules_string: &str) -> String {
//     //     format!(
//     //         r###"
//     // You are a helpful assistant that will only answer with 0 or 1.
//     // Each message of the user will represent a tweet, and possibly an image.
//     // For each of the tweets, respond 0 if the content within the text or image breaks one or more rules, and 1 if no rules are broken.
//     // Do not answer with anything else but 0 or 1. No part of the answer should contain anything but the symbols 0 or 1. There are a total of {} tweets, meaning your answer should be {} digits long. It can't be any longer or shorter than that.
//     // The rules are:
//     // {}
//     // "###,
//     //         tweet_count, tweet_count, rules_string
//     //     )
//     format!(
//         "Respond with a binary sequence exactly {} digits long for {} tweets, where each digit indicates compliance. \
//          Each '1' means the tweet, whether text or image, complies with the rules, and each '0' means it violates them. \
//          When an image is attached, check the entire text in it, and treat it as part of the tweet. \
//          Ensure the sequence matches the tweet count exactly. \
//          The rules are: {} \
//          Remember to only respond with 0 or 1, and the total length of the response should be {} digits.
//          Do not describe the images, your only output should be a sequence of 0s and 1s.
//          ",
//         tweet_count, tweet_count, tweet_count, rules_string
//     )
// }

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
