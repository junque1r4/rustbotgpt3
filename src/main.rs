use std::env;

use gpt3bot::chatbot;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Dialogue,
}

/*
TODO: Fazer o bot responder somente quando é mencionado.
TODO: Fazer o bot responder somente nos canais com permissão
*/
async fn bot_dialogue(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let openai_token = env::var("OPENAI_TOKEN").unwrap();
    match msg.text() {
        Some(valid_input) => {
            log::info!("Query: {}", valid_input);
            log::info!("ChatID: {}", msg.chat.id);

            if valid_input.to_lowercase() == "/start" {
                bot.send_message(msg.chat.id, "Olá, eu sou o bot do Kurono!. Pergunte algo para eu responder.").await?;
            } else {
                match chatbot(valid_input.to_string(), openai_token) {
                    Some(chatbot_response) => {
                        let formated_text = handle_string_to_mkv2(chatbot_response).await;
                        bot.send_message(msg.chat.id, formated_text).parse_mode(teloxide::types::ParseMode::MarkdownV2).await?;
                    },
                    None => {
                        bot.send_message(msg.chat.id, "Não entendi o que você disse.").await?;
                    }
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Não entendi o que você disse.").await?;
        }
    }
    dialogue.exit().await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Dialogue].endpoint(bot_dialogue)),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

pub async fn handle_string_to_mkv2(text: String) -> String {
    text.replace("_", "\\_")
        .replace("*", "\\*")
        .replace("[", "\\[")
        .replace("]", "\\]")
        .replace("(", "\\(")
        .replace(")", "\\)")
        .replace("~", "\\~")
        .replace(">", "\\>")
        .replace("#", "\\#")
        .replace("+", "\\+")
        .replace("-", "\\-")
        .replace("=", "\\=")
        .replace("{", "\\{")
        .replace("}", "\\}")
        .replace(".", "\\.")
        .replace("!", "\\!")
}