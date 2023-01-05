# ChatBot GPT3
This bot implements a callback from OpenAI's GPT3 API. It is a simple bot that can be used to test the API. It is not meant to be used in production.

## Installation
You need to pass the tokens in the environment variables. You can do this by entering the following command:
```
export TELOXIDE_TOKEN={YOUR_TOKEN}
export OPENAI_TOKEN={YOUR_OPEN_AI_TOKEN}
```
Teloxide Token is the telegram bot token that you receive from bot father after the  creation.

Then running: `cargo test` to see if all enviroment variables are set.

After setting the environment variables, you can run the bot by running `cargo run`.

## Bot Biases
You can change the bot bias by changing the `preamble` variable at the `lib.rs` file. The preamble is the bias that the bot will use to generate the response. The default preamble use a portuguese biased response text.