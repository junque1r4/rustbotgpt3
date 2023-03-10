use serde_derive::{Deserialize, Serialize};
use isahc::{prelude::*, Body, Request};


#[derive(Serialize, Deserialize, Debug)]
struct OAIChoices {
    role: String,
	content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChoicesTemplate {
    index: i32,
    message: OAIChoices,
    finish_reason: String
}

#[derive(Deserialize, Debug)]
struct OAIResponse {
	choices: Vec<ChoicesTemplate>,
}

#[derive(Serialize, Debug)]
struct OAIRequest {
	messages: Vec<MessageModel>,
	model: String,
	max_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageModel {
    role: String,
    content: String,
}

pub fn chatbot(user_text: String, openai_token: String) -> Option<String> {
    let openai_url = "https://api.openai.com/v1/chat/completions";

    let auth_header_val = format!("Bearer {}", openai_token);

    let openai_request = OAIRequest {
        messages: vec![ MessageModel {
            role: "user".to_string(),
            content: user_text,
        }],
        model: String::from("gpt-3.5-turbo-0301"),
        max_tokens: 350,
    };

    let requisition_body = Body::from(serde_json::to_vec(&openai_request).unwrap());
    let response = Request::post(openai_url)
        .header("Content-Type", "application/json")
        .header("Authorization", auth_header_val)
        .body(requisition_body).unwrap()
        .send().ok();
    
    // response as json
    match response {
        Some(mut response_body) => {
            let response_json= serde_json::from_str(&response_body.text().unwrap());
            let unraped: OAIResponse  = match response_json {
                Ok(a) => {
                    a
                },
                Err(_) => {
                    return None;
                }
            };
            Some(unraped.choices[0].message.content.trim().to_string())
        },
        None => None,
    }
}
