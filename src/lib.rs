use serde_derive::{Deserialize, Serialize};
use isahc::{prelude::*, Body, Request};


#[derive(Deserialize, Debug)]
struct OAIChoises {
	text: String,
}

#[derive(Deserialize, Debug)]
struct OAIResponse {
	choices: Vec<OAIChoises>,
}

#[derive(Serialize, Debug)]
struct OAIRequest {
	prompt: String,
	model: String,
	max_tokens: u32,
	temperature: u32,
}

pub fn chatbot(user_text: String, openai_token: String) -> Option<String> {
    let openai_url = "https://api.openai.com/v1/completions";
    let preamble = "Devo responder tudo de forma irônica e sarcástica.";

    let auth_header_val = format!("Bearer {}", openai_token);

    let openai_request = OAIRequest {
        prompt: format!("{}{}", preamble, user_text),
        model: String::from("text-davinci-003"),
        max_tokens: 100,
        temperature: 0,
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
            let response_json: OAIResponse = serde_json::from_str(&response_body.text().unwrap()).unwrap();
            Some(response_json.choices[0].text.trim().to_string())
        },
        None => None,
    }
    
}


