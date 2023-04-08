// v0.1.0 Refactor into multiple files
use GPTelegram::handle_message;
use pretty_env_logger;
use teloxide::prelude::*;
use teloxide::types::ChatAction;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting chatgpt-tgbot-rust...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| {
        async move {
            if msg.text().is_some() {
                handle_message(bot, msg).await;
            }
            ResponseResult::<()>::Ok(())
        }
    })
    .await;
}
