use std::env;
use gpt3bot::chatbot;
use isahc::{prelude::*, Body, Request};


#[test]
fn bot_is_responding(){
    match chatbot(String::from("Olá"), env::var("OPENAI_TOKEN").unwrap()) {
        Some(response) => {
            assert!(response.len() > 0);
        },
        None => {
            assert!(false);
        }
        
    }
}