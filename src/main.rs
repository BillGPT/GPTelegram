// v0.1.0 Refactor into multiple files
mod gpt;
mod telegram;

use teloxide::prelude::*;
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting chatgpt-tgbot-rust...");

    let bot = Bot::from_env();

    telegram::init_telegram_bot(bot).await;
}
