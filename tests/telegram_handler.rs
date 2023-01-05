use std::env;

#[test]
fn oai_token_exist(){
    match Some(env::var("OPENAI_TOKEN").unwrap()) {
        Some(_) => {
            assert!(true);
        },
        None => {
            assert!(false);
        }
    }
}

#[test]
fn telegram_token_exist(){
    match Some(env::var("TELOXIDE_TOKEN").unwrap()) {
        Some(_) => {
            assert!(true);
        },
        None => {
            assert!(false);
        }
    }
}